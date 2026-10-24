use super::{sh, succeeds, version_of, Category, Tool};
use crate::package_manager::PackageManager;
use anyhow::Result;

pub struct JDK;

impl Tool for JDK {
    fn name(&self) -> &str { "OpenJDK 21" }
    fn description(&self) -> &str { "Java Development Kit (LTS)" }
    fn category(&self) -> Category { Category::Languages }
    // macOS ships a /usr/bin/java stub that exists without a JDK — run it to be sure
    fn is_installed(&self) -> bool { succeeds("java", &["-version"]) }
    fn version(&self) -> Option<String> { version_of("java", &["-version"]) }

    fn install(&self, pm: &PackageManager) -> Result<()> {
        pm.install(&[pkg(pm)])?;
        if *pm == PackageManager::Brew {
            // keg-only: register with macOS so /usr/bin/java finds it
            sh("sudo ln -sfn \"$(brew --prefix)/opt/openjdk@21/libexec/openjdk.jdk\" /Library/Java/JavaVirtualMachines/openjdk-21.jdk")?;
        }
        Ok(())
    }

    fn uninstall(&self, pm: &PackageManager) -> Result<()> {
        if *pm == PackageManager::Brew {
            sh("sudo rm -f /Library/Java/JavaVirtualMachines/openjdk-21.jdk")?;
        }
        pm.remove(&[pkg(pm)])
    }
}

fn pkg(pm: &PackageManager) -> &'static str {
    match pm {
        PackageManager::Dnf | PackageManager::Yum | PackageManager::Zypper => "java-21-openjdk-devel",
        PackageManager::Pacman => "jdk21-openjdk",
        PackageManager::Brew => "openjdk@21",
        _ => "openjdk-21-jdk",
    }
}
