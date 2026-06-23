<p align="center">
  <img src="assets/forge.png" alt="forge logo" width="180" />
</p>

<h1 align="center">forge</h1>

<p align="center">
  Interactive developer environment setup CLI.<br/>
  Detects your OS and distro, shows a checklist of tools,<br/>
  installs only what you select — all from official sources.
</p>

<p align="center">
  <img src="https://img.shields.io/badge/version-0.1.0-orange" alt="version" />
  <img src="https://img.shields.io/badge/built%20with-Rust-orange?logo=rust" alt="rust" />
  <img src="https://img.shields.io/badge/license-MIT-blue" alt="license" />
</p>

---

## Install

```sh
curl -fsSL https://raw.githubusercontent.com/n1th1n-19/Forge/master/install.sh | bash
```

Then run:

```sh
forge
```

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
| macOS | Homebrew (auto-installed if missing) |
| WSL | inherits Linux distro above |

> Windows (Git Bash / native): not supported — use WSL.

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
| 7 | Bun | Languages | [bun.sh](https://bun.sh) |
| 8 | Python 3 | Languages | distro package |
| 9 | OpenJDK 21 | Languages | distro package |
| 10 | Go | Languages | [go.dev](https://go.dev/dl) |
| 11 | Rust | Languages | [rustup.rs](https://rustup.rs) |
| 12 | Ruby | Languages | [rbenv](https://github.com/rbenv/rbenv) |
| 13 | PHP | Languages | distro package |
| 14 | Docker + Compose | DevOps | [get.docker.com](https://get.docker.com) |
| 15 | AWS CLI v2 | DevOps | [awscli.amazonaws.com](https://awscli.amazonaws.com) |
| 16 | gcloud SDK | DevOps | [packages.cloud.google.com](https://cloud.google.com/sdk) |
| 17 | Terraform | DevOps | [releases.hashicorp.com](https://releases.hashicorp.com) |
| 18 | fzf + ripgrep + bat + eza | CLI Tools | GitHub releases |
| 19 | jq + tree + htop | CLI Tools | distro package |
| 20 | HTTPie | CLI Tools | [httpie.io](https://httpie.io) |
| 21 | tmux | CLI Tools | distro package |
| 22 | zsh | Shell | distro package |
| 23 | Oh My Zsh | Shell | [ohmyz.sh](https://ohmyz.sh) |
| 24 | Starship | Shell | [starship.rs](https://starship.rs) |
| 25 | Android Studio | Mobile | [developer.android.com/studio](https://developer.android.com/studio) |
| 26 | Android Platform Tools | Mobile | [developer.android.com](https://developer.android.com/studio/releases/platform-tools) |
| 27 | Flutter + Dart | Mobile | [flutter.dev](https://flutter.dev) |
| 28 | React Native + Expo | Mobile | [reactnative.dev](https://reactnative.dev) |
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
forge uninstall --all          # remove all managed tools

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

## Build from Source

Requires Rust 1.70+.

```sh
git clone https://github.com/n1th1n-19/Forge
cd forge
cargo build --release
sudo mv target/release/forge /usr/local/bin/forge
```

---

## Releases

Pre-compiled binaries published automatically on each tagged release via GitHub Actions.

| Target | Binary |
|--------|--------|
| Linux x86\_64 | `forge-x86_64-unknown-linux-gnu` |
| Linux aarch64 | `forge-aarch64-unknown-linux-gnu` |
| macOS x86\_64 | `forge-x86_64-apple-darwin` |
| macOS Apple Silicon | `forge-aarch64-apple-darwin` |

---

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

MIT
