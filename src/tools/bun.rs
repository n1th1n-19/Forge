use super::{add_rc_line, home, remove_rc_lines, sh, version_of, Category, Tool};
use crate::package_manager::PackageManager;
use anyhow::{bail, Result};

pub struct Bun;

const RC_INSTALL: &str = r#"export BUN_INSTALL="$HOME/.bun""#;
const RC_PATH: &str = r#"export PATH="$BUN_INSTALL/bin:$PATH""#;
/// Lines forge adds, plus the ones bun's own installer adds.
const RC_LINES: [&str; 5] = [
    RC_INSTALL,
    RC_PATH,
    "# bun",
    "# bun completions",
    r#"[ -s "$HOME/.bun/_bun" ] && source "$HOME/.bun/_bun""#,
];

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
                add_rc_line(RC_INSTALL)?;
                add_rc_line(RC_PATH)
            }
        }
    }

    fn uninstall(&self, pm: &PackageManager) -> Result<()> {
        match pm {
            PackageManager::Brew => pm.remove(&["bun"]),
            _ => {
                if !home()?.join(".bun").exists() {
                    bail!("Bun isn't installed in ~/.bun — remove it with the tool that installed it");
                }
                sh(r#"rm -rf "$HOME/.bun""#)?;
                remove_rc_lines(|l| RC_LINES.contains(&l))
            }
        }
    }
}
