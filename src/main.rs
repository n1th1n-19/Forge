// Acronym-style names (RHEL, WSL, JDK, PHP) read better than Rhel/Wsl here.
#![allow(clippy::upper_case_acronyms, clippy::enum_variant_names)]

mod cmd;
mod detect;
mod package_manager;
mod tools;
mod ui;

use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand};
use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Confirm, MultiSelect};
use package_manager::PackageManager;
use tools::{all_tools, Category};
use ui::{InstallResult, print_header, print_summary, spinner};

#[derive(Parser)]
#[command(name = "forge", about = "Interactive developer environment setup", version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Cmd>,

    /// Install all tools without prompting
    #[arg(long)]
    all: bool,

    /// Comma-separated tools to skip (e.g. --skip go,docker)
    #[arg(long, value_delimiter = ',')]
    skip: Vec<String>,
}

#[derive(Subcommand)]
enum Cmd {
    /// List all tools and their install status
    List,
    /// Show installed vs missing with versions
    Check,
    /// Uninstall specific tools
    Uninstall {
        /// Tool names to uninstall (e.g. forge uninstall go docker)
        #[arg(required = false)]
        tools: Vec<String>,
        /// Uninstall all managed tools
        #[arg(long)]
        all: bool,
    },
    /// Remove the forge binary itself
    SelfUninstall,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let os = detect::detect();
    let pm = PackageManager::detect(&os);

    let result = match cli.command {
        Some(Cmd::List) => cmd_list(),
        Some(Cmd::Check) => cmd_check(),
        Some(Cmd::Uninstall { tools, all }) => cmd_uninstall(&pm, tools, all),
        Some(Cmd::SelfUninstall) => cmd_self_uninstall(),
        None => cmd_install(&cli, &os, &pm),
    };
    cmd::sudo_reset();
    result
}

/// `sudo forge` installs into root's home and adds root (not you) to groups.
fn warn_if_sudo_root() {
    if let Ok(user) = std::env::var("SUDO_USER")
        && cmd::output(&["id", "-u"]).is_ok_and(|u| u.trim() == "0")
    {
        ui::print_warn(
            "running as root",
            &format!("user-level tools will go into root's home, not {user}'s — run forge without sudo"),
        );
    }
}

fn cmd_install(cli: &Cli, os: &detect::OS, pm: &PackageManager) -> Result<()> {
    print_header(&os.to_string(), &pm.to_string());

    if *pm == PackageManager::Unknown {
        bail!("unsupported platform ({os}): no supported package manager found");
    }
    warn_if_sudo_root();

    let tools = all_tools();
    let installed: Vec<bool> = tools.iter().map(|t| t.is_installed()).collect();
    let skip_set: std::collections::HashSet<String> =
        cli.skip.iter().map(|s| s.to_lowercase()).collect();

    let selected_indices: Vec<usize> = if cli.all {
        (0..tools.len()).collect()
    } else {
        let mut items: Vec<String> = Vec::new();
        for (tool, &inst) in tools.iter().zip(&installed) {
            let installed_marker = if inst { "●" } else { "○" };
            items.push(format!(
                "{} {:<22} {}",
                installed_marker,
                tool.name(),
                tool.description()
            ));
        }

        let defaults: Vec<bool> = tools
            .iter()
            .zip(&installed)
            .map(|(t, &inst)| !inst && !skip_set.contains(&t.name().to_lowercase()))
            .collect();

        println!(
            "  {} already installed  {} not installed\n",
            "●".green(),
            "○".dimmed()
        );

        MultiSelect::with_theme(&ColorfulTheme::default())
            .with_prompt("Select tools to install (Space to toggle, Enter to confirm)")
            .items(&items)
            .defaults(&defaults)
            .interact()?
    };

    if selected_indices.is_empty() {
        println!("\n  {}", "No tools selected.".dimmed());
        return Ok(());
    }

    let to_install: Vec<usize> = selected_indices
        .into_iter()
        .filter(|i| !skip_set.contains(&tools[*i].name().to_lowercase()))
        .collect();

    println!(
        "\n  Installing {} tool(s)...\n",
        to_install.len().to_string().bold()
    );

    cmd::sudo_keepalive()?;

    if confirm_update(cli, pm)? {
        let pb = spinner(&format!("Updating package index ({pm})..."));
        let update = pm.update();
        pb.finish_and_clear();
        match update {
            Ok(()) => ui::print_done("package index", "updated"),
            Err(e) => ui::print_warn("package index", &format!("update failed, continuing: {e}")),
        }
    } else {
        ui::print_skipped("package index", "not refreshed — run `sudo pacman -Syu` if installs fail");
    }

    let mut results: Vec<(String, InstallResult)> = Vec::new();

    for i in to_install {
        let tool = &tools[i];

        if installed[i] {
            let v = tool.version().unwrap_or_else(|| String::from("unknown"));
            ui::print_skipped(tool.name(), &format!("already installed ({})", v));
            results.push((
                tool.name().to_string(),
                InstallResult::AlreadyInstalled(v),
            ));
            continue;
        }

        let pb = spinner(&format!("Installing {}...", tool.name()));
        match tool.install(pm) {
            Ok(()) => {
                pb.finish_and_clear();
                let v = tool.version().unwrap_or_else(|| String::from("done"));
                ui::print_done(tool.name(), &v);
                if let Some(note) = tool.post_install_note() {
                    ui::print_warn(tool.name(), note);
                }
                results.push((tool.name().to_string(), InstallResult::Done(v)));
            }
            Err(e) => {
                pb.finish_and_clear();
                let msg = e.to_string();
                ui::print_error(tool.name(), &msg);
                results.push((tool.name().to_string(), InstallResult::Failed(msg)));
            }
        }
    }

    print_summary(&results);
    Ok(())
}

/// Refreshing on Arch means a full system upgrade, so ask first.
/// `--all` promises no prompts, so it skips the upgrade instead.
fn confirm_update(cli: &Cli, pm: &PackageManager) -> Result<bool> {
    if *pm != PackageManager::Pacman {
        return Ok(true);
    }
    if cli.all {
        return Ok(false);
    }
    Ok(Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Arch can only refresh package lists with a full upgrade. Run `pacman -Syu` now?")
        .default(false)
        .interact()?)
}

fn cmd_list() -> Result<()> {
    let tools = all_tools();
    let mut last_cat: Option<Category> = None;

    for tool in &tools {
        if last_cat.as_ref() != Some(&tool.category()) {
            println!(
                "\n  {}",
                tool.category().to_string().truecolor(255, 107, 53).bold()
            );
            last_cat = Some(tool.category());
        }
        let (marker, status) = if tool.is_installed() {
            (
                "●".green().to_string(),
                tool.version().unwrap_or_else(|| String::from("installed")),
            )
        } else {
            ("○".dimmed().to_string(), String::from("not installed"))
        };
        println!(
            "  {} {:<22} {}  {}",
            marker,
            tool.name(),
            tool.description().dimmed(),
            status.dimmed()
        );
    }
    println!();
    Ok(())
}

fn cmd_check() -> Result<()> {
    let tools = all_tools();
    let installed: Vec<_> = tools.iter().filter(|t| t.is_installed()).collect();
    let missing: Vec<_> = tools.iter().filter(|t| !t.is_installed()).collect();

    println!(
        "\n  {} Installed ({}):",
        "✓".green().bold(),
        installed.len()
    );
    for t in &installed {
        println!(
            "    {:<22} {}",
            t.name(),
            t.version()
                .unwrap_or_else(|| String::from("unknown"))
                .dimmed()
        );
    }

    println!("\n  {} Missing ({}):", "○".dimmed(), missing.len());
    for t in &missing {
        println!("    {}", t.name().dimmed());
    }
    println!();
    Ok(())
}

fn cmd_uninstall(pm: &PackageManager, tool_names: Vec<String>, all: bool) -> Result<()> {
    if *pm == PackageManager::Unknown {
        bail!("unsupported platform: no supported package manager found");
    }
    let tools = all_tools();

    if all {
        let names: Vec<&str> = tools
            .iter()
            .filter(|t| t.is_installed() && !t.protected())
            .map(|t| t.name())
            .collect();
        println!(
            "\n  {} This removes {} tool(s): {}\n",
            "!".yellow().bold(),
            names.len(),
            names.join(", ").bold()
        );
        let sure = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Uninstall all of them?")
            .default(false)
            .interact()?;
        if !sure {
            println!("  {}", "Cancelled.".dimmed());
            return Ok(());
        }
    }

    let target_indices: Vec<usize> = if all {
        // base-system tools (git, curl, python3, …) only go when named explicitly
        (0..tools.len()).filter(|&i| !tools[i].protected()).collect()
    } else if tool_names.is_empty() {
        let installed_indices: Vec<usize> = tools
            .iter()
            .enumerate()
            .filter(|(_, t)| t.is_installed())
            .map(|(i, _)| i)
            .collect();

        if installed_indices.is_empty() {
            println!("  {}", "No managed tools are installed.".dimmed());
            return Ok(());
        }

        let items: Vec<String> = installed_indices
            .iter()
            .map(|&i| {
                format!(
                    "{:<22} {}",
                    tools[i].name(),
                    tools[i].version().unwrap_or_default().dimmed()
                )
            })
            .collect();

        let selections = MultiSelect::with_theme(&ColorfulTheme::default())
            .with_prompt("Select tools to uninstall")
            .items(&items)
            .interact()?;

        selections.into_iter().map(|s| installed_indices[s]).collect()
    } else {
        let lower_names: Vec<String> =
            tool_names.iter().map(|s| s.to_lowercase()).collect();
        tools
            .iter()
            .enumerate()
            .filter(|(_, t)| lower_names.contains(&t.name().to_lowercase()))
            .map(|(i, _)| i)
            .collect()
    };

    if target_indices.is_empty() {
        println!("  {}", "No matching tools found.".dimmed());
        return Ok(());
    }

    cmd::sudo_keepalive()?;
    println!();
    let mut results: Vec<(String, InstallResult)> = Vec::new();

    for i in target_indices {
        let tool = &tools[i];
        if !tool.is_installed() {
            ui::print_skipped(tool.name(), "not installed");
            results.push((tool.name().to_string(), InstallResult::Skipped));
            continue;
        }
        let pb = spinner(&format!("Removing {}...", tool.name()));
        match tool.uninstall(pm) {
            Ok(()) => {
                pb.finish_and_clear();
                ui::print_done(tool.name(), "removed");
                results.push((
                    tool.name().to_string(),
                    InstallResult::Done(String::from("removed")),
                ));
            }
            Err(e) => {
                pb.finish_and_clear();
                let msg = e.to_string();
                ui::print_error(tool.name(), &msg);
                results.push((tool.name().to_string(), InstallResult::Failed(msg)));
            }
        }
    }

    print_summary(&results);
    Ok(())
}

fn cmd_self_uninstall() -> Result<()> {
    let exe = std::env::current_exe()?;
    // current_exe resolves symlinks — don't delete a build in a source tree
    if exe.components().any(|c| c.as_os_str() == "target") {
        bail!("{} looks like a cargo build output, not an installed forge", exe.display());
    }
    let exe_str = exe.to_str().context("forge path is not valid UTF-8")?;
    let sure = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(format!("Remove forge from {}?", exe.display()))
        .default(false)
        .interact()?;
    if !sure {
        println!("  {}", "Cancelled.".dimmed());
        return Ok(());
    }
    if let Err(e) = std::fs::remove_file(&exe) {
        if e.kind() != std::io::ErrorKind::PermissionDenied {
            return Err(e.into());
        }
        // /usr/local/bin is usually root-owned (install.sh used sudo)
        cmd::sudo_keepalive()?;
        cmd::run(&["sudo", "rm", "-f", exe_str])?;
    }
    println!("  {} forge removed.", "✓".green().bold());
    Ok(())
}
