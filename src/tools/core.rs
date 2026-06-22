use super::{version_of, Category, Tool};
use crate::package_manager::PackageManager;
use anyhow::Result;

pub struct Git;
impl Tool for Git {
    fn name(&self) -> &str { "git" }
    fn description(&self) -> &str { "version control" }
    fn category(&self) -> Category { Category::Core }
    fn is_installed(&self) -> bool { which::which("git").is_ok() }
    fn version(&self) -> Option<String> { version_of("git", &["--version"]) }
    fn install(&self, pm: &PackageManager) -> Result<()> {
        pm.install(&["git"])
    }
    fn uninstall(&self, pm: &PackageManager) -> Result<()> {
        pm.remove(&["git"])
    }
}

pub struct CurlWget;
impl Tool for CurlWget {
    fn name(&self) -> &str { "curl + wget" }
    fn description(&self) -> &str { "HTTP clients" }
    fn category(&self) -> Category { Category::Core }
    fn is_installed(&self) -> bool {
        which::which("curl").is_ok() && which::which("wget").is_ok()
    }
    fn version(&self) -> Option<String> { version_of("curl", &["--version"]) }
    fn install(&self, pm: &PackageManager) -> Result<()> {
        pm.install(&["curl", "wget"])
    }
    fn uninstall(&self, pm: &PackageManager) -> Result<()> {
        pm.remove(&["curl", "wget"])
    }
}

pub struct MakeVim;
impl Tool for MakeVim {
    fn name(&self) -> &str { "make + vim" }
    fn description(&self) -> &str { "build tool, terminal editor" }
    fn category(&self) -> Category { Category::Core }
    fn is_installed(&self) -> bool {
        which::which("make").is_ok() && which::which("vim").is_ok()
    }
    fn version(&self) -> Option<String> { version_of("vim", &["--version"]) }
    fn install(&self, pm: &PackageManager) -> Result<()> {
        pm.install(&["make", "vim"])
    }
    fn uninstall(&self, pm: &PackageManager) -> Result<()> {
        pm.remove(&["make", "vim"])
    }
}
