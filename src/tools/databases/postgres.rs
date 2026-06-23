use crate::tools::{version_of, Category, Tool};
use crate::package_manager::PackageManager;
use anyhow::Result;

pub struct PostgresClient;

impl Tool for PostgresClient {
    fn name(&self) -> &str { "PostgreSQL client" }
    fn description(&self) -> &str { "psql CLI (postgresql.org)" }
    fn category(&self) -> Category { Category::Databases }
    fn is_installed(&self) -> bool { which::which("psql").is_ok() }
    fn version(&self) -> Option<String> { version_of("psql", &["--version"]) }

    fn install(&self, pm: &PackageManager) -> Result<()> {
        match pm {
            PackageManager::Apt    => pm.install(&["postgresql-client"]),
            PackageManager::Dnf | PackageManager::Yum => pm.install(&["postgresql"]),
            PackageManager::Pacman => pm.install(&["postgresql-libs"]),
            PackageManager::Zypper => pm.install(&["postgresql"]),
            PackageManager::Brew   => pm.install(&["libpq"]),
            _ => pm.install(&["postgresql-client"]),
        }
    }

    fn uninstall(&self, pm: &PackageManager) -> Result<()> {
        match pm {
            PackageManager::Apt    => pm.remove(&["postgresql-client"]),
            PackageManager::Brew   => pm.remove(&["libpq"]),
            PackageManager::Pacman => pm.remove(&["postgresql-libs"]),
            _ => pm.remove(&["postgresql"]),
        }
    }
}
