use super::{pins, run_remote, sh, version_of, Category, Tool};
use crate::package_manager::PackageManager;
use anyhow::Result;

pub struct RustLang;

impl Tool for RustLang {
    fn name(&self) -> &str { "Rust" }
    fn description(&self) -> &str { "rustup.rs — toolchain manager" }
    fn category(&self) -> Category { Category::Languages }
    fn is_installed(&self) -> bool { which::which("rustc").is_ok() }
    fn version(&self) -> Option<String> { version_of("rustc", &["--version"]) }

    fn install(&self, _pm: &PackageManager) -> Result<()> {
        run_remote(pins::RUSTUP.url, pins::RUSTUP.sha256, "sh", "-y")
    }

    fn uninstall(&self, _pm: &PackageManager) -> Result<()> {
        // rustup may not be on forge's PATH right after install
        sh(r#""${CARGO_HOME:-$HOME/.cargo}/bin/rustup" self uninstall -y"#)
    }
}
