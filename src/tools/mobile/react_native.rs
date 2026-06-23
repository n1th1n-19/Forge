use crate::tools::{version_of, Category, Tool};
use crate::package_manager::PackageManager;
use anyhow::{bail, Result};
use std::process::Command;

pub struct ReactNative;

impl Tool for ReactNative {
    fn name(&self) -> &str { "React Native + Expo" }
    fn description(&self) -> &str { "react-native-cli + expo-cli (requires Node)" }
    fn category(&self) -> Category { Category::Mobile }
    fn is_installed(&self) -> bool {
        which::which("react-native").is_ok() || which::which("expo").is_ok()
    }
    fn version(&self) -> Option<String> { version_of("expo", &["--version"]) }

    fn install(&self, _pm: &PackageManager) -> Result<()> {
        if !which::which("npm").is_ok() {
            bail!("Node.js / npm required — install Node first");
        }
        run_sh("npm install -g @react-native-community/cli expo-cli")
    }

    fn uninstall(&self, _pm: &PackageManager) -> Result<()> {
        run_sh("npm uninstall -g @react-native-community/cli expo-cli")
    }
}

fn run_sh(cmd: &str) -> Result<()> {
    let status = Command::new("sh").arg("-c").arg(cmd).status()?;
    if !status.success() { bail!("command failed"); }
    Ok(())
}
