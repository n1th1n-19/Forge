use super::{sh, version_of, Category, Tool};
use crate::package_manager::PackageManager;
use anyhow::{bail, Result};

pub struct GithubCLI;

impl Tool for GithubCLI {
    fn name(&self) -> &str { "gh" }
    fn description(&self) -> &str { "GitHub CLI (cli.github.com)" }
    fn category(&self) -> Category { Category::VersionControl }
    fn is_installed(&self) -> bool { which::which("gh").is_ok() }
    fn version(&self) -> Option<String> { version_of("gh", &["--version"]) }

    fn install(&self, pm: &PackageManager) -> Result<()> {
        match pm {
            PackageManager::Brew | PackageManager::Zypper => pm.install(&["gh"]),
            PackageManager::Apt => {
                sh("sudo mkdir -p -m 755 /etc/apt/keyrings && \
                    curl -fsSL https://cli.github.com/packages/githubcli-archive-keyring.gpg \
                    | sudo tee /etc/apt/keyrings/githubcli-archive-keyring.gpg > /dev/null && \
                    sudo chmod go+r /etc/apt/keyrings/githubcli-archive-keyring.gpg && \
                    echo \"deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/githubcli-archive-keyring.gpg] https://cli.github.com/packages stable main\" \
                    | sudo tee /etc/apt/sources.list.d/github-cli.list > /dev/null")?;
                pm.update()?;
                pm.install(&["gh"])
            }
            PackageManager::Dnf | PackageManager::Yum => {
                // works on yum, dnf4 and dnf5 alike
                sh("sudo curl -fsSL https://cli.github.com/packages/rpm/gh-cli.repo -o /etc/yum.repos.d/gh-cli.repo")?;
                pm.install(&["gh"])
            }
            PackageManager::Pacman => pm.install(&["github-cli"]),
            PackageManager::Unknown => bail!("Install gh manually from https://cli.github.com"),
        }
    }

    fn uninstall(&self, pm: &PackageManager) -> Result<()> {
        match pm {
            PackageManager::Pacman => pm.remove(&["github-cli"]),
            _ => pm.remove(&["gh"]),
        }
    }
}
