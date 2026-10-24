use super::{os_release, sh, version_of, Category, Tool};
use crate::package_manager::PackageManager;
use anyhow::{bail, Result};

pub struct Terraform;

impl Tool for Terraform {
    fn name(&self) -> &str { "Terraform" }
    fn description(&self) -> &str { "HashiCorp (releases.hashicorp.com)" }
    fn category(&self) -> Category { Category::DevOps }
    fn is_installed(&self) -> bool { which::which("terraform").is_ok() }
    fn version(&self) -> Option<String> { version_of("terraform", &["version"]) }

    fn install(&self, pm: &PackageManager) -> Result<()> {
        match pm {
            PackageManager::Brew => pm.install(&["hashicorp/tap/terraform"]),
            PackageManager::Apt => {
                // os-release instead of lsb_release, which minimal images lack
                let codename = os_release("UBUNTU_CODENAME")
                    .or_else(|| os_release("VERSION_CODENAME"))
                    .unwrap_or_else(|| "noble".into());
                // goes into a root-written apt source line — keep it to a plain word
                if !codename.chars().all(|c| c.is_ascii_alphanumeric()) {
                    bail!("unexpected distro codename {codename:?}");
                }
                sh(&format!(
                    "curl -fsSL https://apt.releases.hashicorp.com/gpg \
                     | sudo gpg --batch --yes --dearmor -o /usr/share/keyrings/hashicorp-archive-keyring.gpg && \
                     echo 'deb [signed-by=/usr/share/keyrings/hashicorp-archive-keyring.gpg] https://apt.releases.hashicorp.com {codename} main' \
                     | sudo tee /etc/apt/sources.list.d/hashicorp.list > /dev/null"
                ))?;
                pm.update()?;
                pm.install(&["terraform"])
            }
            PackageManager::Dnf | PackageManager::Yum => {
                // fetch the .repo file directly — works on yum, dnf4 and dnf5 (whose config-manager syntax changed)
                let distro = if os_release("ID").as_deref() == Some("fedora") { "fedora" } else { "RHEL" };
                sh(&format!(
                    "sudo curl -fsSL https://rpm.releases.hashicorp.com/{distro}/hashicorp.repo -o /etc/yum.repos.d/hashicorp.repo"
                ))?;
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
