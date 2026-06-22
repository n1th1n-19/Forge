use super::{version_of, Category, Tool};
use crate::package_manager::PackageManager;
use anyhow::Result;

pub struct PHP;

impl Tool for PHP {
    fn name(&self) -> &str { "PHP" }
    fn description(&self) -> &str { "PHP runtime + composer" }
    fn category(&self) -> Category { Category::Languages }
    fn is_installed(&self) -> bool { which::which("php").is_ok() }
    fn version(&self) -> Option<String> { version_of("php", &["--version"]) }

    fn install(&self, pm: &PackageManager) -> Result<()> {
        match pm {
            PackageManager::Apt => pm.install(&["php", "php-cli", "php-mbstring", "php-xml", "php-curl"]),
            PackageManager::Dnf | PackageManager::Yum => pm.install(&["php", "php-cli", "php-mbstring"]),
            PackageManager::Pacman => pm.install(&["php"]),
            PackageManager::Zypper => pm.install(&["php8"]),
            PackageManager::Brew => pm.install(&["php"]),
            _ => pm.install(&["php"]),
        }
    }

    fn uninstall(&self, pm: &PackageManager) -> Result<()> {
        match pm {
            PackageManager::Apt => pm.remove(&["php", "php-cli", "php-mbstring", "php-xml", "php-curl"]),
            PackageManager::Pacman => pm.remove(&["php"]),
            PackageManager::Brew => pm.remove(&["php"]),
            _ => pm.remove(&["php"]),
        }
    }
}
