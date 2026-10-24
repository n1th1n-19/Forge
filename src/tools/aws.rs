use super::{sh, Category, Tool};
use crate::package_manager::PackageManager;
use anyhow::Result;

pub struct AwsCLI;

impl Tool for AwsCLI {
    fn name(&self) -> &str { "AWS CLI v2" }
    fn description(&self) -> &str { "awscli.amazonaws.com official" }
    fn category(&self) -> Category { Category::DevOps }
    fn is_installed(&self) -> bool { which::which("aws").is_ok() }
    fn version(&self) -> Option<String> {
        // "aws-cli/2.x.y Python/3.x ..." → "aws-cli/2.x.y"
        let o = std::process::Command::new("aws").arg("--version").output().ok()?;
        let out = if o.stdout.is_empty() { o.stderr } else { o.stdout };
        String::from_utf8_lossy(&out)
            .split_whitespace()
            .find(|t| t.starts_with("aws-cli/"))
            .map(str::to_string)
    }

    fn install(&self, pm: &PackageManager) -> Result<()> {
        match pm {
            PackageManager::Brew => pm.install(&["awscli"]),
            _ => {
                pm.install(&["unzip"])?;
                sh(r#"
                    dir=$(mktemp -d); trap 'rm -rf "$dir"' EXIT
                    curl -fsSL "https://awscli.amazonaws.com/awscli-exe-linux-$(uname -m).zip" -o "$dir/aws.zip"
                    unzip -q "$dir/aws.zip" -d "$dir"
                    sudo "$dir/aws/install" --update
                "#)
            }
        }
    }

    fn uninstall(&self, pm: &PackageManager) -> Result<()> {
        match pm {
            PackageManager::Brew => pm.remove(&["awscli"]),
            _ => sh("sudo rm -f /usr/local/bin/aws /usr/local/bin/aws_completer && sudo rm -rf /usr/local/aws-cli"),
        }
    }
}
