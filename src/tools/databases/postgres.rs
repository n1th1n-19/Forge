use crate::tools::{sh, version_of, Category, Tool};
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
        pm.install(&[pkg(pm)])?;
        if *pm == PackageManager::Brew {
            // keg-only formula — not on PATH until linked
            sh("brew link --force libpq")?;
        }
        Ok(())
    }

    fn uninstall(&self, pm: &PackageManager) -> Result<()> { pm.remove(&[pkg(pm)]) }
}

fn pkg(pm: &PackageManager) -> &'static str {
    match pm {
        PackageManager::Dnf | PackageManager::Yum | PackageManager::Zypper => "postgresql",
        PackageManager::Pacman => "postgresql-libs",
        PackageManager::Brew => "libpq",
        _ => "postgresql-client",
    }
}
