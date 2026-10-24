use super::{pins, remove_rc_lines, run_remote, sh, version_of, Category, Tool};
use crate::package_manager::PackageManager;
use anyhow::Result;

pub struct Node;

impl Tool for Node {
    fn name(&self) -> &str { "Node.js" }
    fn description(&self) -> &str { "via nvm — node + npm" }
    fn category(&self) -> Category { Category::Languages }
    fn is_installed(&self) -> bool { which::which("node").is_ok() }
    fn version(&self) -> Option<String> { version_of("node", &["--version"]) }

    fn install(&self, _pm: &PackageManager) -> Result<()> {
        run_remote(pins::NVM.url, pins::NVM.sha256, "bash", "")?;
        sh(r#"export NVM_DIR="$HOME/.nvm"; . "$NVM_DIR/nvm.sh"
nvm install --lts && nvm alias default 'lts/*'"#)
    }

    fn uninstall(&self, _pm: &PackageManager) -> Result<()> {
        sh(r#"rm -rf "$HOME/.nvm""#)?;
        remove_rc_lines(&["NVM_DIR"])
    }
}
