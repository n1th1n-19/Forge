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

    fn install(&self, pm: &PackageManager) -> Result<()> {
        match pm {
            PackageManager::Apt    => pm.install(&["cmake", "ninja-build"]),
            PackageManager::Dnf | PackageManager::Yum => pm.install(&["cmake", "ninja-build"]),
            PackageManager::Pacman => pm.install(&["cmake", "ninja"]),
            PackageManager::Zypper => pm.install(&["cmake", "ninja"]),
            PackageManager::Brew   => pm.install(&["cmake", "ninja"]),
            _ => pm.install(&["cmake", "ninja-build"]),
        }
    }

    fn uninstall(&self, pm: &PackageManager) -> Result<()> {
        match pm {
            PackageManager::Apt    => pm.remove(&["cmake", "ninja-build"]),
            PackageManager::Pacman | PackageManager::Brew => pm.remove(&["cmake", "ninja"]),
            _ => pm.remove(&["cmake", "ninja-build"]),
        }
    }
}
