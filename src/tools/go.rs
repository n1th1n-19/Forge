use super::{add_rc_line, remove_rc_lines, sh, version_of, Category, Tool};
use crate::package_manager::PackageManager;
use anyhow::Result;

pub struct Go;

const PATH_LINE: &str = "export PATH=$PATH:/usr/local/go/bin";

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
                sh(r#"
                    GOARCH=$(uname -m | sed 's/x86_64/amd64/;s/aarch64/arm64/')
                    LATEST=$(curl -fsSL "https://go.dev/VERSION?m=text" | head -1)
                    dir=$(mktemp -d); trap 'rm -rf "$dir"' EXIT
                    URL="https://dl.google.com/go/${LATEST}.linux-${GOARCH}.tar.gz"
                    curl -fsSL "$URL" -o "$dir/go.tar.gz"
                    verify_sha256 "$dir/go.tar.gz" "$(curl -fsSL "$URL.sha256")"
                    sudo rm -rf /usr/local/go
                    sudo tar -C /usr/local -xzf "$dir/go.tar.gz"
                "#)?;
                add_rc_line(PATH_LINE)
            }
        }
    }

    fn uninstall(&self, pm: &PackageManager) -> Result<()> {
        match pm {
            PackageManager::Brew => pm.remove(&["go"]),
            _ => {
                sh("sudo rm -rf /usr/local/go")?;
                remove_rc_lines(&[PATH_LINE])
            }
        }
    }
}
