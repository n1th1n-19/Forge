use super::{version_of, Category, Tool};
use crate::package_manager::PackageManager;
use anyhow::{bail, Result};
use std::process::Command;

pub struct Bun;

impl Tool for Bun {
    fn name(&self) -> &str { "Bun" }
    fn description(&self) -> &str { "fast JS runtime + package manager (bun.sh)" }
    fn category(&self) -> Category { Category::Languages }
    fn is_installed(&self) -> bool { which::which("bun").is_ok() }
    fn version(&self) -> Option<String> { version_of("bun", &["--version"]) }

    fn install(&self, pm: &PackageManager) -> Result<()> {
        match pm {
            PackageManager::Brew => pm.install(&["bun"]),
            _ => run_sh("curl -fsSL https://bun.sh/install | bash"),
        }
    }

    fn uninstall(&self, pm: &PackageManager) -> Result<()> {
        match pm {
            PackageManager::Brew => pm.remove(&["bun"]),
            _ => run_sh("rm -rf \"$HOME/.bun\" && sed -i '/bun/d' \"$HOME/.bashrc\" \"$HOME/.zshrc\" 2>/dev/null || true"),
        }
    }
}

fn run_sh(cmd: &str) -> Result<()> {
    let status = Command::new("bash").arg("-c").arg(cmd).status()?;
    if !status.success() {
        bail!("command failed");
    }
    Ok(())
}
