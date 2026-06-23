use crate::tools::{version_of, Category, Tool};
use crate::package_manager::PackageManager;
use anyhow::{bail, Result};
use std::process::Command;

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
            PackageManager::Apt  => {
                run_sh("curl -fsSL https://www.mongodb.org/static/pgp/server-8.0.asc | sudo gpg --dearmor -o /usr/share/keyrings/mongodb-server-8.0.gpg")?;
                run_sh(&format!(
                    "echo 'deb [ signed-by=/usr/share/keyrings/mongodb-server-8.0.gpg ] https://repo.mongodb.org/apt/ubuntu {} /mongodb-org/8.0 multiverse' | sudo tee /etc/apt/sources.list.d/mongodb-org-8.0.list",
                    lsb_codename()
                ))?;
                pm.update()?;
                pm.install(&["mongodb-mongosh"])
            }
            PackageManager::Dnf | PackageManager::Yum => {
                run_sh(r#"sudo tee /etc/yum.repos.d/mongodb-org-8.0.repo << 'EOF'
[mongodb-org-8.0]
name=MongoDB Repository
baseurl=https://repo.mongodb.org/yum/redhat/9/mongodb-org/8.0/x86_64/
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

fn lsb_codename() -> String {
    std::process::Command::new("lsb_release")
        .arg("-cs")
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|_| String::from("noble"))
}

fn run_sh(cmd: &str) -> Result<()> {
    let status = Command::new("sh").arg("-c").arg(cmd).status()?;
    if !status.success() { bail!("command failed"); }
    Ok(())
}
