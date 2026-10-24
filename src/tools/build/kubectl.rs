use crate::tools::{pins, run_remote, sh, Category, Tool};
use crate::package_manager::PackageManager;
use anyhow::{bail, Result};

pub struct KubectlHelm;

// pkgs.k8s.io repos are per-minor; bump when a new minor ships.
const K8S_MINOR: &str = "v1.37";

impl Tool for KubectlHelm {
    fn name(&self) -> &str { "kubectl + Helm" }
    fn description(&self) -> &str { "Kubernetes CLI + chart manager" }
    fn category(&self) -> Category { Category::Build }
    fn is_installed(&self) -> bool {
        which::which("kubectl").is_ok() && which::which("helm").is_ok()
    }
    fn version(&self) -> Option<String> {
        // `--short` was removed in kubectl 1.28; first line is "Client Version: v1.x.y"
        let o = std::process::Command::new("kubectl").args(["version", "--client"]).output().ok()?;
        String::from_utf8_lossy(&o.stdout)
            .lines()
            .next()
            .and_then(|l| l.strip_prefix("Client Version: "))
            .map(|v| format!("kubectl {v}"))
    }

    fn install(&self, pm: &PackageManager) -> Result<()> {
        match pm {
            PackageManager::Brew => pm.install(&["kubectl", "helm"]),
            PackageManager::Pacman => pm.install(&["kubectl", "helm"]),
            PackageManager::Zypper => pm.install(&["kubernetes-client", "helm"]),
            PackageManager::Apt => {
                sh(&format!(
                    "sudo mkdir -p -m 755 /etc/apt/keyrings && \
                     curl -fsSL https://pkgs.k8s.io/core:/stable:/{K8S_MINOR}/deb/Release.key \
                     | sudo gpg --batch --yes --dearmor -o /etc/apt/keyrings/kubernetes-apt-keyring.gpg && \
                     echo 'deb [signed-by=/etc/apt/keyrings/kubernetes-apt-keyring.gpg] https://pkgs.k8s.io/core:/stable:/{K8S_MINOR}/deb/ /' \
                     | sudo tee /etc/apt/sources.list.d/kubernetes.list > /dev/null"
                ))?;
                pm.update()?;
                pm.install(&["kubectl"])?;
                run_remote(pins::HELM.url, pins::HELM.sha256, "bash", "")
            }
            PackageManager::Dnf | PackageManager::Yum => {
                sh(&format!(
                    r#"sudo tee /etc/yum.repos.d/kubernetes.repo > /dev/null << 'EOF'
[kubernetes]
name=Kubernetes
baseurl=https://pkgs.k8s.io/core:/stable:/{K8S_MINOR}/rpm/
enabled=1
gpgcheck=1
gpgkey=https://pkgs.k8s.io/core:/stable:/{K8S_MINOR}/rpm/repodata/repomd.xml.key
EOF"#
                ))?;
                pm.install(&["kubectl"])?;
                run_remote(pins::HELM.url, pins::HELM.sha256, "bash", "")
            }
            PackageManager::Unknown => bail!("Install kubectl + Helm manually from https://kubernetes.io/docs/tasks/tools/"),
        }
    }

    fn uninstall(&self, pm: &PackageManager) -> Result<()> {
        match pm {
            PackageManager::Brew | PackageManager::Pacman => pm.remove(&["kubectl", "helm"]),
            PackageManager::Zypper => pm.remove(&["kubernetes-client", "helm"]),
            _ => {
                pm.remove(&["kubectl"])?;
                sh("sudo rm -f /usr/local/bin/helm")
            }
        }
    }
}
