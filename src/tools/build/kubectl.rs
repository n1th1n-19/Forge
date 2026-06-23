use crate::tools::{version_of, Category, Tool};
use crate::package_manager::PackageManager;
use anyhow::{bail, Result};
use std::process::Command;

pub struct KubectlHelm;

impl Tool for KubectlHelm {
    fn name(&self) -> &str { "kubectl + Helm" }
    fn description(&self) -> &str { "Kubernetes CLI + chart manager" }
    fn category(&self) -> Category { Category::Build }
    fn is_installed(&self) -> bool {
        which::which("kubectl").is_ok() && which::which("helm").is_ok()
    }
    fn version(&self) -> Option<String> { version_of("kubectl", &["version", "--client", "--short"]) }

    fn install(&self, pm: &PackageManager) -> Result<()> {
        match pm {
            PackageManager::Brew => {
                pm.install(&["kubectl"])?;
                pm.install(&["helm"])
            }
            PackageManager::Apt => {
                // kubectl
                run_sh("curl -fsSL https://pkgs.k8s.io/core:/stable:/v1.32/deb/Release.key | sudo gpg --dearmor -o /etc/apt/keyrings/kubernetes-apt-keyring.gpg")?;
                run_sh("echo 'deb [signed-by=/etc/apt/keyrings/kubernetes-apt-keyring.gpg] https://pkgs.k8s.io/core:/stable:/v1.32/deb/ /' | sudo tee /etc/apt/sources.list.d/kubernetes.list")?;
                pm.update()?;
                pm.install(&["kubectl"])?;
                // helm
                run_sh("curl -fsSL https://raw.githubusercontent.com/helm/helm/main/scripts/get-helm-3 | bash")
            }
            PackageManager::Dnf | PackageManager::Yum => {
                run_sh(r#"sudo tee /etc/yum.repos.d/kubernetes.repo << 'EOF'
[kubernetes]
name=Kubernetes
baseurl=https://pkgs.k8s.io/core:/stable:/v1.32/rpm/
enabled=1
gpgcheck=1
gpgkey=https://pkgs.k8s.io/core:/stable:/v1.32/rpm/repodata/repomd.xml.key
EOF"#)?;
                pm.install(&["kubectl"])?;
                run_sh("curl -fsSL https://raw.githubusercontent.com/helm/helm/main/scripts/get-helm-3 | bash")
            }
            _ => bail!("Install kubectl + Helm manually from https://kubernetes.io/docs/tasks/tools/"),
        }
    }

    fn uninstall(&self, pm: &PackageManager) -> Result<()> {
        match pm {
            PackageManager::Brew => {
                pm.remove(&["kubectl"])?;
                pm.remove(&["helm"])
            }
            _ => {
                pm.remove(&["kubectl"])?;
                run_sh("sudo rm -f $(which helm)")
            }
        }
    }
}

fn run_sh(cmd: &str) -> Result<()> {
    let status = Command::new("sh").arg("-c").arg(cmd).status()?;
    if !status.success() { bail!("command failed"); }
    Ok(())
}
