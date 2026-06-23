use crate::tools::{version_of, Category, Tool};
use crate::package_manager::PackageManager;
use anyhow::{bail, Result};
use std::process::Command;

pub struct Gradle;

impl Tool for Gradle {
    fn name(&self) -> &str { "Gradle" }
    fn description(&self) -> &str { "Android/Java build tool (gradle.org)" }
    fn category(&self) -> Category { Category::Build }
    fn is_installed(&self) -> bool { which::which("gradle").is_ok() }
    fn version(&self) -> Option<String> { version_of("gradle", &["--version"]) }

    fn install(&self, pm: &PackageManager) -> Result<()> {
        match pm {
            PackageManager::Brew => pm.install(&["gradle"]),
            _ => {
                if which::which("snap").is_ok() {
                    run_sh("sudo snap install gradle --classic")
                } else if which::which("sdk").is_ok() {
                    run_sh("sdk install gradle")
                } else {
                    // Download latest from gradle.org
                    run_sh(r#"
                        LATEST=$(curl -fsSL https://services.gradle.org/versions/current | grep -o '"version":"[^"]*"' | cut -d'"' -f4)
                        curl -fsSL "https://services.gradle.org/distributions/gradle-${LATEST}-bin.zip" -o /tmp/gradle.zip
                        sudo unzip -q /tmp/gradle.zip -d /opt/
                        sudo ln -sfn "/opt/gradle-${LATEST}/bin/gradle" /usr/local/bin/gradle
                        rm /tmp/gradle.zip
                    "#)
                }
            }
        }
    }

    fn uninstall(&self, pm: &PackageManager) -> Result<()> {
        match pm {
            PackageManager::Brew => pm.remove(&["gradle"]),
            _ => run_sh("sudo rm -f /usr/local/bin/gradle && sudo rm -rf /opt/gradle-*"),
        }
    }
}

fn run_sh(cmd: &str) -> Result<()> {
    let status = Command::new("sh").arg("-c").arg(cmd).status()?;
    if !status.success() { bail!("command failed"); }
    Ok(())
}
