use super::{version_of, Category, Tool};
use crate::package_manager::PackageManager;
use anyhow::{bail, Result};
use std::process::Command;

pub struct Zsh;
impl Tool for Zsh {
    fn name(&self) -> &str { "zsh" }
    fn description(&self) -> &str { "Z shell" }
    fn category(&self) -> Category { Category::Shell }
    fn is_installed(&self) -> bool { which::which("zsh").is_ok() }
    fn version(&self) -> Option<String> { version_of("zsh", &["--version"]) }
    fn install(&self, pm: &PackageManager) -> Result<()> { pm.install(&["zsh"]) }
    fn uninstall(&self, pm: &PackageManager) -> Result<()> { pm.remove(&["zsh"]) }
}

pub struct OhMyZsh;
impl Tool for OhMyZsh {
    fn name(&self) -> &str { "Oh My Zsh" }
    fn description(&self) -> &str { "zsh plugin framework (ohmyz.sh)" }
    fn category(&self) -> Category { Category::Shell }
    fn is_installed(&self) -> bool {
        std::path::Path::new(&format!("{}/.oh-my-zsh", std::env::var("HOME").unwrap_or_default()))
            .exists()
    }
    fn version(&self) -> Option<String> {
        let home = std::env::var("HOME").unwrap_or_default();
        let path = format!("{}/.oh-my-zsh/THIS_IS_OH_MY_ZSH", home);
        if std::path::Path::new(&path).exists() {
            Some(String::from("installed"))
        } else {
            None
        }
    }

    fn install(&self, _pm: &PackageManager) -> Result<()> {
        run_sh(r#"sh -c "$(curl -fsSL https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh)" "" --unattended"#)
    }

    fn uninstall(&self, _pm: &PackageManager) -> Result<()> {
        let home = std::env::var("HOME").unwrap_or_default();
        run_sh(&format!("rm -rf {}/.oh-my-zsh", home))
    }
}

pub struct Starship;
impl Tool for Starship {
    fn name(&self) -> &str { "Starship" }
    fn description(&self) -> &str { "cross-shell prompt (starship.rs)" }
    fn category(&self) -> Category { Category::Shell }
    fn is_installed(&self) -> bool { which::which("starship").is_ok() }
    fn version(&self) -> Option<String> { version_of("starship", &["--version"]) }

    fn install(&self, pm: &PackageManager) -> Result<()> {
        match pm {
            PackageManager::Brew => pm.install(&["starship"]),
            _ => run_sh("curl -sS https://starship.rs/install.sh | sh -s -- -y"),
        }
    }

    fn uninstall(&self, pm: &PackageManager) -> Result<()> {
        match pm {
            PackageManager::Brew => pm.remove(&["starship"]),
            _ => run_sh("sh -c 'rm -f $(which starship)'"),
        }
    }
}

fn run_sh(cmd: &str) -> Result<()> {
    let status = Command::new("sh").arg("-c").arg(cmd).status()?;
    if !status.success() {
        bail!("command failed");
    }
    Ok(())
}
