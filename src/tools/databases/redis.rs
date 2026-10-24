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

    fn install(&self, pm: &PackageManager) -> Result<()> { pm.install(&[pkg(pm)]) }
    fn uninstall(&self, pm: &PackageManager) -> Result<()> { pm.remove(&[pkg(pm)]) }
}

fn pkg(pm: &PackageManager) -> &'static str {
    match pm {
        PackageManager::Apt | PackageManager::Unknown => "redis-tools",
        _ => "redis",
    }
}
