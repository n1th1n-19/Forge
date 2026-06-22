use super::{version_of, Category, Tool};
use crate::package_manager::PackageManager;
use anyhow::{bail, Result};
use std::process::Command;

pub struct Docker;

impl Tool for Docker {
    fn name(&self) -> &str { "Docker + Compose" }
    fn description(&self) -> &str { "get.docker.com official script" }
    fn category(&self) -> Category { Category::DevOps }
    fn is_installed(&self) -> bool { which::which("docker").is_ok() }
    fn version(&self) -> Option<String> { version_of("docker", &["--version"]) }

    fn install(&self, pm: &PackageManager) -> Result<()> {
        match pm {
            PackageManager::Brew => pm.install_cask(&["docker"]),
            _ => {
                run_sh("curl -fsSL https://get.docker.com | sh")?;
                // Add current user to docker group
                run_sh("sudo usermod -aG docker \"$USER\" || true")
            }
        }
    }

    fn uninstall(&self, pm: &PackageManager) -> Result<()> {
        match pm {
            PackageManager::Brew => pm.remove(&["docker"]),
            PackageManager::Apt => {
                pm.remove(&["docker-ce", "docker-ce-cli", "containerd.io", "docker-compose-plugin"])
            }
            PackageManager::Dnf | PackageManager::Yum => {
                pm.remove(&["docker-ce", "docker-ce-cli", "containerd.io"])
            }
            _ => bail!("Uninstall Docker manually for this platform"),
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
