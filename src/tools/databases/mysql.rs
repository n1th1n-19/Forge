use crate::tools::{sh, version_of, Category, Tool};
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
        pm.install(&[pkg(pm)])?;
        if *pm == PackageManager::Brew {
            // keg-only formula — not on PATH until linked
            sh("brew link --force mysql-client")?;
        }
        Ok(())
    }

    fn uninstall(&self, pm: &PackageManager) -> Result<()> { pm.remove(&[pkg(pm)]) }
}

fn pkg(pm: &PackageManager) -> &'static str {
    match pm {
        // `default-mysql-client` exists on both Debian and Ubuntu; `mysql-client` is Ubuntu-only
        PackageManager::Apt => "default-mysql-client",
        PackageManager::Dnf | PackageManager::Yum => "mysql",
        PackageManager::Pacman => "mariadb-clients",
        PackageManager::Zypper => "mariadb-client",
        _ => "mysql-client",
    }
}
