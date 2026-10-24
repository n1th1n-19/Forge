use super::{add_rc_line, remove_rc_lines, sh, version_of, Category, Tool};
use crate::package_manager::PackageManager;
use anyhow::Result;

pub struct Bun;

impl Tool for Bun {
    fn name(&self) -> &str { "Bun" }
    fn description(&self) -> &str { "fast JS runtime + package manager (bun.sh)" }
    fn category(&self) -> Category { Category::Languages }
    fn is_installed(&self) -> bool { which::which("bun").is_ok() }
    fn version(&self) -> Option<String> { version_of("bun", &["--version"]) }

    fn install(&self, pm: &PackageManager) -> Result<()> {
        match pm {
            PackageManager::Brew => pm.install(&["oven-sh/bun/bun"]),
            _ => {
                pm.install(&["unzip"])?;
                // bun.sh/install isn't versioned in the repo, so fetch the release zip
                // directly and check it against Bun's published SHASUMS256.txt
                sh(r#"
                    case "$(uname -m)" in
                      aarch64|arm64) t=aarch64 ;;
                      *) t=x64; grep -q avx2 /proc/cpuinfo || t=x64-baseline ;;
                    esac
                    base=https://github.com/oven-sh/bun/releases/latest/download
                    dir=$(mktemp -d); trap 'rm -rf "$dir"' EXIT
                    curl -fsSL "$base/bun-linux-$t.zip" -o "$dir/bun.zip"
                    curl -fsSL "$base/SHASUMS256.txt" -o "$dir/SHASUMS256.txt"
                    verify_sha256 "$dir/bun.zip" "$(grep " bun-linux-$t.zip$" "$dir/SHASUMS256.txt" | cut -d' ' -f1)"
                    unzip -qo "$dir/bun.zip" -d "$dir"
                    mkdir -p "$HOME/.bun/bin"
                    install -m 755 "$dir/bun-linux-$t/bun" "$HOME/.bun/bin/bun"
                    ln -sf bun "$HOME/.bun/bin/bunx"
                "#)?;
                add_rc_line(r#"export BUN_INSTALL="$HOME/.bun""#)?;
                add_rc_line(r#"export PATH="$BUN_INSTALL/bin:$PATH""#)
            }
        }
    }

    fn uninstall(&self, pm: &PackageManager) -> Result<()> {
        match pm {
            PackageManager::Brew => pm.remove(&["bun"]),
            _ => {
                sh(r#"rm -rf "$HOME/.bun""#)?;
                // only the lines the bun installer adds — not every line containing "bun" (e.g. "ubuntu")
                remove_rc_lines(&["BUN_INSTALL", "# bun completions", "/.bun/_bun"])
            }
        }
    }
}
