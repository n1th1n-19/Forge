//! Installer scripts pinned to an immutable tag/commit, with their SHA-256.
//! A mismatch aborts the install instead of running a tampered or changed script.
//!
//! To bump one: point the URL at a newer tag/commit, then
//! `curl -fsSL <url> | sha256sum` and paste the hash.
//! Most of these scripts install the *latest* tool release themselves, so
//! the pin only freezes the installer, not the tool version.

pub struct Script {
    pub url: &'static str,
    pub sha256: &'static str,
}

pub const NVM: Script = Script {
    url: "https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.8/install.sh",
    sha256: "48a0eee9a60e07422dce0eb5774754c83889570ca1ee2566c516acbe8af03a9e",
};

pub const HELM: Script = Script {
    url: "https://raw.githubusercontent.com/helm/helm/v4.3.1/scripts/get-helm-4",
    sha256: "aa9943145fbc13c10ea271735cc8fdf538cebbdaeefc7ae5b021ed86ac01f30f",
};

pub const RUSTUP: Script = Script {
    url: "https://raw.githubusercontent.com/rust-lang/rustup/1.29.2/rustup-init.sh",
    sha256: "9502b514362a21096210d498f143b8cdacda365d3dfcf45e8f5541e089ccafc0",
};

pub const STARSHIP: Script = Script {
    url: "https://raw.githubusercontent.com/starship/starship/v1.27.0/install/install.sh",
    sha256: "52c64f14a558034ebeb1907ea9364e802b32474576fd3e68265f73bc33cc8fbb",
};

pub const OH_MY_ZSH: Script = Script {
    url: "https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/2100c630ba4bd67bb48802c3e7da893a4fdaa1a0/tools/install.sh",
    sha256: "5574b96e94dbcb769f0d1592fa83aeb6ca2caf41c6ae5d76fcc7f04c524b4f55",
};

pub const RBENV: Script = Script {
    url: "https://raw.githubusercontent.com/rbenv/rbenv-installer/a5350919f70fd398f327b2efcd072500ab7f29ce/bin/rbenv-installer",
    sha256: "9a1f19c4564cc8954bca75640d320fcc98cf40187d73531156629edd36606f2f",
};

pub const DOCKER: Script = Script {
    url: "https://raw.githubusercontent.com/docker/docker-install/ada7a567e59b3935bd601bb21e422da50e80e376/install.sh",
    sha256: "2b102d809152ea157a59f171319f489d971273b2d26dd7e9a6d1d737ce89289d",
};

/// eza release binary for older Debian/Ubuntu that lack the apt package.
pub const EZA_VERSION: &str = "v0.23.5";
pub const EZA_SHA256_X86_64: &str = "35c70c5c43c29108075e58b893234c67ef585f0b53a7eaf8e9e7d4eec9f339b4";
pub const EZA_SHA256_AARCH64: &str = "40b87ae8628aa2ff0f0d2dc24ab52f689631366385c3da630bae745671fd71ec";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pins_are_full_sha256() {
        for s in [&NVM, &HELM, &RUSTUP, &STARSHIP, &OH_MY_ZSH, &RBENV, &DOCKER] {
            assert!(s.sha256.len() == 64 && s.sha256.chars().all(|c| c.is_ascii_hexdigit()), "{}", s.url);
        }
        for h in [EZA_SHA256_X86_64, EZA_SHA256_AARCH64] {
            assert!(h.len() == 64 && h.chars().all(|c| c.is_ascii_hexdigit()));
        }
    }

    /// Online: `cargo test -- --ignored` downloads every pinned script and checks its hash.
    /// `true` as the interpreter means nothing is executed.
    #[test]
    #[ignore]
    fn pinned_scripts_match_upstream() {
        for s in [&NVM, &HELM, &RUSTUP, &STARSHIP, &OH_MY_ZSH, &RBENV, &DOCKER] {
            crate::cmd::run_remote(s.url, s.sha256, "true", "").unwrap();
        }
        let bad = crate::cmd::run_remote(NVM.url, &"0".repeat(64), "true", "").unwrap_err();
        assert!(bad.to_string().contains("checksum mismatch"), "{bad}");
    }
}
