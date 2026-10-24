use super::{home, pins, remove_rc_lines, run_remote, sh, version_of, Category, Tool};
use crate::package_manager::PackageManager;
use anyhow::{bail, Result};

pub struct Node;

/// Exactly what the pinned nvm installer appends to rc files.
const NVM_RC_LINES: [&str; 3] = [
    r#"export NVM_DIR="$HOME/.nvm""#,
    r#"[ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"  # This loads nvm"#,
    r#"[ -s "$NVM_DIR/bash_completion" ] && \. "$NVM_DIR/bash_completion"  # This loads nvm bash_completion"#,
];

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
        // `which node` may be a distro/nodesource node forge never installed
        if !home()?.join(".nvm").exists() {
            bail!("Node isn't managed by nvm (~/.nvm missing) — remove it with the tool that installed it");
        }
        sh(r#"rm -rf "$HOME/.nvm""#)?;
        remove_rc_lines(|l| NVM_RC_LINES.contains(&l))
    }
}
