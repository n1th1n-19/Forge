use crate::tools::{sh, succeeds, Category, Tool};
use crate::package_manager::PackageManager;
use anyhow::{bail, Result};

pub struct XcodeTools;

impl Tool for XcodeTools {
    fn name(&self) -> &str { "Xcode CLI Tools" }
    fn description(&self) -> &str { "macOS only — xcode-select" }
    fn category(&self) -> Category { Category::Mobile }
    // the xcode-select binary always exists on macOS; `-p` only succeeds once tools are installed
    fn is_installed(&self) -> bool { succeeds("xcode-select", &["-p"]) }
    fn version(&self) -> Option<String> {
        let o = std::process::Command::new("pkgutil")
            .arg("--pkg-info=com.apple.pkg.CLTools_Executables")
            .output()
            .ok()?;
        String::from_utf8_lossy(&o.stdout)
            .lines()
            .find_map(|l| l.strip_prefix("version: "))
            .map(str::to_string)
    }

    fn install(&self, pm: &PackageManager) -> Result<()> {
        match pm {
            // opens Apple's GUI installer and returns immediately
            PackageManager::Brew => sh("xcode-select --install"),
            _ => bail!("Xcode CLI Tools are macOS only"),
        }
    }

    fn uninstall(&self, pm: &PackageManager) -> Result<()> {
        match pm {
            // never `rm -rf $(xcode-select -p)` — that points inside Xcode.app when Xcode is selected
            PackageManager::Brew => sh("sudo rm -rf /Library/Developer/CommandLineTools"),
            _ => bail!("Xcode CLI Tools are macOS only"),
        }
    }
}
