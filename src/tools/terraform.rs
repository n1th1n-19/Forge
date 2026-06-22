use super::{version_of, Category, Tool};
use crate::package_manager::PackageManager;
use anyhow::{bail, Result};
use std::process::Command;

pub struct Terraform;

impl Tool for Terraform {
    fn name(&self) -> &str { "Terraform" }
    fn description(&self) -> &str { "HashiCorp (releases.hashicorp.com)" }
    fn category(&self) -> Category { Category::DevOps }
    fn is_installed(&self) -> bool { which::which("terraform").is_ok() }
    fn version(&self) -> Option<String> { version_of("terraform", &["version"]) }

    fn install(&self, pm: &PackageManager) -> Result<()> {
        match pm {
            PackageManager::Brew => pm.install(&["terraform"]),
            PackageManager::Apt => {
                run_sh("wget -O- https://apt.releases.hashicorp.com/gpg | sudo gpg --dearmor -o /usr/share/keyrings/hashicorp-archive-keyring.gpg")?;
                run_sh("echo \"deb [signed-by=/usr/share/keyrings/hashicorp-archive-keyring.gpg] https://apt.releases.hashicorp.com $(lsb_release -cs) main\" | sudo tee /etc/apt/sources.list.d/hashicorp.list")?;
                pm.update()?;
                pm.install(&["terraform"])
            }
            PackageManager::Dnf | PackageManager::Yum => {
                run_sh("sudo dnf config-manager --add-repo https://rpm.releases.hashicorp.com/RHEL/hashicorp.repo")?;
                pm.install(&["terraform"])
            }
            PackageManager::Pacman => pm.install(&["terraform"]),
            _ => bail!("Install Terraform manually from https://developer.hashicorp.com/terraform/install"),
        }
    }

    fn uninstall(&self, pm: &PackageManager) -> Result<()> {
        pm.remove(&["terraform"])
    }
}

fn run_sh(cmd: &str) -> Result<()> {
    let status = Command::new("sh").arg("-c").arg(cmd).status()?;
    if !status.success() {
        bail!("command failed");
    }
    Ok(())
}
