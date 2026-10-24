use crate::tools::{version_of, Category, Tool};
use crate::package_manager::PackageManager;
use anyhow::{bail, Result};

pub struct CocoaPods;

impl Tool for CocoaPods {
    fn name(&self) -> &str { "CocoaPods" }
    fn description(&self) -> &str { "iOS dependency manager (macOS only)" }
    fn category(&self) -> Category { Category::Mobile }
    fn is_installed(&self) -> bool { which::which("pod").is_ok() }
    fn version(&self) -> Option<String> { version_of("pod", &["--version"]) }

    fn install(&self, pm: &PackageManager) -> Result<()> {
        match pm {
            PackageManager::Brew => pm.install(&["cocoapods"]),
            _ => bail!("CocoaPods is macOS only"),
        }
    }

    fn uninstall(&self, pm: &PackageManager) -> Result<()> {
        match pm {
            PackageManager::Brew => pm.remove(&["cocoapods"]),
            _ => bail!("CocoaPods is macOS only"),
        }
    }
}
