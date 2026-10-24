use crate::tools::{add_rc_line, has_snap, home, remove_rc_lines, sh, snap_has, version_of, Category, Tool};
use crate::package_manager::PackageManager;
use anyhow::{bail, Result};

pub struct Flutter;

const PATH_LINE: &str = r#"export PATH="$PATH:$HOME/flutter/bin""#;

impl Tool for Flutter {
    fn name(&self) -> &str { "Flutter" }
    fn description(&self) -> &str { "cross-platform SDK + Dart (flutter.dev)" }
    fn category(&self) -> Category { Category::Mobile }
    fn is_installed(&self) -> bool { which::which("flutter").is_ok() }
    fn version(&self) -> Option<String> { version_of("flutter", &["--version"]) }

    fn install(&self, pm: &PackageManager) -> Result<()> {
        match pm {
            PackageManager::Brew => pm.install_cask(&["flutter"]),
            _ if has_snap() => sh("sudo snap install flutter --classic"),
            _ => {
                // git clone of the stable channel works on every arch (tarballs are x64-only)
                pm.install(&["git", "unzip", "xz-utils"]).or_else(|_| pm.install(&["git", "unzip", "xz"]))?;
                sh(r#"git clone --depth 1 -b stable https://github.com/flutter/flutter.git "$HOME/flutter""#)?;
                add_rc_line(PATH_LINE)
            }
        }
    }

    fn uninstall(&self, pm: &PackageManager) -> Result<()> {
        match pm {
            PackageManager::Brew => pm.remove(&["flutter"]),
            _ if snap_has("flutter") => sh("sudo snap remove flutter"),
            _ => {
                // ~/flutter may be the user's own checkout; only remove it if it's the flutter on PATH
                let ours = home()?.join("flutter/bin/flutter");
                let on_path = which::which("flutter").ok().and_then(|p| p.canonicalize().ok());
                if on_path.is_none() || on_path != ours.canonicalize().ok() {
                    bail!("flutter on PATH isn't ~/flutter — remove it with the tool that installed it");
                }
                sh(r#"rm -rf "$HOME/flutter""#)?;
                remove_rc_lines(|l| l == PATH_LINE)
            }
        }
    }
}
