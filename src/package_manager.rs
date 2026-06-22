use crate::detect::{Distro, OS};
use anyhow::{bail, Result};
use std::process::Command;

#[derive(Debug, Clone, PartialEq)]
pub enum PackageManager {
    Apt,
    Dnf,
    Yum,
    Pacman,
    Zypper,
    Brew,
    Unknown,
}

impl std::fmt::Display for PackageManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PackageManager::Apt => write!(f, "apt"),
            PackageManager::Dnf => write!(f, "dnf"),
            PackageManager::Yum => write!(f, "yum"),
            PackageManager::Pacman => write!(f, "pacman"),
            PackageManager::Zypper => write!(f, "zypper"),
            PackageManager::Brew => write!(f, "brew"),
            PackageManager::Unknown => write!(f, "unknown"),
        }
    }
}

impl PackageManager {
    pub fn detect(os: &OS) -> Self {
        match os {
            OS::MacOS => PackageManager::Brew,
            OS::Linux(d) | OS::WSL(d) => distro_pkg_manager(d),
            OS::Windows => PackageManager::Unknown,
            OS::Unknown => PackageManager::Unknown,
        }
    }

    pub fn update(&self) -> Result<()> {
        let status = match self {
            PackageManager::Apt => run(&["sudo", "apt-get", "update", "-y"]),
            PackageManager::Dnf => run(&["sudo", "dnf", "check-update"]),
            PackageManager::Yum => run(&["sudo", "yum", "check-update"]),
            PackageManager::Pacman => run(&["sudo", "pacman", "-Sy"]),
            PackageManager::Zypper => run(&["sudo", "zypper", "refresh"]),
            PackageManager::Brew => run(&["brew", "update"]),
            PackageManager::Unknown => bail!("no supported package manager detected"),
        };
        // dnf/yum return exit code 100 when updates available — treat as success
        match status {
            Ok(_) => Ok(()),
            Err(e) => {
                if matches!(self, PackageManager::Dnf | PackageManager::Yum) {
                    Ok(())
                } else {
                    Err(e)
                }
            }
        }
    }

    pub fn install(&self, pkgs: &[&str]) -> Result<()> {
        match self {
            PackageManager::Apt => run(&build_cmd(&["sudo", "apt-get", "install", "-y"], pkgs)),
            PackageManager::Dnf => run(&build_cmd(&["sudo", "dnf", "install", "-y"], pkgs)),
            PackageManager::Yum => run(&build_cmd(&["sudo", "yum", "install", "-y"], pkgs)),
            PackageManager::Pacman => {
                run(&build_cmd(&["sudo", "pacman", "-S", "--noconfirm"], pkgs))
            }
            PackageManager::Zypper => {
                run(&build_cmd(&["sudo", "zypper", "install", "-y"], pkgs))
            }
            PackageManager::Brew => run(&build_cmd(&["brew", "install"], pkgs)),
            PackageManager::Unknown => bail!("no supported package manager detected"),
        }
    }

    pub fn install_cask(&self, pkgs: &[&str]) -> Result<()> {
        match self {
            PackageManager::Brew => {
                run(&build_cmd(&["brew", "install", "--cask"], pkgs))
            }
            _ => self.install(pkgs),
        }
    }

    pub fn remove(&self, pkgs: &[&str]) -> Result<()> {
        match self {
            PackageManager::Apt => {
                run(&build_cmd(&["sudo", "apt-get", "remove", "-y"], pkgs))
            }
            PackageManager::Dnf => run(&build_cmd(&["sudo", "dnf", "remove", "-y"], pkgs)),
            PackageManager::Yum => run(&build_cmd(&["sudo", "yum", "remove", "-y"], pkgs)),
            PackageManager::Pacman => {
                run(&build_cmd(&["sudo", "pacman", "-R", "--noconfirm"], pkgs))
            }
            PackageManager::Zypper => {
                run(&build_cmd(&["sudo", "zypper", "remove", "-y"], pkgs))
            }
            PackageManager::Brew => run(&build_cmd(&["brew", "uninstall"], pkgs)),
            PackageManager::Unknown => bail!("no supported package manager detected"),
        }
    }
}

fn distro_pkg_manager(distro: &Distro) -> PackageManager {
    match distro {
        Distro::Ubuntu | Distro::Debian => PackageManager::Apt,
        Distro::Fedora => PackageManager::Dnf,
        Distro::RHEL | Distro::CentOS => {
            if which::which("dnf").is_ok() {
                PackageManager::Dnf
            } else {
                PackageManager::Yum
            }
        }
        Distro::Arch | Distro::Manjaro => PackageManager::Pacman,
        Distro::OpenSUSE => PackageManager::Zypper,
        Distro::Unknown(_) => {
            if which::which("apt-get").is_ok() {
                PackageManager::Apt
            } else if which::which("dnf").is_ok() {
                PackageManager::Dnf
            } else if which::which("pacman").is_ok() {
                PackageManager::Pacman
            } else if which::which("zypper").is_ok() {
                PackageManager::Zypper
            } else {
                PackageManager::Unknown
            }
        }
    }
}

fn build_cmd<'a>(base: &[&'a str], pkgs: &[&'a str]) -> Vec<&'a str> {
    let mut cmd: Vec<&str> = base.to_vec();
    cmd.extend_from_slice(pkgs);
    cmd
}

fn run(args: &[&str]) -> Result<()> {
    let (prog, rest) = args.split_first().expect("empty command");
    let status = Command::new(prog).args(rest).status()?;
    if !status.success() {
        bail!("command failed: {}", args.join(" "));
    }
    Ok(())
}
