use crate::tools::{version_of, Category, Tool};
use crate::package_manager::PackageManager;
use anyhow::Result;

pub struct CMakeNinja;

impl Tool for CMakeNinja {
    fn name(&self) -> &str { "CMake + Ninja" }
    fn description(&self) -> &str { "C/C++ build system (cmake.org)" }
    fn category(&self) -> Category { Category::Build }
    fn is_installed(&self) -> bool { which::which("cmake").is_ok() && which::which("ninja").is_ok() }
    fn version(&self) -> Option<String> { version_of("cmake", &["--version"]) }

    fn install(&self, pm: &PackageManager) -> Result<()> { pm.install(&pkgs(pm)) }
    fn uninstall(&self, pm: &PackageManager) -> Result<()> { pm.remove(&pkgs(pm)) }
}

fn pkgs(pm: &PackageManager) -> [&'static str; 2] {
    match pm {
        PackageManager::Pacman | PackageManager::Zypper | PackageManager::Brew => ["cmake", "ninja"],
        _ => ["cmake", "ninja-build"],
    }
}
