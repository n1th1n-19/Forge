use super::{version_of, Category, Tool};
use crate::package_manager::PackageManager;
use anyhow::{bail, Result};
use std::process::Command;

pub struct RustLang;

impl Tool for RustLang {
    fn name(&self) -> &str { "Rust" }
    fn description(&self) -> &str { "rustup.rs — toolchain manager" }
    fn category(&self) -> Category { Category::Languages }
    fn is_installed(&self) -> bool { which::which("rustc").is_ok() }
    fn version(&self) -> Option<String> { version_of("rustc", &["--version"]) }

    fn install(&self, _pm: &PackageManager) -> Result<()> {
        run_sh("curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y")
    }

    fn uninstall(&self, _pm: &PackageManager) -> Result<()> {
        run_sh("rustup self uninstall -y")
    }
}

fn run_sh(cmd: &str) -> Result<()> {
    let status = Command::new("sh").arg("-c").arg(cmd).status()?;
    if !status.success() {
        bail!("command failed");
    }
    Ok(())
}
