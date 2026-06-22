use super::{version_of, Category, Tool};
use crate::package_manager::PackageManager;
use anyhow::{bail, Result};
use std::process::Command;

pub struct Ruby;

impl Tool for Ruby {
    fn name(&self) -> &str { "Ruby" }
    fn description(&self) -> &str { "rbenv — Ruby version manager" }
    fn category(&self) -> Category { Category::Languages }
    fn is_installed(&self) -> bool { which::which("ruby").is_ok() }
    fn version(&self) -> Option<String> { version_of("ruby", &["--version"]) }

    fn install(&self, pm: &PackageManager) -> Result<()> {
        match pm {
            PackageManager::Brew => pm.install(&["rbenv", "ruby-build"]),
            _ => {
                run_sh("curl -fsSL https://github.com/rbenv/rbenv-installer/raw/HEAD/bin/rbenv-installer | bash")?;
                run_sh(r#"
                    export PATH="$HOME/.rbenv/bin:$PATH"
                    eval "$(rbenv init -)"
                    LATEST=$(rbenv install -l 2>/dev/null | grep -E '^\s+[0-9]+\.[0-9]+\.[0-9]+$' | tail -1 | tr -d ' ')
                    rbenv install "$LATEST"
                    rbenv global "$LATEST"
                "#)
            }
        }
    }

    fn uninstall(&self, pm: &PackageManager) -> Result<()> {
        match pm {
            PackageManager::Brew => pm.remove(&["rbenv", "ruby-build"]),
            _ => run_sh("rm -rf \"$HOME/.rbenv\""),
        }
    }
}

fn run_sh(cmd: &str) -> Result<()> {
    let status = Command::new("bash").arg("-c").arg(cmd).status()?;
    if !status.success() {
        bail!("command failed");
    }
    Ok(())
}
