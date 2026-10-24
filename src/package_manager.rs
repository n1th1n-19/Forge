use crate::detect::{Distro, OS};
use crate::cmd::{run, run_codes};
use anyhow::{bail, Result};

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
        match self {
            PackageManager::Apt => run(&[
                "sudo", "env", "DEBIAN_FRONTEND=noninteractive", "apt-get", "update", "-y",
            ]),
            // check-update exits 100 when updates are available — that's success
            PackageManager::Dnf => run_codes(&["sudo", "dnf", "check-update"], &[0, 100]),
            PackageManager::Yum => run_codes(&["sudo", "yum", "check-update"], &[0, 100]),
            // Arch only supports refreshing together with a full upgrade
            // (`-Sy` + `-S pkg` is a partial upgrade). main.rs asks before calling this.
            PackageManager::Pacman => run(&["sudo", "pacman", "-Syu", "--noconfirm"]),
            PackageManager::Zypper => run(&["sudo", "zypper", "--non-interactive", "refresh"]),
            PackageManager::Brew => run(&["brew", "update"]),
            PackageManager::Unknown => bail!("no supported package manager detected"),
        }
    }

    pub fn install(&self, pkgs: &[&str]) -> Result<()> {
        let base: &[&str] = match self {
            PackageManager::Apt => &[
                "sudo", "env", "DEBIAN_FRONTEND=noninteractive", "apt-get", "install", "-y",
            ],
            PackageManager::Dnf => &["sudo", "dnf", "install", "-y"],
            PackageManager::Yum => &["sudo", "yum", "install", "-y"],
            PackageManager::Pacman => &["sudo", "pacman", "-S", "--needed", "--noconfirm"],
            PackageManager::Zypper => &["sudo", "zypper", "--non-interactive", "install"],
            PackageManager::Brew => &["brew", "install"],
            PackageManager::Unknown => bail!("no supported package manager detected"),
        };
        run(&[base, pkgs].concat())
    }

    pub fn install_cask(&self, pkgs: &[&str]) -> Result<()> {
        match self {
            PackageManager::Brew => run(&[&["brew", "install", "--cask"], pkgs].concat()),
            _ => self.install(pkgs),
        }
    }

    pub fn remove(&self, pkgs: &[&str]) -> Result<()> {
        let base: &[&str] = match self {
            PackageManager::Apt => &[
                "sudo", "env", "DEBIAN_FRONTEND=noninteractive", "apt-get", "remove", "-y",
            ],
            PackageManager::Dnf => &["sudo", "dnf", "remove", "-y"],
            PackageManager::Yum => &["sudo", "yum", "remove", "-y"],
            PackageManager::Pacman => &["sudo", "pacman", "-R", "--noconfirm"],
            PackageManager::Zypper => &["sudo", "zypper", "--non-interactive", "remove"],
            PackageManager::Brew => &["brew", "uninstall"],
            PackageManager::Unknown => bail!("no supported package manager detected"),
        };
        run(&[base, pkgs].concat())
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
