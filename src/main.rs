mod detect;
mod package_manager;
mod tools;
mod ui;

use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, MultiSelect};
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

    match cli.command {
        Some(Cmd::List) => cmd_list(),
        Some(Cmd::Check) => cmd_check(),
        Some(Cmd::Uninstall { tools, all }) => cmd_uninstall(&pm, tools, all),
        Some(Cmd::SelfUninstall) => cmd_self_uninstall(),
        None => cmd_install(&cli, &os, &pm),
    }
}

fn cmd_install(cli: &Cli, os: &detect::OS, pm: &PackageManager) -> Result<()> {
    print_header(&os.to_string(), &pm.to_string());

    let tools = all_tools();
    let skip_set: std::collections::HashSet<String> =
        cli.skip.iter().map(|s| s.to_lowercase()).collect();

    let selected_indices: Vec<usize> = if cli.all {
        (0..tools.len()).collect()
    } else {
        let mut items: Vec<String> = Vec::new();
        for tool in &tools {
            let installed_marker = if tool.is_installed() { "●" } else { "○" };
            items.push(format!(
                "{} {:<22} {}",
                installed_marker,
                tool.name(),
                tool.description()
            ));
        }

        let defaults: Vec<bool> = tools
            .iter()
            .map(|t| !t.is_installed() && !skip_set.contains(&t.name().to_lowercase()))
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

    let _ = pm.update();

    let mut results: Vec<(String, InstallResult)> = Vec::new();

    for i in to_install {
        let tool = &tools[i];

        if tool.is_installed() {
            let v = tool.version().unwrap_or_else(|| String::from("unknown"));
            ui::print_skipped(tool.name(), &format!("already installed ({})", v));
            results.push((
                tool.name().to_string(),
                InstallResult::AlreadyInstalled(v),
            ));
            continue;
        }

        let pb = spinner(&format!("{}...", tool.name()));
        match tool.install(pm) {
            Ok(()) => {
                pb.finish_and_clear();
                let v = tool.version().unwrap_or_else(|| String::from("done"));
                ui::print_done(tool.name(), &v);
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
    let tools = all_tools();

    let target_indices: Vec<usize> = if all {
        (0..tools.len()).collect()
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
    println!(
        "\n  {} Remove forge from {}?",
        "!".yellow().bold(),
        exe.display().to_string().bold()
    );
    print!("  [y/N] ");
    use std::io::{self, BufRead};
    let stdin = io::stdin();
    let line = stdin
        .lock()
        .lines()
        .next()
        .unwrap_or(Ok(String::new()))?;
    if line.trim().to_lowercase() == "y" {
        std::fs::remove_file(&exe)?;
        println!("  {} forge removed.", "✓".green().bold());
    } else {
        println!("  {}", "Cancelled.".dimmed());
    }
    Ok(())
}
