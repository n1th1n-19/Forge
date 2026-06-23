use crate::tools::{version_of, Category, Tool};
use crate::package_manager::PackageManager;
use anyhow::{bail, Result};
use std::process::Command;

pub struct Flutter;

impl Tool for Flutter {
    fn name(&self) -> &str { "Flutter" }
    fn description(&self) -> &str { "cross-platform SDK + Dart (flutter.dev)" }
    fn category(&self) -> Category { Category::Mobile }
    fn is_installed(&self) -> bool { which::which("flutter").is_ok() }
    fn version(&self) -> Option<String> { version_of("flutter", &["--version"]) }

    fn install(&self, pm: &PackageManager) -> Result<()> {
        match pm {
            PackageManager::Brew => pm.install(&["flutter"]),
            _ => {
                if which::which("snap").is_ok() {
                    run_sh("sudo snap install flutter --classic")
                } else {
                    // Manual install to ~/.flutter
                    run_sh(r#"
                        ARCH=$(uname -m | sed 's/x86_64/x64/;s/aarch64/arm64/')
                        FLUTTER_URL="https://storage.googleapis.com/flutter_infra_release/releases/stable/linux/flutter_linux_stable.tar.xz"
                        curl -fsSL "$FLUTTER_URL" -o /tmp/flutter.tar.xz
                        tar -xJf /tmp/flutter.tar.xz -C "$HOME"
                        rm /tmp/flutter.tar.xz
                        echo 'export PATH="$PATH:$HOME/flutter/bin"' >> "$HOME/.bashrc"
                        echo 'export PATH="$PATH:$HOME/flutter/bin"' >> "$HOME/.zshrc" 2>/dev/null || true
                    "#)
                }
            }
        }
    }

    fn uninstall(&self, pm: &PackageManager) -> Result<()> {
        match pm {
            PackageManager::Brew => pm.remove(&["flutter"]),
            _ => {
                if which::which("snap").is_ok() {
                    run_sh("sudo snap remove flutter")
                } else {
                    run_sh("rm -rf \"$HOME/flutter\"")
                }
            }
        }
    }
}

fn run_sh(cmd: &str) -> Result<()> {
    let status = Command::new("sh").arg("-c").arg(cmd).status()?;
    if !status.success() { bail!("command failed"); }
    Ok(())
}
