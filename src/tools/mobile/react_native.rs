use crate::tools::{sh, version_of, Category, Tool};
use crate::package_manager::PackageManager;
use anyhow::{bail, Result};

pub struct ReactNative;

// expo-cli is deprecated: Expo now runs via `npx expo`, builds via eas-cli
const PKGS: &str = "@react-native-community/cli eas-cli";

impl Tool for ReactNative {
    fn name(&self) -> &str { "React Native + Expo" }
    fn description(&self) -> &str { "react-native CLI + eas-cli (requires Node)" }
    fn category(&self) -> Category { Category::Mobile }
    fn is_installed(&self) -> bool { which::which("react-native").is_ok() && which::which("eas").is_ok() }
    fn version(&self) -> Option<String> { version_of("eas", &["--version"]) }

    fn install(&self, _pm: &PackageManager) -> Result<()> {
        if which::which("npm").is_err() {
            bail!("Node.js / npm required — install Node first");
        }
        sh(&format!("npm install -g {PKGS}"))
    }

    fn uninstall(&self, _pm: &PackageManager) -> Result<()> {
        sh(&format!("npm uninstall -g {PKGS} expo-cli"))
    }
}
