use super::{version_of, Category, Tool};
use crate::package_manager::PackageManager;
use anyhow::Result;

pub struct PHP;

impl Tool for PHP {
    fn name(&self) -> &str { "PHP" }
    fn description(&self) -> &str { "PHP runtime + composer" }
    fn category(&self) -> Category { Category::Languages }
    fn is_installed(&self) -> bool { which::which("php").is_ok() && which::which("composer").is_ok() }
    fn version(&self) -> Option<String> { version_of("php", &["--version"]) }

    fn install(&self, pm: &PackageManager) -> Result<()> { pm.install(&pkgs(pm)) }
    fn uninstall(&self, pm: &PackageManager) -> Result<()> { pm.remove(&pkgs(pm)) }
}

fn pkgs(pm: &PackageManager) -> Vec<&'static str> {
    match pm {
        PackageManager::Apt => vec!["php-cli", "php-mbstring", "php-xml", "php-curl", "composer"],
        PackageManager::Dnf | PackageManager::Yum => vec!["php-cli", "php-mbstring", "php-xml", "composer"],
        PackageManager::Zypper => vec!["php8", "php-composer2"],
        _ => vec!["php", "composer"],
    }
}
