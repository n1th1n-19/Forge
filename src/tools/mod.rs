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

pub mod build;
pub mod databases;
pub mod mobile;

use crate::package_manager::PackageManager;
use anyhow::Result;

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
    std::process::Command::new(cmd)
        .args(args)
        .output()
        .ok()
        .map(|o| {
            let out = if o.stdout.is_empty() { o.stderr } else { o.stdout };
            let first_line = String::from_utf8_lossy(&out)
                .lines()
                .next()
                .unwrap_or("")
                .trim()
                .to_string();
            let short = first_line
                .split(" (")
                .next()
                .unwrap_or(&first_line)
                .trim()
                .to_string();
            if short.len() > 40 {
                short[..40].trim().to_string()
            } else {
                short
            }
        })
        .filter(|s| !s.is_empty())
}
