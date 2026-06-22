use super::{version_of, Category, Tool};
use crate::package_manager::PackageManager;
use anyhow::Result;

pub struct Python;

impl Tool for Python {
    fn name(&self) -> &str { "Python 3" }
    fn description(&self) -> &str { "python3 + pip" }
    fn category(&self) -> Category { Category::Languages }
    fn is_installed(&self) -> bool { which::which("python3").is_ok() }
    fn version(&self) -> Option<String> { version_of("python3", &["--version"]) }

    fn install(&self, pm: &PackageManager) -> Result<()> {
        match pm {
            PackageManager::Apt => pm.install(&["python3", "python3-pip", "python3-venv"]),
            PackageManager::Dnf | PackageManager::Yum => {
                pm.install(&["python3", "python3-pip"])
            }
            PackageManager::Pacman => pm.install(&["python", "python-pip"]),
            PackageManager::Zypper => pm.install(&["python3", "python3-pip"]),
            PackageManager::Brew => pm.install(&["python"]),
            _ => pm.install(&["python3"]),
        }
    }

    fn uninstall(&self, pm: &PackageManager) -> Result<()> {
        match pm {
            PackageManager::Apt => pm.remove(&["python3", "python3-pip", "python3-venv"]),
            PackageManager::Pacman => pm.remove(&["python", "python-pip"]),
            PackageManager::Brew => pm.remove(&["python"]),
            _ => pm.remove(&["python3", "python3-pip"]),
        }
    }
}
