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
    // most distros' own tooling runs on the system python3
    fn protected(&self) -> bool { true }

    fn install(&self, pm: &PackageManager) -> Result<()> { pm.install(&pkgs(pm)) }
    fn uninstall(&self, pm: &PackageManager) -> Result<()> { pm.remove(&pkgs(pm)) }
}

fn pkgs(pm: &PackageManager) -> Vec<&'static str> {
    match pm {
        PackageManager::Apt => vec!["python3", "python3-pip", "python3-venv"],
        PackageManager::Pacman => vec!["python", "python-pip"],
        PackageManager::Brew => vec!["python"],
        _ => vec!["python3", "python3-pip"],
    }
}
