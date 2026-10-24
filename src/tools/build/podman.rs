use crate::tools::{version_of, Category, Tool};
use crate::package_manager::PackageManager;
use anyhow::Result;

pub struct Podman;

impl Tool for Podman {
    fn name(&self) -> &str { "Podman" }
    fn description(&self) -> &str { "rootless container runtime (podman.io)" }
    fn category(&self) -> Category { Category::Build }
    fn is_installed(&self) -> bool { which::which("podman").is_ok() }
    fn version(&self) -> Option<String> { version_of("podman", &["--version"]) }
    fn install(&self, pm: &PackageManager) -> Result<()> { pm.install(&["podman"]) }
    fn uninstall(&self, pm: &PackageManager) -> Result<()> { pm.remove(&["podman"]) }
}
