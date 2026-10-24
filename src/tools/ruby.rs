use super::{pins, remove_rc_lines, run_remote, sh, version_of, Category, Tool};
use crate::package_manager::PackageManager;
use anyhow::Result;

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
                // `rbenv install` compiles Ruby — needs a toolchain and headers
                pm.install(&build_deps(pm))?;
                run_remote(pins::RBENV.url, pins::RBENV.sha256, "bash", "")?;
                sh(r#"
                    export PATH="$HOME/.rbenv/bin:$PATH"
                    eval "$(rbenv init - bash)"
                    LATEST=$(rbenv install -l 2>/dev/null | grep -E '^[[:space:]]*[0-9]+\.[0-9]+\.[0-9]+$' | tail -1 | tr -d ' ')
                    [ -n "$LATEST" ] || { echo "could not resolve latest Ruby version" >&2; exit 1; }
                    rbenv install -s "$LATEST"
                    rbenv global "$LATEST"
                "#)
            }
        }
    }

    fn uninstall(&self, pm: &PackageManager) -> Result<()> {
        match pm {
            PackageManager::Brew => pm.remove(&["rbenv", "ruby-build"]),
            _ => {
                sh(r#"rm -rf "$HOME/.rbenv""#)?;
                remove_rc_lines(&["rbenv init", ".rbenv/bin"])
            }
        }
    }
}

fn build_deps(pm: &PackageManager) -> Vec<&'static str> {
    match pm {
        PackageManager::Apt => vec![
            "git", "build-essential", "libssl-dev", "libyaml-dev", "zlib1g-dev", "libffi-dev", "libreadline-dev",
        ],
        PackageManager::Dnf | PackageManager::Yum => vec![
            "git", "gcc", "make", "openssl-devel", "libyaml-devel", "zlib-devel", "libffi-devel", "readline-devel",
        ],
        PackageManager::Pacman => vec!["git", "base-devel", "openssl", "libyaml", "zlib", "libffi", "readline"],
        PackageManager::Zypper => vec![
            "git", "gcc", "make", "libopenssl-devel", "libyaml-devel", "zlib-devel", "libffi-devel", "readline-devel",
        ],
        _ => vec!["git"],
    }
}
