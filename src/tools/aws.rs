use super::{Category, Tool};
use crate::package_manager::PackageManager;
use anyhow::{bail, Result};
use std::process::Command;

pub struct AwsCLI;

impl Tool for AwsCLI {
    fn name(&self) -> &str { "AWS CLI v2" }
    fn description(&self) -> &str { "awscli.amazonaws.com official" }
    fn category(&self) -> Category { Category::DevOps }
    fn is_installed(&self) -> bool { which::which("aws").is_ok() }
    fn version(&self) -> Option<String> {
        std::process::Command::new("aws")
            .arg("--version")
            .output()
            .ok()
            .and_then(|o| {
                let out = if o.stdout.is_empty() {
                    String::from_utf8_lossy(&o.stderr).to_string()
                } else {
                    String::from_utf8_lossy(&o.stdout).to_string()
                };
                out.split_whitespace()
                    .find(|t| t.starts_with("aws-cli/"))
                    .map(|t| t.to_string())
            })
    }

    fn install(&self, pm: &PackageManager) -> Result<()> {
        match pm {
            PackageManager::Brew => pm.install(&["awscli"]),
            _ => {
                let arch = std::env::consts::ARCH;
                let url = if arch == "aarch64" {
                    "https://awscli.amazonaws.com/awscli-exe-linux-aarch64.zip"
                } else {
                    "https://awscli.amazonaws.com/awscli-exe-linux-x86_64.zip"
                };
                run_sh(&format!(
                    "curl -fsSL '{}' -o /tmp/awscliv2.zip && \
                     unzip -q /tmp/awscliv2.zip -d /tmp/ && \
                     sudo /tmp/aws/install && \
                     rm -rf /tmp/awscliv2.zip /tmp/aws",
                    url
                ))
            }
        }
    }

    fn uninstall(&self, pm: &PackageManager) -> Result<()> {
        match pm {
            PackageManager::Brew => pm.remove(&["awscli"]),
            _ => run_sh("sudo rm -f $(which aws) && sudo rm -rf /usr/local/aws-cli"),
        }
    }
}

fn run_sh(cmd: &str) -> Result<()> {
    let status = Command::new("sh").arg("-c").arg(cmd).status()?;
    if !status.success() {
        bail!("command failed");
    }
    Ok(())
}
