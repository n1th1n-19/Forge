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
    fn install(&self, pm: &PackageManager) -> Result<()> { pm.install(&pkgs(pm)?) }
    fn uninstall(&self, pm: &PackageManager) -> Result<()> { pm.remove(&pkgs(pm)?) }
}

fn pkgs(pm: &PackageManager) -> Result<Vec<&'static str>> {
    Ok(match pm {
        PackageManager::Brew => vec!["android-platform-tools"],
        // the android-tools-* names are gone from current Debian/Ubuntu
        PackageManager::Apt => vec!["adb", "fastboot"],
        PackageManager::Dnf | PackageManager::Yum | PackageManager::Pacman | PackageManager::Zypper => {
            vec!["android-tools"]
        }
        PackageManager::Unknown => bail!(
            "Install Android Platform Tools manually from https://developer.android.com/tools/releases/platform-tools"
        ),
    })
}
