pub mod aws;
pub mod bun;
pub mod cli_tools;
pub mod core;
pub mod docker;
pub mod editors;
pub mod gcloud;
pub mod go;
pub mod jdk;
pub mod node;
pub mod php;
pub mod python;
pub mod ruby;
pub mod rust_lang;
pub mod shell;
pub mod terraform;
pub mod vcs;

pub mod pins;

pub mod build;
pub mod databases;
pub mod mobile;

pub use crate::cmd::{run_remote, sh};
use crate::package_manager::PackageManager;
use anyhow::Result;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Category {
    Core,
    Editors,
    VersionControl,
    Languages,
    DevOps,
    CliTools,
    Shell,
    Mobile,
    Databases,
    Build,
}

impl std::fmt::Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Category::Core          => write!(f, "CORE"),
            Category::Editors       => write!(f, "EDITORS"),
            Category::VersionControl => write!(f, "VERSION CONTROL"),
            Category::Languages     => write!(f, "LANGUAGES"),
            Category::DevOps        => write!(f, "DEVOPS & CLOUD"),
            Category::CliTools      => write!(f, "CLI TOOLS"),
            Category::Shell         => write!(f, "SHELL"),
            Category::Mobile        => write!(f, "MOBILE"),
            Category::Databases     => write!(f, "DATABASES"),
            Category::Build         => write!(f, "BUILD & INFRA"),
        }
    }
}

pub trait Tool: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn category(&self) -> Category;
    fn is_installed(&self) -> bool;
    fn install(&self, pm: &PackageManager) -> Result<()>;
    fn uninstall(&self, pm: &PackageManager) -> Result<()>;
    fn version(&self) -> Option<String>;
}

pub fn all_tools() -> Vec<Box<dyn Tool>> {
    vec![
        // Core
        Box::new(core::Git),
        Box::new(core::CurlWget),
        Box::new(core::MakeVim),
        // Editors
        Box::new(editors::VSCode),
        // Version Control
        Box::new(vcs::GithubCLI),
        // Languages
        Box::new(node::Node),
        Box::new(bun::Bun),
        Box::new(python::Python),
        Box::new(jdk::JDK),
        Box::new(go::Go),
        Box::new(rust_lang::RustLang),
        Box::new(ruby::Ruby),
        Box::new(php::PHP),
        // DevOps & Cloud
        Box::new(docker::Docker),
        Box::new(aws::AwsCLI),
        Box::new(gcloud::GCloud),
        Box::new(terraform::Terraform),
        // CLI Tools
        Box::new(cli_tools::FzfBundle),
        Box::new(cli_tools::JqTreeHtop),
        Box::new(cli_tools::HTTPie),
        Box::new(cli_tools::Tmux),
        // Shell
        Box::new(shell::Zsh),
        Box::new(shell::OhMyZsh),
        Box::new(shell::Starship),
        // Mobile
        Box::new(mobile::android_studio::AndroidStudio),
        Box::new(mobile::android_tools::AndroidPlatformTools),
        Box::new(mobile::flutter::Flutter),
        Box::new(mobile::react_native::ReactNative),
        Box::new(mobile::xcode::XcodeTools),
        Box::new(mobile::cocoapods::CocoaPods),
        // Databases
        Box::new(databases::postgres::PostgresClient),
        Box::new(databases::mysql::MySQLClient),
        Box::new(databases::mongodb::MongoshClient),
        Box::new(databases::redis::RedisClient),
        // Build & Infra
        Box::new(build::maven::Maven),
        Box::new(build::gradle::Gradle),
        Box::new(build::cmake::CMakeNinja),
        Box::new(build::kubectl::KubectlHelm),
        Box::new(build::podman::Podman),
    ]
}

pub fn version_of(cmd: &str, args: &[&str]) -> Option<String> {
    let o = std::process::Command::new(cmd).args(args).output().ok()?;
    let out = if o.stdout.is_empty() { o.stderr } else { o.stdout };
    Some(short_version(&String::from_utf8_lossy(&out))).filter(|s| !s.is_empty())
}

/// First line, cut at " (", max 40 chars (char-safe — byte slicing panics on UTF-8).
fn short_version(out: &str) -> String {
    let first = out.lines().next().unwrap_or("").trim();
    let short = first.split(" (").next().unwrap_or(first).trim();
    short.chars().take(40).collect::<String>().trim().to_string()
}

pub fn home() -> PathBuf {
    PathBuf::from(std::env::var("HOME").unwrap_or_default())
}

/// Shell rc files that exist — never create one the user doesn't have.
fn rc_files() -> Vec<PathBuf> {
    [".profile", ".bashrc", ".zshrc"]
        .iter()
        .map(|f| home().join(f))
        .filter(|p| p.exists())
        .collect()
}

/// Append `line` to each existing rc file, once.
pub fn add_rc_line(line: &str) -> Result<()> {
    for f in rc_files() {
        let body = std::fs::read_to_string(&f)?;
        if !body.lines().any(|l| l.trim() == line) {
            let sep = if body.is_empty() || body.ends_with('\n') { "" } else { "\n" };
            std::fs::write(&f, format!("{body}{sep}{line}\n"))?;
        }
    }
    Ok(())
}

/// Drop lines containing any of `needles` from each existing rc file.
pub fn remove_rc_lines(needles: &[&str]) -> Result<()> {
    for f in rc_files() {
        let body = std::fs::read_to_string(&f)?;
        let kept: Vec<&str> = body.lines().filter(|l| !needles.iter().any(|n| l.contains(n))).collect();
        if kept.len() != body.lines().count() {
            std::fs::write(&f, kept.join("\n") + "\n")?;
        }
    }
    Ok(())
}

pub fn snap_has(name: &str) -> bool {
    std::process::Command::new("snap")
        .args(["list", name])
        .output()
        .is_ok_and(|o| o.status.success())
}

pub fn has_snap() -> bool {
    which::which("snap").is_ok()
}

/// Value of a key in /etc/os-release, e.g. `os_release("VERSION_CODENAME")`.
pub fn os_release(key: &str) -> Option<String> {
    let body = std::fs::read_to_string("/etc/os-release").ok()?;
    body.lines()
        .find_map(|l| l.strip_prefix(&format!("{key}=")))
        .map(|v| v.trim_matches('"').to_string())
        .filter(|v| !v.is_empty())
}

pub fn succeeds(cmd: &str, args: &[&str]) -> bool {
    std::process::Command::new(cmd)
        .args(args)
        .output()
        .is_ok_and(|o| o.status.success())
}

#[cfg(test)]
mod tests {
    use super::short_version;

    #[test]
    fn short_version_cuts_paren_and_first_line() {
        assert_eq!(short_version("git version 2.43.0 (Apple)\nmore"), "git version 2.43.0");
    }

    #[test]
    fn short_version_multibyte_does_not_panic() {
        let s = format!("{}é tail", "a".repeat(39));
        assert_eq!(short_version(&s).chars().count(), 40);
    }
}
