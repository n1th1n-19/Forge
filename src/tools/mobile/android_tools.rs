use crate::tools::{version_of, Category, Tool};
use crate::package_manager::PackageManager;
use anyhow::{bail, Result};

pub struct AndroidPlatformTools;

impl Tool for AndroidPlatformTools {
    fn name(&self) -> &str { "Android Platform Tools" }
    fn description(&self) -> &str { "adb + fastboot (developer.android.com)" }
    fn category(&self) -> Category { Category::Mobile }
    fn is_installed(&self) -> bool { which::which("adb").is_ok() }
    fn version(&self) -> Option<String> { version_of("adb", &["version"]) }
    fn install(&self, pm: &PackageManager) -> Result<()> {
        match pm {
            PackageManager::Brew => pm.install(&["android-platform-tools"]),
            PackageManager::Apt  => pm.install(&["android-tools-adb", "android-tools-fastboot"]),
            PackageManager::Dnf | PackageManager::Yum => pm.install(&["android-tools"]),
            PackageManager::Pacman => pm.install(&["android-tools"]),
            _ => bail!("Install Android Platform Tools manually from https://developer.android.com/studio/releases/platform-tools"),
        }
    }
    fn uninstall(&self, pm: &PackageManager) -> Result<()> {
        match pm {
            PackageManager::Brew   => pm.remove(&["android-platform-tools"]),
            PackageManager::Apt    => pm.remove(&["android-tools-adb", "android-tools-fastboot"]),
            _ => pm.remove(&["android-tools"]),
        }
    }
}

