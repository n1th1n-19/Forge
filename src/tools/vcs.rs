use super::{version_of, Category, Tool};
use crate::package_manager::PackageManager;
use anyhow::{bail, Result};
use std::process::Command;

pub struct GithubCLI;

impl Tool for GithubCLI {
    fn name(&self) -> &str { "gh" }
    fn description(&self) -> &str { "GitHub CLI (cli.github.com)" }
    fn category(&self) -> Category { Category::VersionControl }
    fn is_installed(&self) -> bool { which::which("gh").is_ok() }
    fn version(&self) -> Option<String> { version_of("gh", &["--version"]) }

    fn install(&self, pm: &PackageManager) -> Result<()> {
        match pm {
            PackageManager::Brew => pm.install(&["gh"]),
            PackageManager::Apt => {
                run_sh("curl -fsSL https://cli.github.com/packages/githubcli-archive-keyring.gpg | sudo dd of=/usr/share/keyrings/githubcli-archive-keyring.gpg && sudo chmod go+r /usr/share/keyrings/githubcli-archive-keyring.gpg")?;
                run_sh("echo \"deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/githubcli-archive-keyring.gpg] https://cli.github.com/packages stable main\" | sudo tee /etc/apt/sources.list.d/github-cli.list > /dev/null")?;
                pm.update()?;
                pm.install(&["gh"])
            }
            PackageManager::Dnf | PackageManager::Yum => {
                run_sh("sudo dnf config-manager --add-repo https://cli.github.com/packages/rpm/gh-cli.repo")?;
                pm.install(&["gh"])
            }
            PackageManager::Pacman => pm.install(&["github-cli"]),
            _ => bail!("Install gh manually from https://cli.github.com"),
        }
    }

    fn uninstall(&self, pm: &PackageManager) -> Result<()> {
        match pm {
            PackageManager::Pacman => pm.remove(&["github-cli"]),
            _ => pm.remove(&["gh"]),
        }
    }
}

fn run_sh(cmd: &str) -> Result<()> {
    let status = Command::new("sh").arg("-c").arg(cmd).status()?;
    if !status.success() {
        bail!("command failed: {}", cmd);
    }
    Ok(())
}
