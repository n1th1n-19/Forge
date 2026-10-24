use crate::tools::{has_snap, sh, snap_has, Category, Tool};
use crate::package_manager::PackageManager;
use anyhow::{bail, Result};
use std::path::Path;

pub struct AndroidStudio;

impl Tool for AndroidStudio {
    fn name(&self) -> &str { "Android Studio" }
    fn description(&self) -> &str { "Android IDE (developer.android.com)" }
    fn category(&self) -> Category { Category::Mobile }
    fn is_installed(&self) -> bool {
        which::which("android-studio").is_ok()
            || Path::new("/opt/android-studio").exists()
            || Path::new("/snap/bin/android-studio").exists()
            || Path::new("/Applications/Android Studio.app").exists()
    }
    fn version(&self) -> Option<String> {
        ["/opt/android-studio/build.txt", "/usr/local/android-studio/build.txt"]
            .iter()
            .find_map(|p| std::fs::read_to_string(p).ok())
            .map(|v| v.trim().to_string())
            .filter(|v| !v.is_empty())
            .or_else(|| self.is_installed().then(|| String::from("installed")))
    }
    fn install(&self, pm: &PackageManager) -> Result<()> {
        match pm {
            PackageManager::Brew => pm.install_cask(&["android-studio"]),
            _ if has_snap() => sh("sudo snap install android-studio --classic"),
            _ => bail!("Install Android Studio manually from https://developer.android.com/studio"),
        }
    }
    fn uninstall(&self, pm: &PackageManager) -> Result<()> {
        match pm {
            PackageManager::Brew => pm.remove(&["android-studio"]),
            _ if snap_has("android-studio") => sh("sudo snap remove android-studio"),
            _ => sh("sudo rm -rf /opt/android-studio"),
        }
    }
}
