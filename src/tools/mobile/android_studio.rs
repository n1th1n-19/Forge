use crate::tools::{Category, Tool};
use crate::package_manager::PackageManager;
use anyhow::{bail, Result};
use std::process::Command;

pub struct AndroidStudio;

impl Tool for AndroidStudio {
    fn name(&self) -> &str { "Android Studio" }
    fn description(&self) -> &str { "Android IDE (developer.android.com)" }
    fn category(&self) -> Category { Category::Mobile }
    fn is_installed(&self) -> bool {
        which::which("android-studio").is_ok()
            || std::path::Path::new("/opt/android-studio").exists()
            || std::path::Path::new("/snap/bin/android-studio").exists()
    }
    fn version(&self) -> Option<String> {
        let paths = [
            "/opt/android-studio/build.txt",
            "/usr/local/android-studio/build.txt",
        ];
        for p in &paths {
            if let Ok(v) = std::fs::read_to_string(p) {
                let ver = v.trim().to_string();
                if !ver.is_empty() { return Some(ver); }
            }
        }
        if self.is_installed() { Some(String::from("installed")) } else { None }
    }
    fn install(&self, pm: &PackageManager) -> Result<()> {
        match pm {
            PackageManager::Brew => pm.install_cask(&["android-studio"]),
            _ => {
                if which::which("snap").is_ok() {
                    run_sh("sudo snap install android-studio --classic")
                } else {
                    bail!("Install Android Studio manually from https://developer.android.com/studio")
                }
            }
        }
    }
    fn uninstall(&self, pm: &PackageManager) -> Result<()> {
        match pm {
            PackageManager::Brew => pm.remove(&["android-studio"]),
            _ => {
                if which::which("snap").is_ok() {
                    run_sh("sudo snap remove android-studio")
                } else {
                    run_sh("sudo rm -rf /opt/android-studio")
                }
            }
        }
    }
}

fn run_sh(cmd: &str) -> Result<()> {
    let status = Command::new("sh").arg("-c").arg(cmd).status()?;
    if !status.success() { bail!("command failed"); }
    Ok(())
}
