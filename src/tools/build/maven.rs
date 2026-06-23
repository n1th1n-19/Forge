use crate::tools::{version_of, Category, Tool};
use crate::package_manager::PackageManager;
use anyhow::Result;

pub struct Maven;

impl Tool for Maven {
    fn name(&self) -> &str { "Maven" }
    fn description(&self) -> &str { "Java build tool (maven.apache.org)" }
    fn category(&self) -> Category { Category::Build }
    fn is_installed(&self) -> bool { which::which("mvn").is_ok() }
    fn version(&self) -> Option<String> { version_of("mvn", &["--version"]) }

    fn install(&self, pm: &PackageManager) -> Result<()> {
        match pm {
            PackageManager::Apt    => pm.install(&["maven"]),
            PackageManager::Dnf | PackageManager::Yum => pm.install(&["maven"]),
            PackageManager::Pacman => pm.install(&["maven"]),
            PackageManager::Zypper => pm.install(&["maven"]),
            PackageManager::Brew   => pm.install(&["maven"]),
            _ => pm.install(&["maven"]),
        }
    }

    fn uninstall(&self, pm: &PackageManager) -> Result<()> {
        pm.remove(&["maven"])
    }
}
