<p align="center">
  <img src="assets/forge.png" alt="forge logo" width="180" />
</p>

<h1 align="center">forge</h1>

<p align="center">
  Interactive developer environment setup CLI.<br/>
  Detects your OS and distro, shows a checklist of tools,<br/>
  installs only what you select — from official sources, checksum-verified.
</p>

<p align="center">
  <img src="https://img.shields.io/badge/version-0.3.0-orange" alt="version" />
  <img src="https://img.shields.io/badge/built%20with-Rust-orange?logo=rust" alt="rust" />
  <img src="https://img.shields.io/badge/license-MIT-blue" alt="license" />
</p>

---

## Install

```sh
curl -fsSL https://raw.githubusercontent.com/n1th1n-19/Forge/master/install.sh | bash
```

The script picks the right binary for your OS/arch, verifies it against the release's
`SHA256SUMS`, and installs it to `/usr/local/bin` (using `sudo` if needed).

Then run:

```sh
forge
```

forge asks for your `sudo` password once, up front, and drops the cached credential when it exits.
Run it as your normal user — not with `sudo`, or user-level tools land in root's home.

---

## Preview

<p align="center">
  <img src="assets/forge-ascii.png" alt="forge terminal preview" width="480" />
</p>

---

## Supported Platforms

| Platform | Package Manager |
|----------|----------------|
| Ubuntu / Debian / Mint | apt |
| Fedora | dnf |
| RHEL / CentOS | dnf / yum |
| Arch / Manjaro | pacman |
| openSUSE | zypper |
| macOS | Homebrew (install it first from [brew.sh](https://brew.sh)) |
| WSL | inherits Linux distro above |

> Windows (Git Bash / native): not supported — use WSL.
>
> Arch / Manjaro: Arch only supports refreshing package lists together with a full upgrade, so forge
> asks before running `pacman -Syu` (skipped under `--all`).

---

## Tools

| # | Tool | Category | Source |
|---|------|----------|--------|
| 1 | git | Core | distro package |
| 2 | curl + wget | Core | distro package |
| 3 | make + vim | Core | distro package |
| 4 | VS Code | Editors | [packages.microsoft.com](https://packages.microsoft.com) |
| 5 | gh | Version Control | [cli.github.com](https://cli.github.com) |
| 6 | Node.js | Languages | [nvm.sh](https://github.com/nvm-sh/nvm) |
| 7 | Bun | Languages | [bun.sh](https://bun.sh) — GitHub release, checksum-verified |
| 8 | Python 3 | Languages | distro package |
| 9 | OpenJDK 21 | Languages | distro package |
| 10 | Go | Languages | [go.dev](https://go.dev/dl) |
| 11 | Rust | Languages | [rustup.rs](https://rustup.rs) |
| 12 | Ruby | Languages | [rbenv](https://github.com/rbenv/rbenv) |
| 13 | PHP + Composer | Languages | distro package |
| 14 | Docker + Compose | DevOps | [get.docker.com](https://get.docker.com) (distro packages on Arch / openSUSE) |
| 15 | AWS CLI v2 | DevOps | [awscli.amazonaws.com](https://awscli.amazonaws.com) |
| 16 | gcloud SDK | DevOps | [packages.cloud.google.com](https://cloud.google.com/sdk) |
| 17 | Terraform | DevOps | [releases.hashicorp.com](https://releases.hashicorp.com) |
| 18 | fzf + ripgrep + bat + eza | CLI Tools | distro package (eza from GitHub release on older Debian/Ubuntu) |
| 19 | jq + tree + htop | CLI Tools | distro package |
| 20 | HTTPie | CLI Tools | distro package |
| 21 | tmux | CLI Tools | distro package |
| 22 | zsh | Shell | distro package |
| 23 | Oh My Zsh | Shell | [ohmyz.sh](https://ohmyz.sh) |
| 24 | Starship | Shell | [starship.rs](https://starship.rs) |
| 25 | Android Studio | Mobile | [developer.android.com/studio](https://developer.android.com/studio) |
| 26 | Android Platform Tools | Mobile | [developer.android.com](https://developer.android.com/studio/releases/platform-tools) |
| 27 | Flutter + Dart | Mobile | [flutter.dev](https://flutter.dev) — snap, or `stable` git clone |
| 28 | React Native + Expo | Mobile | npm — `@react-native-community/cli` + `eas-cli` (needs Node) |
| 29 | Xcode CLI Tools | Mobile | macOS only — `xcode-select --install` |
| 30 | CocoaPods | Mobile | macOS only — [cocoapods.org](https://cocoapods.org) |
| 31 | PostgreSQL client | Databases | [postgresql.org](https://www.postgresql.org) |
| 32 | MySQL client | Databases | [dev.mysql.com](https://dev.mysql.com) |
| 33 | mongosh | Databases | [mongodb.com](https://www.mongodb.com/try/download/shell) |
| 34 | redis-cli | Databases | [redis.io](https://redis.io) |
| 35 | Maven | Build & Infra | [maven.apache.org](https://maven.apache.org) |
| 36 | Gradle | Build & Infra | [gradle.org](https://gradle.org) |
| 37 | CMake + Ninja | Build & Infra | [cmake.org](https://cmake.org) |
| 38 | kubectl + Helm | Build & Infra | [kubernetes.io](https://kubernetes.io/docs/tasks/tools/) |
| 39 | Podman | Build & Infra | [podman.io](https://podman.io) |

---

## Usage

```sh
forge                          # interactive install menu
forge --all                    # install everything, no prompts
forge --skip go,docker,php     # install all except listed tools

forge list                     # show all tools with install status
forge check                    # installed vs missing with versions

forge uninstall go docker      # remove specific tools by name
forge uninstall --all          # remove all managed tools (asks first; skips git, curl,
                               # make/vim, python3 — name those explicitly)

forge self-uninstall           # remove the forge binary itself
```

### Examples

```sh
# Fresh machine — pick what you need interactively
forge

# CI / dotfiles bootstrap — install everything silently
forge --all

# Set up frontend tooling only
forge --skip go,rust,ruby,php,docker,aws,gcloud,terraform

# Check what's installed
forge check

# Remove tools you no longer need
forge uninstall gcloud terraform php
```

---

## Security

forge runs installers with `sudo`, so it is deliberately conservative:

- **Pinned installer scripts.** nvm, rustup, Helm, Starship, Oh My Zsh, rbenv and Docker's
  installer are pinned to a tag/commit and checked against a SHA-256 before running
  ([`src/tools/pins.rs`](src/tools/pins.rs)). They are downloaded to a temp file, never piped into a shell.
- **Verified downloads.** The AWS CLI and eza binaries are pinned by SHA-256; Go, Gradle and Bun are
  checked against their official checksum files.
- **Safe uninstall.**
  - apt removals are simulated first and refused if they would take other packages with them.
  - forge only deletes what it installed (e.g. `~/.nvm`, `/usr/local/go`, its own `/opt/gradle-<ver>`).
  - Base-system tools are never removed by `--all`.
  - zsh can't be removed while it's your login shell.
- **Dotfile edits.** PATH lines are added once and removed by exact match. Files are written atomically
  and keep their line endings and symlinks (stow / chezmoi friendly).
- **Release pipeline.** Release binaries come from a workflow with SHA-pinned actions and `--locked` builds.
  Only the publish job can write to releases. CI runs `cargo audit`.

Known limits:
- Bun, Go and Gradle checksums come from the same origin as the download.
- Third-party apt/yum repo keys are fetched over HTTPS without fingerprint pinning.
- Docker adds you to the `docker` group, which is root-equivalent (forge warns).

Found a vulnerability? Please open a private
[security advisory](https://github.com/n1th1n-19/Forge/security/advisories/new) instead of a public issue.

---

## Build from Source

Requires Rust 1.88+.

```sh
git clone https://github.com/n1th1n-19/Forge
cd Forge
cargo build --release --locked
sudo install -m 755 target/release/forge /usr/local/bin/forge
```

---

## Releases

Pre-compiled binaries are published automatically on each `v*` tag via GitHub Actions, together with a
`SHA256SUMS` file.

| Target | Binary |
|--------|--------|
| Linux x86\_64 | `forge-x86_64-unknown-linux-gnu` |
| Linux aarch64 | `forge-aarch64-unknown-linux-gnu` |
| macOS x86\_64 | `forge-x86_64-apple-darwin` |
| macOS Apple Silicon | `forge-aarch64-apple-darwin` |

Verify a manual download:

```sh
curl -fsSLO https://github.com/n1th1n-19/Forge/releases/latest/download/SHA256SUMS
grep " forge-x86_64-unknown-linux-gnu$" SHA256SUMS | sha256sum -c -   # macOS: shasum -a 256 -c -
```

---

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

MIT
