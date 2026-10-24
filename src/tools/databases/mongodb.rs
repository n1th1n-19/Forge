use crate::tools::{os_release, sh, version_of, Category, Tool};
use crate::package_manager::PackageManager;
use anyhow::{bail, Result};

pub struct MongoshClient;

impl Tool for MongoshClient {
    fn name(&self) -> &str { "mongosh" }
    fn description(&self) -> &str { "MongoDB shell (mongodb.com)" }
    fn category(&self) -> Category { Category::Databases }
    fn is_installed(&self) -> bool { which::which("mongosh").is_ok() }
    fn version(&self) -> Option<String> { version_of("mongosh", &["--version"]) }

    fn install(&self, pm: &PackageManager) -> Result<()> {
        match pm {
            PackageManager::Brew => pm.install(&["mongosh"]),
            PackageManager::Apt => {
                // MongoDB publishes separate Ubuntu ("multiverse") and Debian ("main") repos
                let (distro, component) = match os_release("ID").as_deref() {
                    Some("debian") => ("debian", "main"),
                    _ => ("ubuntu", "multiverse"),
                };
                let codename = os_release("UBUNTU_CODENAME")
                    .or_else(|| os_release("VERSION_CODENAME"))
                    .unwrap_or_else(|| "noble".into());
                // goes into a root-written apt source line — keep it to a plain word
                if !codename.chars().all(|c| c.is_ascii_alphanumeric()) {
                    bail!("unexpected distro codename {codename:?}");
                }
                sh(&format!(
                    "curl -fsSL https://www.mongodb.org/static/pgp/server-8.0.asc \
                     | sudo gpg --batch --yes --dearmor -o /usr/share/keyrings/mongodb-server-8.0.gpg && \
                     echo 'deb [ signed-by=/usr/share/keyrings/mongodb-server-8.0.gpg ] https://repo.mongodb.org/apt/{distro} {codename}/mongodb-org/8.0 {component}' \
                     | sudo tee /etc/apt/sources.list.d/mongodb-org-8.0.list > /dev/null"
                ))?;
                pm.update()?;
                pm.install(&["mongodb-mongosh"])
            }
            PackageManager::Dnf | PackageManager::Yum => {
                sh(r#"sudo tee /etc/yum.repos.d/mongodb-org-8.0.repo > /dev/null << 'EOF'
[mongodb-org-8.0]
name=MongoDB Repository
baseurl=https://repo.mongodb.org/yum/redhat/9/mongodb-org/8.0/$basearch/
gpgcheck=1
enabled=1
gpgkey=https://www.mongodb.org/static/pgp/server-8.0.asc
EOF"#)?;
                pm.install(&["mongodb-mongosh"])
            }
            _ => bail!("Install mongosh manually from https://www.mongodb.com/try/download/shell"),
        }
    }

    fn uninstall(&self, pm: &PackageManager) -> Result<()> {
        match pm {
            PackageManager::Brew => pm.remove(&["mongosh"]),
            _ => pm.remove(&["mongodb-mongosh"]),
        }
    }
}
