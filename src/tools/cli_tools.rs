use super::{version_of, Category, Tool};
use crate::package_manager::PackageManager;
use anyhow::{bail, Result};
use std::process::Command;

pub struct FzfBundle;
impl Tool for FzfBundle {
    fn name(&self) -> &str { "fzf + ripgrep + bat + eza" }
    fn description(&self) -> &str { "fuzzy finder, fast grep, better cat, better ls" }
    fn category(&self) -> Category { Category::CliTools }
    fn is_installed(&self) -> bool {
        which::which("fzf").is_ok()
            && which::which("rg").is_ok()
            && which::which("bat").is_ok()
    }
    fn version(&self) -> Option<String> { version_of("fzf", &["--version"]) }

    fn install(&self, pm: &PackageManager) -> Result<()> {
        match pm {
            PackageManager::Apt => {
                pm.install(&["fzf", "ripgrep", "bat"])?;
                // eza not in main apt repos — install from GitHub release
                run_sh(r#"
                    ARCH=$(uname -m | sed 's/x86_64/x86_64/;s/aarch64/aarch64/')
                    EZA_URL=$(curl -fsSL https://api.github.com/repos/eza-community/eza/releases/latest \
                        | grep "browser_download_url" | grep "${ARCH}-unknown-linux-gnu.tar.gz" | head -1 | cut -d'"' -f4)
                    curl -fsSL "$EZA_URL" -o /tmp/eza.tar.gz
                    tar -xzf /tmp/eza.tar.gz -C /tmp/
                    sudo mv /tmp/eza /usr/local/bin/eza
                    rm /tmp/eza.tar.gz
                "#)
            }
            PackageManager::Dnf | PackageManager::Yum => {
                pm.install(&["fzf", "ripgrep", "bat", "eza"])
            }
            PackageManager::Pacman => pm.install(&["fzf", "ripgrep", "bat", "eza"]),
            PackageManager::Zypper => pm.install(&["fzf", "ripgrep", "bat"]),
            PackageManager::Brew => pm.install(&["fzf", "ripgrep", "bat", "eza"]),
            _ => pm.install(&["fzf", "ripgrep", "bat"]),
        }
    }

    fn uninstall(&self, pm: &PackageManager) -> Result<()> {
        match pm {
            PackageManager::Apt => {
                pm.remove(&["fzf", "ripgrep", "bat"])?;
                run_sh("sudo rm -f /usr/local/bin/eza")
            }
            PackageManager::Brew => pm.remove(&["fzf", "ripgrep", "bat", "eza"]),
            _ => pm.remove(&["fzf", "ripgrep", "bat", "eza"]),
        }
    }
}

pub struct JqTreeHtop;
impl Tool for JqTreeHtop {
    fn name(&self) -> &str { "jq + tree + htop" }
    fn description(&self) -> &str { "JSON processor, dir tree, process monitor" }
    fn category(&self) -> Category { Category::CliTools }
    fn is_installed(&self) -> bool {
        which::which("jq").is_ok() && which::which("tree").is_ok() && which::which("htop").is_ok()
    }
    fn version(&self) -> Option<String> { version_of("jq", &["--version"]) }
    fn install(&self, pm: &PackageManager) -> Result<()> {
        pm.install(&["jq", "tree", "htop"])
    }
    fn uninstall(&self, pm: &PackageManager) -> Result<()> {
        pm.remove(&["jq", "tree", "htop"])
    }
}

pub struct HTTPie;
impl Tool for HTTPie {
    fn name(&self) -> &str { "HTTPie" }
    fn description(&self) -> &str { "modern HTTP client for APIs (httpie.io)" }
    fn category(&self) -> Category { Category::CliTools }
    fn is_installed(&self) -> bool { which::which("http").is_ok() || which::which("httpie").is_ok() }
    fn version(&self) -> Option<String> { version_of("http", &["--version"]) }

    fn install(&self, pm: &PackageManager) -> Result<()> {
        match pm {
            PackageManager::Brew => pm.install(&["httpie"]),
            PackageManager::Apt => pm.install(&["httpie"]),
            PackageManager::Dnf | PackageManager::Yum => pm.install(&["httpie"]),
            PackageManager::Pacman => pm.install(&["httpie"]),
            _ => run_sh("pip3 install --upgrade httpie"),
        }
    }

    fn uninstall(&self, pm: &PackageManager) -> Result<()> {
        match pm {
            PackageManager::Brew => pm.remove(&["httpie"]),
            _ => pm.remove(&["httpie"]),
        }
    }
}

pub struct Tmux;
impl Tool for Tmux {
    fn name(&self) -> &str { "tmux" }
    fn description(&self) -> &str { "terminal multiplexer" }
    fn category(&self) -> Category { Category::CliTools }
    fn is_installed(&self) -> bool { which::which("tmux").is_ok() }
    fn version(&self) -> Option<String> { version_of("tmux", &["-V"]) }
    fn install(&self, pm: &PackageManager) -> Result<()> { pm.install(&["tmux"]) }
    fn uninstall(&self, pm: &PackageManager) -> Result<()> { pm.remove(&["tmux"]) }
}

fn run_sh(cmd: &str) -> Result<()> {
    let status = Command::new("sh").arg("-c").arg(cmd).status()?;
    if !status.success() {
        bail!("command failed");
    }
    Ok(())
}
