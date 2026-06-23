use crate::tools::{version_of, Category, Tool};
use crate::package_manager::PackageManager;
use anyhow::{bail, Result};

pub struct Podman;

impl Tool for Podman {
    fn name(&self) -> &str { "Podman" }
    fn description(&self) -> &str { "rootless container runtime (podman.io)" }
    fn category(&self) -> Category { Category::Build }
    fn is_installed(&self) -> bool { which::which("podman").is_ok() }
    fn version(&self) -> Option<String> { version_of("podman", &["--version"]) }

    fn install(&self, pm: &PackageManager) -> Result<()> {
        match pm {
            PackageManager::Apt    => pm.install(&["podman"]),
            PackageManager::Dnf | PackageManager::Yum => pm.install(&["podman"]),
            PackageManager::Pacman => pm.install(&["podman"]),
            PackageManager::Zypper => pm.install(&["podman"]),
            PackageManager::Brew   => pm.install(&["podman"]),
            _ => bail!("Install Podman from https://podman.io/docs/installation"),
        }
    }

    fn uninstall(&self, pm: &PackageManager) -> Result<()> {
        pm.remove(&["podman"])
    }
}
