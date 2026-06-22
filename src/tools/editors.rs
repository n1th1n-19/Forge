use super::{version_of, Category, Tool};
use crate::package_manager::PackageManager;
use anyhow::{bail, Result};
use std::process::Command;

pub struct VSCode;

impl Tool for VSCode {
    fn name(&self) -> &str { "VS Code" }
    fn description(&self) -> &str { "code editor (Microsoft official)" }
    fn category(&self) -> Category { Category::Editors }
    fn is_installed(&self) -> bool {
        which::which("code").is_ok()
    }
    fn version(&self) -> Option<String> { version_of("code", &["--version"]) }

    fn install(&self, pm: &PackageManager) -> Result<()> {
        match pm {
            PackageManager::Brew => pm.install_cask(&["visual-studio-code"]),
            PackageManager::Apt => {
                // Add Microsoft apt repo then install
                run_sh("wget -qO- https://packages.microsoft.com/keys/microsoft.asc | gpg --dearmor > /tmp/microsoft.gpg && sudo install -D -o root -g root -m 644 /tmp/microsoft.gpg /etc/apt/keyrings/microsoft.gpg")?;
                run_sh("echo 'deb [arch=amd64 signed-by=/etc/apt/keyrings/microsoft.gpg] https://packages.microsoft.com/repos/code stable main' | sudo tee /etc/apt/sources.list.d/vscode.list > /dev/null")?;
                pm.update()?;
                pm.install(&["code"])
            }
            PackageManager::Dnf | PackageManager::Yum => {
                run_sh("sudo rpm --import https://packages.microsoft.com/keys/microsoft.asc")?;
                run_sh("sudo sh -c 'echo -e \"[code]\\nname=Visual Studio Code\\nbaseurl=https://packages.microsoft.com/yumrepos/vscode\\nenabled=1\\nautorefresh=1\\ntype=rpm-md\\ngpgcheck=1\\ngpgkey=https://packages.microsoft.com/keys/microsoft.asc\" > /etc/yum.repos.d/vscode.repo'")?;
                pm.install(&["code"])
            }
            PackageManager::Pacman => {
                // Try snap fallback
                if which::which("snap").is_ok() {
                    run_sh("sudo snap install code --classic")
                } else {
                    bail!("Install snap first or use an AUR helper: yay -S visual-studio-code-bin")
                }
            }
            _ => {
                if which::which("snap").is_ok() {
                    run_sh("sudo snap install code --classic")
                } else {
                    bail!("Unsupported package manager for VS Code. Install manually from https://code.visualstudio.com")
                }
            }
        }
    }

    fn uninstall(&self, pm: &PackageManager) -> Result<()> {
        match pm {
            PackageManager::Brew => pm.remove(&["visual-studio-code"]),
            _ => pm.remove(&["code"]),
        }
    }
}

fn run_sh(cmd: &str) -> Result<()> {
    let status = Command::new("sh").arg("-c").arg(cmd).status()?;
    if !status.success() {
        bail!("command failed: {}", cmd);
    }
    Ok(())
}
