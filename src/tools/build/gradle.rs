use crate::tools::{has_snap, sh, snap_has, Category, Tool};
use crate::package_manager::PackageManager;
use anyhow::Result;

pub struct Gradle;

impl Tool for Gradle {
    fn name(&self) -> &str { "Gradle" }
    fn description(&self) -> &str { "Android/Java build tool (gradle.org)" }
    fn category(&self) -> Category { Category::Build }
    fn is_installed(&self) -> bool { which::which("gradle").is_ok() }
    fn version(&self) -> Option<String> {
        // first line of `gradle --version` is blank; "Gradle 9.x" follows
        let o = std::process::Command::new("gradle").arg("--version").output().ok()?;
        String::from_utf8_lossy(&o.stdout)
            .lines()
            .find(|l| l.starts_with("Gradle "))
            .map(str::to_string)
    }

    fn install(&self, pm: &PackageManager) -> Result<()> {
        match pm {
            PackageManager::Brew => pm.install(&["gradle"]),
            _ if has_snap() => sh("sudo snap install gradle --classic"),
            _ => {
                pm.install(&["unzip"])?;
                // services.gradle.org returns pretty JSON: "version" : "9.8.2"
                sh(r#"
                    LATEST=$(curl -fsSL https://services.gradle.org/versions/current | grep -o '"version" *: *"[^"]*"' | head -1 | cut -d'"' -f4)
                    # network-supplied: must look like 9.8.2 before it touches a URL or sudo path
                    [[ $LATEST =~ ^[0-9]+(\.[0-9]+)*$ ]] || { echo "unexpected Gradle version: $LATEST" >&2; exit 1; }
                    dir=$(mktemp -d); trap 'rm -rf "$dir"' EXIT
                    URL="https://services.gradle.org/distributions/gradle-${LATEST}-bin.zip"
                    curl -fsSL "$URL" -o "$dir/gradle.zip"
                    verify_sha256 "$dir/gradle.zip" "$(curl -fsSL "$URL.sha256")"
                    sudo unzip -qo "$dir/gradle.zip" -d /opt/
                    sudo ln -sfn "/opt/gradle-${LATEST}/bin/gradle" /usr/local/bin/gradle
                "#)
            }
        }
    }

    fn uninstall(&self, pm: &PackageManager) -> Result<()> {
        match pm {
            PackageManager::Brew => pm.remove(&["gradle"]),
            _ if snap_has("gradle") => sh("sudo snap remove gradle"),
            // remove only the version forge linked, not every /opt/gradle-* the user installed
            _ => sh(r#"
                target=$(readlink -f /usr/local/bin/gradle) || { echo "no /usr/local/bin/gradle" >&2; exit 1; }
                [[ $target =~ ^/opt/gradle-[0-9.]+/bin/gradle$ ]] || { echo "gradle wasn't installed by forge ($target)" >&2; exit 1; }
                sudo rm -f /usr/local/bin/gradle
                sudo rm -rf "${target%/bin/gradle}"
            "#),
        }
    }
}
