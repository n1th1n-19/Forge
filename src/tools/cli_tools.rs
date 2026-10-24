use super::{pins, sh, version_of, Category, Tool};
use crate::package_manager::PackageManager;
use anyhow::Result;

pub struct FzfBundle;
impl Tool for FzfBundle {
    fn name(&self) -> &str { "fzf + ripgrep + bat + eza" }
    fn description(&self) -> &str { "fuzzy finder, fast grep, better cat, better ls" }
    fn category(&self) -> Category { Category::CliTools }
    fn is_installed(&self) -> bool {
        // Debian/Ubuntu ship bat as `batcat`
        which::which("fzf").is_ok()
            && which::which("rg").is_ok()
            && (which::which("bat").is_ok() || which::which("batcat").is_ok())
            && which::which("eza").is_ok()
    }
    fn version(&self) -> Option<String> { version_of("fzf", &["--version"]).map(|v| format!("fzf {v}")) }

    fn install(&self, pm: &PackageManager) -> Result<()> {
        match pm {
            PackageManager::Apt => {
                pm.install(&["fzf", "ripgrep", "bat"])?;
                // eza is in apt on Ubuntu 24.04+/Debian 13+; older releases get the GitHub binary
                pm.install(&["eza"]).or_else(|_| install_eza_release())
            }
            _ => pm.install(&["fzf", "ripgrep", "bat", "eza"]),
        }
    }

    fn uninstall(&self, pm: &PackageManager) -> Result<()> {
        match pm {
            PackageManager::Apt => {
                pm.remove(&["fzf", "ripgrep", "bat"])?;
                let _ = pm.remove(&["eza"]);
                sh("sudo rm -f /usr/local/bin/eza")
            }
            _ => pm.remove(&["fzf", "ripgrep", "bat", "eza"]),
        }
    }
}

fn install_eza_release() -> Result<()> {
    let (arch, sha) = match std::env::consts::ARCH {
        "aarch64" => ("aarch64", pins::EZA_SHA256_AARCH64),
        _ => ("x86_64", pins::EZA_SHA256_X86_64),
    };
    sh(&format!(
        r#"dir=$(mktemp -d); trap 'rm -rf "$dir"' EXIT
curl -fsSL "https://github.com/eza-community/eza/releases/download/{v}/eza_{arch}-unknown-linux-gnu.tar.gz" -o "$dir/eza.tar.gz"
verify_sha256 "$dir/eza.tar.gz" '{sha}'
tar -xzf "$dir/eza.tar.gz" -C "$dir"
sudo install -m 755 "$dir/eza" /usr/local/bin/eza"#,
        v = pins::EZA_VERSION
    ))
}

pub struct JqTreeHtop;
impl Tool for JqTreeHtop {
    fn name(&self) -> &str { "jq + tree + htop" }
    fn description(&self) -> &str { "JSON processor, dir tree, process monitor" }
    fn category(&self) -> Category { Category::CliTools }
    fn is_installed(&self) -> bool {
        which::which("jq").is_ok() && which::which("tree").is_ok() && which::which("htop").is_ok()
    }
    fn version(&self) -> Option<String> { version_of("jq", &["--version"]) }
    fn install(&self, pm: &PackageManager) -> Result<()> { pm.install(&["jq", "tree", "htop"]) }
    fn uninstall(&self, pm: &PackageManager) -> Result<()> { pm.remove(&["jq", "tree", "htop"]) }
}

pub struct HTTPie;
impl Tool for HTTPie {
    fn name(&self) -> &str { "HTTPie" }
    fn description(&self) -> &str { "modern HTTP client for APIs (httpie.io)" }
    fn category(&self) -> Category { Category::CliTools }
    fn is_installed(&self) -> bool { which::which("http").is_ok() }
    fn version(&self) -> Option<String> { version_of("http", &["--version"]) }
    // packaged everywhere supported; `pip install` is blocked by PEP 668 on modern distros
    fn install(&self, pm: &PackageManager) -> Result<()> { pm.install(&["httpie"]) }
    fn uninstall(&self, pm: &PackageManager) -> Result<()> { pm.remove(&["httpie"]) }
}

pub struct Tmux;
impl Tool for Tmux {
    fn name(&self) -> &str { "tmux" }
    fn description(&self) -> &str { "terminal multiplexer" }
    fn category(&self) -> Category { Category::CliTools }
    fn is_installed(&self) -> bool { which::which("tmux").is_ok() }
    fn version(&self) -> Option<String> { version_of("tmux", &["-V"]) }
    fn install(&self, pm: &PackageManager) -> Result<()> { pm.install(&["tmux"]) }
    fn uninstall(&self, pm: &PackageManager) -> Result<()> { pm.remove(&["tmux"]) }
}
