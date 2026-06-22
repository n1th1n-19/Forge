use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

// ASCII art from personal/forge-ascii.md — flame + anvil symbol
pub const LOGO: &str = r#"
                                          ..
                                        .:-:
                                      ..=%-..
                                     ..-@@-..
                                    .:@@@+-...
                                   ..:@++*+:....@.
                                 ::..:#****+-:.:.
                                .....:-@@*%@@@-:....@:.
                                ..::..:=++++++=-:...=::.
                               ..:@-..:-%+%@@@@@-:...-@#..
                             ..:-#%-..:::=%%%%%%%:...-@@@.
                            ..:=+==:....::=++++=#:..:=++=:.
                           .:#@%+-:::@:.-=#%%%#%#:::%%%%%:.
                          .::===-:.--:.:========-:-====+-:.
                         ..%%%%%:....:%%%-=--%%%:==#%%#%:..
                         ..-----:..::-=---::--::-----=-:....
                         .###%#%=.:#%%#%-:-#=::%#+-*%#:...:.
                         .::::::::-:--::..::.:::---:-:..::-:.
                         ..+***.:=+###.:-.-..##%#-.:%##%#+-:.
                          ..:::..:::::..::...::::...:--:-:..
                           ...=+-..-+++:.:=:..*++-...-*+=..
                        .::::::::::::::::::::::::::::::::::::::::::::::
         .****-****:***+-=:..........................................:=.
          -%:.        .:-+%%%.%%:%%*%%%%:%%.%%.%%.%%%.%%.%%%:%%%%%%%:%=.
           .:%..       .%%%:.
             ..%:..
                ..:%:....
                    ...:-%%%%=%%.
                              ..::.
                                .::
                                .::
                              ..:-.
                            .:%:.
                         ..+:.       .::#++++*:..
                        .*:.       .%:..      ..%:.
                        .::........%.           .::........::
                         ...........              ...........
"#;

// Color mapping for forge-ascii-color.md style rendering
// Used for a compact colored variant in tighter contexts
pub fn print_logo() {
    for line in LOGO.lines() {
        let colored_line: String = line
            .chars()
            .map(|c| match c {
                '@' => c.to_string().truecolor(30, 30, 46).to_string(),
                '+' => c.to_string().truecolor(255, 107, 53).bold().to_string(),
                '*' => c.to_string().truecolor(255, 200, 80).bold().to_string(),
                '#' => c.to_string().truecolor(100, 80, 60).to_string(),
                '%' => c.to_string().truecolor(220, 80, 30).bold().to_string(),
                '=' => c.to_string().truecolor(255, 140, 0).to_string(),
                '-' => c.to_string().truecolor(180, 60, 20).to_string(),
                ':' => c.to_string().truecolor(150, 50, 15).to_string(),
                '.' => c.to_string().truecolor(100, 40, 10).to_string(),
                _ => c.to_string(),
            })
            .collect();
        println!("{}", colored_line);
    }
}

pub fn print_header(os: &str, pkg: &str) {
    print_logo();
    println!();
    println!(
        "  {}",
        "forge — dev environment setup".bold().truecolor(255, 107, 53)
    );
    println!(
        "  {} {}  {}  {} {}",
        "OS:".dimmed(),
        os.white().bold(),
        "|".dimmed(),
        "pkg:".dimmed(),
        pkg.white().bold()
    );
    println!();
}

pub fn spinner(msg: &str) -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::with_template("  {spinner:.yellow} {msg}")
            .unwrap()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
    );
    pb.enable_steady_tick(Duration::from_millis(80));
    pb.set_message(msg.to_string());
    pb
}

pub fn print_done(tool: &str, version: &str) {
    println!(
        "  {} {:<20} {}",
        "✓".green().bold(),
        tool,
        version.dimmed()
    );
}

pub fn print_skipped(tool: &str, reason: &str) {
    println!(
        "  {} {:<20} {}",
        "↷".yellow(),
        tool,
        reason.dimmed()
    );
}

pub fn print_error(tool: &str, err: &str) {
    println!(
        "  {} {:<20} {}",
        "✗".red().bold(),
        tool,
        err.red().dimmed()
    );
}

pub fn print_summary(results: &[(String, InstallResult)]) {
    println!();
    println!("  {}", "─".repeat(50).dimmed());
    println!(
        "  {:<22} {:<12} {}",
        "Tool".bold(),
        "Status".bold(),
        "Version".bold()
    );
    println!("  {}", "─".repeat(50).dimmed());
    for (tool, result) in results {
        match result {
            InstallResult::Done(v) => println!(
                "  {:<22} {:<12} {}",
                tool,
                "✓ done".green(),
                v.dimmed()
            ),
            InstallResult::AlreadyInstalled(v) => println!(
                "  {:<22} {:<12} {}",
                tool,
                "↷ skip".yellow(),
                v.dimmed()
            ),
            InstallResult::Skipped => println!(
                "  {:<22} {}",
                tool,
                "— skipped".dimmed()
            ),
            InstallResult::Failed(e) => println!(
                "  {:<22} {:<12} {}",
                tool,
                "✗ failed".red(),
                e.red().dimmed()
            ),
        }
    }
    println!("  {}", "─".repeat(50).dimmed());
    println!();
}

#[derive(Debug)]
pub enum InstallResult {
    Done(String),
    AlreadyInstalled(String),
    Skipped,
    Failed(String),
}
