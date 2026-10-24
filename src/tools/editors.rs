use super::{has_snap, sh, snap_has, version_of, Category, Tool};
use crate::package_manager::PackageManager;
use anyhow::{bail, Result};

pub struct VSCode;

const MS_KEY: &str = "https://packages.microsoft.com/keys/microsoft.asc";
const RPM_REPO: &str = "[code]
name=Visual Studio Code
baseurl=https://packages.microsoft.com/yumrepos/vscode
enabled=1
autorefresh=1
type=rpm-md
gpgcheck=1
gpgkey=https://packages.microsoft.com/keys/microsoft.asc";

impl Tool for VSCode {
    fn name(&self) -> &str { "VS Code" }
    fn description(&self) -> &str { "code editor (Microsoft official)" }
    fn category(&self) -> Category { Category::Editors }
    fn is_installed(&self) -> bool { which::which("code").is_ok() }
    fn version(&self) -> Option<String> { version_of("code", &["--version"]) }

    fn install(&self, pm: &PackageManager) -> Result<()> {
        match pm {
            PackageManager::Brew => pm.install_cask(&["visual-studio-code"]),
            PackageManager::Apt => {
                sh(&format!(
                    "sudo mkdir -p -m 755 /etc/apt/keyrings && \
                     curl -fsSL {MS_KEY} | sudo gpg --batch --yes --dearmor -o /etc/apt/keyrings/microsoft.gpg && \
                     sudo chmod 644 /etc/apt/keyrings/microsoft.gpg && \
                     echo \"deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/microsoft.gpg] https://packages.microsoft.com/repos/code stable main\" \
                     | sudo tee /etc/apt/sources.list.d/vscode.list > /dev/null"
                ))?;
                pm.update()?;
                pm.install(&["code"])
            }
            PackageManager::Dnf | PackageManager::Yum | PackageManager::Zypper => {
                let dir = if *pm == PackageManager::Zypper { "/etc/zypp/repos.d" } else { "/etc/yum.repos.d" };
                sh(&format!(
                    "sudo rpm --import {MS_KEY} && printf '%s\\n' '{RPM_REPO}' | sudo tee {dir}/vscode.repo > /dev/null"
                ))?;
                pm.install(&["code"])
            }
            // Microsoft's build is AUR-only on Arch; snap is the official alternative
            PackageManager::Pacman if has_snap() => sh("sudo snap install code --classic"),
            PackageManager::Pacman => bail!("Install snap first or use an AUR helper: yay -S visual-studio-code-bin"),
            PackageManager::Unknown => bail!("Install VS Code manually from https://code.visualstudio.com"),
        }
    }

    fn uninstall(&self, pm: &PackageManager) -> Result<()> {
        match pm {
            PackageManager::Brew => pm.remove(&["visual-studio-code"]),
            _ if snap_has("code") => sh("sudo snap remove code"),
            _ => pm.remove(&["code"]),
        }
    }
}
