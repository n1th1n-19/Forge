use crate::tools::{version_of, Category, Tool};
use crate::package_manager::PackageManager;
use anyhow::Result;

pub struct MySQLClient;

impl Tool for MySQLClient {
    fn name(&self) -> &str { "MySQL client" }
    fn description(&self) -> &str { "mysql CLI (dev.mysql.com)" }
    fn category(&self) -> Category { Category::Databases }
    fn is_installed(&self) -> bool { which::which("mysql").is_ok() }
    fn version(&self) -> Option<String> { version_of("mysql", &["--version"]) }

    fn install(&self, pm: &PackageManager) -> Result<()> {
        match pm {
            PackageManager::Apt    => pm.install(&["mysql-client", "default-mysql-client"]),
            PackageManager::Dnf | PackageManager::Yum => pm.install(&["mysql"]),
            PackageManager::Pacman => pm.install(&["mysql-clients"]),
            PackageManager::Zypper => pm.install(&["mysql-client"]),
            PackageManager::Brew   => pm.install(&["mysql-client"]),
            _ => pm.install(&["mysql-client"]),
        }
    }

    fn uninstall(&self, pm: &PackageManager) -> Result<()> {
        match pm {
            PackageManager::Apt    => pm.remove(&["mysql-client", "default-mysql-client"]),
            PackageManager::Brew   => pm.remove(&["mysql-client"]),
            PackageManager::Pacman => pm.remove(&["mysql-clients"]),
            _ => pm.remove(&["mysql"]),
        }
    }
}
