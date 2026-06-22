use super::{version_of, Category, Tool};
use crate::package_manager::PackageManager;
use anyhow::{bail, Result};
use std::process::Command;

pub struct Node;

impl Tool for Node {
    fn name(&self) -> &str { "Node.js" }
    fn description(&self) -> &str { "via nvm — node + npm" }
    fn category(&self) -> Category { Category::Languages }
    fn is_installed(&self) -> bool { which::which("node").is_ok() }
    fn version(&self) -> Option<String> { version_of("node", &["--version"]) }

    fn install(&self, _pm: &PackageManager) -> Result<()> {
        // Install nvm then latest LTS node
        run_sh("curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.3/install.sh | bash")?;
        // Source nvm and install LTS in same shell
        run_sh(r#"export NVM_DIR="$HOME/.nvm" && [ -s "$NVM_DIR/nvm.sh" ] && . "$NVM_DIR/nvm.sh" && nvm install --lts && nvm use --lts && nvm alias default 'lts/*'"#)
    }

    fn uninstall(&self, _pm: &PackageManager) -> Result<()> {
        run_sh(r#"export NVM_DIR="$HOME/.nvm" && [ -s "$NVM_DIR/nvm.sh" ] && . "$NVM_DIR/nvm.sh" && nvm deactivate && nvm uninstall --lts"#)?;
        run_sh("rm -rf \"$HOME/.nvm\"")
    }
}

fn run_sh(cmd: &str) -> Result<()> {
    let status = Command::new("bash").arg("-c").arg(cmd).status()?;
    if !status.success() {
        bail!("command failed: {}", cmd);
    }
    Ok(())
}
