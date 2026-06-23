use crate::tools::{version_of, Category, Tool};
use crate::package_manager::PackageManager;
use anyhow::{bail, Result};
use std::process::Command;

pub struct XcodeTools;

impl Tool for XcodeTools {
    fn name(&self) -> &str { "Xcode CLI Tools" }
    fn description(&self) -> &str { "macOS only — xcode-select" }
    fn category(&self) -> Category { Category::Mobile }
    fn is_installed(&self) -> bool { which::which("xcode-select").is_ok() }
    fn version(&self) -> Option<String> { version_of("xcode-select", &["-p"]) }

    fn install(&self, pm: &PackageManager) -> Result<()> {
        match pm {
            PackageManager::Brew => run_sh("xcode-select --install"),
            _ => bail!("Xcode CLI Tools are macOS only"),
        }
    }

    fn uninstall(&self, pm: &PackageManager) -> Result<()> {
        match pm {
            PackageManager::Brew => run_sh("sudo rm -rf $(xcode-select -p)"),
            _ => bail!("Xcode CLI Tools are macOS only"),
        }
    }
}

fn run_sh(cmd: &str) -> Result<()> {
    let status = Command::new("sh").arg("-c").arg(cmd).status()?;
    if !status.success() { bail!("command failed"); }
    Ok(())
}
