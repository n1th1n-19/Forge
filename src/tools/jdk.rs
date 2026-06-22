use super::{version_of, Category, Tool};
use crate::package_manager::PackageManager;
use anyhow::Result;

pub struct JDK;

impl Tool for JDK {
    fn name(&self) -> &str { "OpenJDK 21" }
    fn description(&self) -> &str { "Java Development Kit (LTS)" }
    fn category(&self) -> Category { Category::Languages }
    fn is_installed(&self) -> bool { which::which("java").is_ok() }
    fn version(&self) -> Option<String> { version_of("java", &["-version"]) }

    fn install(&self, pm: &PackageManager) -> Result<()> {
        match pm {
            PackageManager::Apt => pm.install(&["openjdk-21-jdk"]),
            PackageManager::Dnf | PackageManager::Yum => {
                pm.install(&["java-21-openjdk-devel"])
            }
            PackageManager::Pacman => pm.install(&["jdk21-openjdk"]),
            PackageManager::Zypper => pm.install(&["java-21-openjdk-devel"]),
            PackageManager::Brew => pm.install(&["openjdk@21"]),
            _ => pm.install(&["openjdk-21-jdk"]),
        }
    }

    fn uninstall(&self, pm: &PackageManager) -> Result<()> {
        match pm {
            PackageManager::Apt => pm.remove(&["openjdk-21-jdk"]),
            PackageManager::Dnf | PackageManager::Yum => {
                pm.remove(&["java-21-openjdk-devel"])
            }
            PackageManager::Pacman => pm.remove(&["jdk21-openjdk"]),
            PackageManager::Brew => pm.remove(&["openjdk@21"]),
            _ => pm.remove(&["openjdk-21-jdk"]),
        }
    }
}
