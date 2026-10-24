use super::{home, pins, run_remote, sh, version_of, Category, Tool};
use crate::package_manager::PackageManager;
use anyhow::{bail, Result};

pub struct Zsh;
impl Tool for Zsh {
    fn name(&self) -> &str { "zsh" }
    fn description(&self) -> &str { "Z shell" }
    fn category(&self) -> Category { Category::Shell }
    fn is_installed(&self) -> bool { which::which("zsh").is_ok() }
    fn version(&self) -> Option<String> { version_of("zsh", &["--version"]) }
    fn install(&self, pm: &PackageManager) -> Result<()> { pm.install(&["zsh"]) }
    fn uninstall(&self, pm: &PackageManager) -> Result<()> {
        // removing your login shell locks you out of new terminals
        if std::env::var("SHELL").is_ok_and(|s| s.ends_with("/zsh")) {
            bail!("zsh is your login shell — run `chsh -s /bin/bash` first");
        }
        pm.remove(&["zsh"])
    }
}

pub struct OhMyZsh;
impl Tool for OhMyZsh {
    fn name(&self) -> &str { "Oh My Zsh" }
    fn description(&self) -> &str { "zsh plugin framework (ohmyz.sh)" }
    fn category(&self) -> Category { Category::Shell }
    fn is_installed(&self) -> bool { home().join(".oh-my-zsh").exists() }
    fn version(&self) -> Option<String> {
        let dir = home().join(".oh-my-zsh");
        if !dir.exists() {
            return None;
        }
        let hash = std::process::Command::new("git")
            .arg("-C")
            .arg(&dir)
            .args(["rev-parse", "--short", "HEAD"])
            .output()
            .ok()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
            .filter(|h| !h.is_empty());
        Some(hash.map_or_else(|| String::from("installed"), |h| format!("git@{h}")))
    }

    fn install(&self, _pm: &PackageManager) -> Result<()> {
        if which::which("zsh").is_err() {
            bail!("zsh required — install zsh first");
        }
        run_remote(pins::OH_MY_ZSH.url, pins::OH_MY_ZSH.sha256, "sh", "--unattended")
    }

    fn uninstall(&self, _pm: &PackageManager) -> Result<()> {
        // restore the .zshrc that the installer backed up
        sh(r#"rm -rf "$HOME/.oh-my-zsh"
if [ -f "$HOME/.zshrc.pre-oh-my-zsh" ]; then mv "$HOME/.zshrc.pre-oh-my-zsh" "$HOME/.zshrc"; fi"#)
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
            _ => run_remote(pins::STARSHIP.url, pins::STARSHIP.sha256, "sh", "-y"),
        }
    }

    fn uninstall(&self, pm: &PackageManager) -> Result<()> {
        match pm {
            PackageManager::Brew => pm.remove(&["starship"]),
            _ => sh("sudo rm -f /usr/local/bin/starship"),
        }
    }
}
