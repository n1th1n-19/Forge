use super::{version_of, Category, Tool};
use crate::package_manager::PackageManager;
use anyhow::{bail, Result};
use std::process::Command;

pub struct Go;

impl Tool for Go {
    fn name(&self) -> &str { "Go" }
    fn description(&self) -> &str { "go.dev official binary" }
    fn category(&self) -> Category { Category::Languages }
    fn is_installed(&self) -> bool { which::which("go").is_ok() }
    fn version(&self) -> Option<String> { version_of("go", &["version"]) }

    fn install(&self, pm: &PackageManager) -> Result<()> {
        match pm {
            PackageManager::Brew => pm.install(&["go"]),
            _ => {
                // Download latest stable from go.dev
                run_sh(r#"
                    GOARCH=$(uname -m | sed 's/x86_64/amd64/;s/aarch64/arm64/')
                    GOOS=$(uname -s | tr '[:upper:]' '[:lower:]')
                    LATEST=$(curl -fsSL "https://go.dev/VERSION?m=text" | head -1)
                    TARBALL="${LATEST}.${GOOS}-${GOARCH}.tar.gz"
                    curl -fsSL "https://go.dev/dl/${TARBALL}" -o /tmp/go.tar.gz
                    sudo rm -rf /usr/local/go
                    sudo tar -C /usr/local -xzf /tmp/go.tar.gz
                    rm /tmp/go.tar.gz
                    echo 'export PATH=$PATH:/usr/local/go/bin' >> "$HOME/.profile"
                    echo 'export PATH=$PATH:/usr/local/go/bin' >> "$HOME/.bashrc"
                    echo 'export PATH=$PATH:/usr/local/go/bin' >> "$HOME/.zshrc" 2>/dev/null || true
                "#)
            }
        }
    }

    fn uninstall(&self, pm: &PackageManager) -> Result<()> {
        match pm {
            PackageManager::Brew => pm.remove(&["go"]),
            _ => run_sh("sudo rm -rf /usr/local/go"),
        }
    }
}

fn run_sh(cmd: &str) -> Result<()> {
    let status = Command::new("sh").arg("-c").arg(cmd).status()?;
    if !status.success() {
        bail!("command failed");
    }
    Ok(())
}
