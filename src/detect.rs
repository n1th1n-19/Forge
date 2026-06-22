use std::fs;
use std::process::Command;

#[derive(Debug, Clone, PartialEq)]
pub enum Distro {
    Ubuntu,
    Debian,
    Fedora,
    RHEL,
    CentOS,
    Arch,
    Manjaro,
    OpenSUSE,
    Unknown(String),
}

impl std::fmt::Display for Distro {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Distro::Ubuntu => write!(f, "Ubuntu"),
            Distro::Debian => write!(f, "Debian"),
            Distro::Fedora => write!(f, "Fedora"),
            Distro::RHEL => write!(f, "RHEL"),
            Distro::CentOS => write!(f, "CentOS"),
            Distro::Arch => write!(f, "Arch"),
            Distro::Manjaro => write!(f, "Manjaro"),
            Distro::OpenSUSE => write!(f, "openSUSE"),
            Distro::Unknown(s) => write!(f, "{}", s),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum OS {
    MacOS,
    Linux(Distro),
    WSL(Distro),
    Windows,
    Unknown,
}

impl std::fmt::Display for OS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OS::MacOS => write!(f, "macOS"),
            OS::Linux(d) => write!(f, "Linux ({})", d),
            OS::WSL(d) => write!(f, "WSL ({})", d),
            OS::Windows => write!(f, "Windows"),
            OS::Unknown => write!(f, "Unknown"),
        }
    }
}

pub fn detect() -> OS {
    let target_os = std::env::consts::OS;

    match target_os {
        "macos" => OS::MacOS,
        "windows" => OS::Windows,
        "linux" => {
            let distro = detect_distro();
            if is_wsl() {
                OS::WSL(distro)
            } else {
                OS::Linux(distro)
            }
        }
        _ => OS::Unknown,
    }
}

fn is_wsl() -> bool {
    if let Ok(contents) = fs::read_to_string("/proc/version") {
        let lower = contents.to_lowercase();
        return lower.contains("microsoft") || lower.contains("wsl");
    }
    if let Ok(contents) = fs::read_to_string("/proc/sys/kernel/osrelease") {
        let lower = contents.to_lowercase();
        return lower.contains("microsoft") || lower.contains("wsl");
    }
    false
}

fn detect_distro() -> Distro {
    if let Ok(contents) = fs::read_to_string("/etc/os-release") {
        return parse_os_release(&contents);
    }
    if let Ok(output) = Command::new("lsb_release").arg("-si").output() {
        let s = String::from_utf8_lossy(&output.stdout).trim().to_lowercase();
        return match_distro_name(&s);
    }
    Distro::Unknown(String::from("unknown"))
}

fn parse_os_release(contents: &str) -> Distro {
    let mut id = String::new();
    let mut id_like = String::new();

    for line in contents.lines() {
        if let Some(val) = line.strip_prefix("ID=") {
            id = val.trim_matches('"').to_lowercase();
        }
        if let Some(val) = line.strip_prefix("ID_LIKE=") {
            id_like = val.trim_matches('"').to_lowercase();
        }
    }

    let primary = match_distro_name(&id);
    if primary != Distro::Unknown(id.clone()) {
        return primary;
    }
    match_distro_name(&id_like)
}

fn match_distro_name(name: &str) -> Distro {
    if name.contains("ubuntu") {
        Distro::Ubuntu
    } else if name.contains("debian") {
        Distro::Debian
    } else if name.contains("fedora") {
        Distro::Fedora
    } else if name.contains("rhel") || name.contains("redhat") {
        Distro::RHEL
    } else if name.contains("centos") {
        Distro::CentOS
    } else if name.contains("manjaro") {
        Distro::Manjaro
    } else if name.contains("arch") {
        Distro::Arch
    } else if name.contains("opensuse") || name.contains("suse") {
        Distro::OpenSUSE
    } else {
        Distro::Unknown(name.to_string())
    }
}
