use crate::tools::{version_of, Category, Tool};
use crate::package_manager::PackageManager;
use anyhow::Result;

pub struct RedisClient;

impl Tool for RedisClient {
    fn name(&self) -> &str { "redis-cli" }
    fn description(&self) -> &str { "Redis CLI client (redis.io)" }
    fn category(&self) -> Category { Category::Databases }
    fn is_installed(&self) -> bool { which::which("redis-cli").is_ok() }
    fn version(&self) -> Option<String> { version_of("redis-cli", &["--version"]) }

    fn install(&self, pm: &PackageManager) -> Result<()> {
        match pm {
            PackageManager::Apt    => pm.install(&["redis-tools"]),
            PackageManager::Dnf | PackageManager::Yum => pm.install(&["redis"]),
            PackageManager::Pacman => pm.install(&["redis"]),
            PackageManager::Zypper => pm.install(&["redis"]),
            PackageManager::Brew   => pm.install(&["redis"]),
            _ => pm.install(&["redis-tools"]),
        }
    }

    fn uninstall(&self, pm: &PackageManager) -> Result<()> {
        match pm {
            PackageManager::Apt  => pm.remove(&["redis-tools"]),
            _ => pm.remove(&["redis"]),
        }
    }
}
