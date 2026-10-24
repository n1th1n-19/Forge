use super::{sh, version_of, Category, Tool};
use crate::package_manager::PackageManager;
use anyhow::{bail, Result};

pub struct GCloud;

impl Tool for GCloud {
    fn name(&self) -> &str { "gcloud SDK" }
    fn description(&self) -> &str { "Google Cloud CLI (packages.cloud.google.com)" }
    fn category(&self) -> Category { Category::DevOps }
    fn is_installed(&self) -> bool { which::which("gcloud").is_ok() }
    fn version(&self) -> Option<String> { version_of("gcloud", &["--version"]) }

    fn install(&self, pm: &PackageManager) -> Result<()> {
        match pm {
            PackageManager::Brew => pm.install_cask(&["gcloud-cli"]),
            PackageManager::Apt => {
                sh("curl -fsSL https://packages.cloud.google.com/apt/doc/apt-key.gpg \
                    | sudo gpg --batch --yes --dearmor -o /usr/share/keyrings/cloud.google.gpg && \
                    echo 'deb [signed-by=/usr/share/keyrings/cloud.google.gpg] https://packages.cloud.google.com/apt cloud-sdk main' \
                    | sudo tee /etc/apt/sources.list.d/google-cloud-sdk.list > /dev/null")?;
                pm.update()?;
                pm.install(&["google-cloud-cli"])
            }
            PackageManager::Dnf | PackageManager::Yum => {
                sh(r#"sudo tee /etc/yum.repos.d/google-cloud-sdk.repo > /dev/null << EOF
[google-cloud-cli]
name=Google Cloud CLI
baseurl=https://packages.cloud.google.com/yum/repos/cloud-sdk-el9-$(uname -m)
enabled=1
gpgcheck=1
repo_gpgcheck=0
gpgkey=https://packages.cloud.google.com/yum/doc/rpm-package-key.gpg
EOF"#)?;
                pm.install(&["google-cloud-cli"])
            }
            _ => bail!("Install gcloud manually from https://cloud.google.com/sdk/docs/install"),
        }
    }

    fn uninstall(&self, pm: &PackageManager) -> Result<()> {
        match pm {
            PackageManager::Brew => pm.remove(&["gcloud-cli"]),
            _ => pm.remove(&["google-cloud-cli"]),
        }
    }
}
