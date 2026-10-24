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
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

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
    /// Base-system tools the OS may depend on; `uninstall --all` skips them,
    /// they must be named explicitly.
    fn protected(&self) -> bool { false }
    /// Something the user must know after a successful install.
    fn post_install_note(&self) -> Option<&str> { None }
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

/// The user's home dir. Errors instead of falling back to "" — an empty HOME
/// would make every rc-file path relative to the current directory.
pub fn home() -> Result<PathBuf> {
    std::env::var_os("HOME")
        .map(PathBuf::from)
        .filter(|h| h.is_absolute() && h.parent().is_some())
        .context("HOME is unset, relative or / — refusing to touch dotfiles")
}

/// Shell rc files that exist — never create one the user doesn't have.
fn rc_files() -> Result<Vec<PathBuf>> {
    let home = home()?;
    Ok([".profile", ".bash_profile", ".bashrc", ".zshrc"]
        .iter()
        .map(|f| home.join(f))
        .filter(|p| p.exists())
        .collect())
}

/// Append `line` to each existing rc file, once.
pub fn add_rc_line(line: &str) -> Result<()> {
    rc_files()?.iter().try_for_each(|f| add_line_to(f, line))
}

/// Drop lines matching `is_ours` (compared trimmed) from each existing rc file.
/// Match exact lines forge or a known installer wrote — never substrings a user might also use.
pub fn remove_rc_lines(is_ours: impl Fn(&str) -> bool) -> Result<()> {
    rc_files()?.iter().try_for_each(|f| remove_lines_from(f, &is_ours))
}

fn add_line_to(f: &Path, line: &str) -> Result<()> {
    let body = std::fs::read_to_string(f)?;
    if body.lines().any(|l| l.trim() == line) {
        return Ok(());
    }
    let nl = if body.contains("\r\n") { "\r\n" } else { "\n" };
    let sep = if body.is_empty() || body.ends_with('\n') { "" } else { nl };
    write_atomic(f, &format!("{body}{sep}{line}{nl}"))
}

fn remove_lines_from(f: &Path, is_ours: &dyn Fn(&str) -> bool) -> Result<()> {
    let body = std::fs::read_to_string(f)?;
    // split_inclusive keeps each line's own ending, so CRLF files stay CRLF
    let kept: String = body.split_inclusive('\n').filter(|l| !is_ours(l.trim())).collect();
    if kept.len() != body.len() {
        write_atomic(f, &kept)?;
    }
    Ok(())
}

/// Write via temp file + rename so a crash or full disk can't leave a truncated rc file.
/// Resolves symlinks first so stow/chezmoi-managed dotfiles stay links.
fn write_atomic(f: &Path, body: &str) -> Result<()> {
    let real = std::fs::canonicalize(f)?;
    let mut tmp = real.clone().into_os_string();
    tmp.push(".forge-tmp");
    let tmp = PathBuf::from(tmp);
    std::fs::write(&tmp, body)?;
    std::fs::set_permissions(&tmp, std::fs::metadata(&real)?.permissions())?;
    std::fs::rename(&tmp, &real)?;
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
    use super::*;

    fn scratch(name: &str, body: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("forge-test-{}-{name}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let f = dir.join("rc");
        std::fs::write(&f, body).unwrap();
        f
    }

    #[test]
    fn rc_add_is_idempotent_and_remove_is_exact() {
        let f = scratch("exact", "alias x=1\r\n# my NVM_DIR notes\r\n");
        add_line_to(&f, "export A=1").unwrap();
        add_line_to(&f, "export A=1").unwrap();
        assert_eq!(std::fs::read_to_string(&f).unwrap(), "alias x=1\r\n# my NVM_DIR notes\r\nexport A=1\r\n");
        remove_lines_from(&f, &|l| l == "export A=1").unwrap();
        // CRLF kept, user line mentioning NVM_DIR untouched
        assert_eq!(std::fs::read_to_string(&f).unwrap(), "alias x=1\r\n# my NVM_DIR notes\r\n");
    }

    #[cfg(unix)]
    #[test]
    fn rc_edit_keeps_symlinked_dotfile_a_symlink() {
        let target = scratch("link", "x\n");
        let link = target.with_file_name("link");
        let _ = std::fs::remove_file(&link);
        std::os::unix::fs::symlink(&target, &link).unwrap();
        add_line_to(&link, "y").unwrap();
        assert!(std::fs::symlink_metadata(&link).unwrap().file_type().is_symlink());
        assert_eq!(std::fs::read_to_string(&target).unwrap(), "x\ny\n");
    }

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
