# Changelog

All notable changes to forge are documented here.

Format: [Conventional Commits](https://www.conventionalcommits.org). Versions follow [Semantic Versioning](https://semver.org).

---

## [0.2.0] — 2026-10-24

### Added

- Live spinner with elapsed time for package-index update, install and uninstall; command output is captured so it no longer garbles the spinner, and failures show the last lines of stderr
- sudo password asked once up front and kept warm during long installs
- Confirmation prompt for `forge uninstall --all`
- CI workflow (clippy + tests on Linux and macOS) and unit tests for distro detection, version parsing and the command runner

### Fixed

- `forge self-uninstall` prompt not shown before reading input; now also works when the binary is root-owned
- Panic in version parsing on multi-byte UTF-8 output
- Package-index update errors silently ignored; dnf/yum now only treat exit code 100 as success
- Arch: no more `pacman -Sy` partial upgrades; forge asks before running a full `pacman -Syu` (skipped with `--all`), installs use `-S --needed`
- Installer scripts (nvm, rustup, Helm, Starship, Oh My Zsh, rbenv, Docker) pinned to a tag/commit and verified by SHA-256 before running (`src/tools/pins.rs`); Go, Gradle, Bun and eza downloads verified against official/pinned checksums; shell steps run with `pipefail`
- Gradle latest-version lookup (JSON format changed), kubectl version (`--short` removed), Ruby latest-version lookup and missing build deps
- bat detected as `batcat` on Debian/Ubuntu; eza from apt when available
- Bun uninstall no longer deletes every rc line containing "bun"; Go/Flutter PATH lines added once and removed on uninstall
- VS Code apt repo hardcoded to amd64; MongoDB repo on Debian; gcloud/MongoDB rpm repos hardcoded to x86_64
- RPM repo setup works on dnf5; Docker on Arch/openSUSE uses distro packages
- macOS: JDK and Xcode CLI Tools no longer always reported installed; Xcode uninstall no longer able to delete inside Xcode.app; keg-only libpq / mysql-client / openjdk linked
- Refuse to uninstall zsh while it is the login shell; Oh My Zsh uninstall restores the previous `.zshrc`
- Unrecognised distro keeps its name instead of showing blank
- Clear error on platforms without a supported package manager (Windows: use WSL)

### Changed

- Dependencies: dialoguer 0.12, indicatif 0.18, colored 3, which 8, clap 4.6; actions/checkout v5
- nvm v0.40.8, Kubernetes repo v1.37; React Native bundle installs eas-cli instead of deprecated expo-cli; PHP installs composer
- Android platform tools use current `adb`/`fastboot` apt packages; MySQL client package names fixed for Debian, Arch and openSUSE
- 22 copies of `run_sh` replaced by one shared runner (`src/cmd.rs`)

---

## [0.1.0] — 2026-06-22

### Added

- Interactive multi-select install menu (`dialoguer` MultiSelect)
- OS and distro detection: Ubuntu, Debian, Fedora, RHEL, CentOS, Arch, Manjaro, openSUSE, macOS, WSL
- Package manager abstraction: apt, dnf, yum, pacman, zypper, brew
- 24 tools across 7 categories — all from official sources:
  - **Core**: git, curl + wget, make + vim
  - **Editors**: VS Code
  - **Version Control**: gh (GitHub CLI)
  - **Languages**: Node.js (nvm), Bun, Python 3, OpenJDK 21, Go, Rust (rustup), Ruby (rbenv), PHP
  - **DevOps & Cloud**: Docker + Compose, AWS CLI v2, gcloud SDK, Terraform
  - **CLI Tools**: fzf + ripgrep + bat + eza, jq + tree + htop, HTTPie, tmux
  - **Shell**: zsh, Oh My Zsh, Starship
- `forge list` — all tools with install status and version
- `forge check` — installed vs missing summary
- `forge uninstall <tool>` — remove specific tools
- `forge uninstall --all` — remove all managed tools
- `forge self-uninstall` — remove forge binary
- `forge --all` flag — install everything without prompting
- `forge --skip <tools>` flag — exclude specific tools
- Colored ASCII forge logo on startup
- Per-tool progress spinners (`indicatif`)
- Install summary table with status and version
- `install.sh` bootstrap — detects platform, downloads correct binary from GitHub Releases
- GitHub Actions release workflow — cross-compiles 4 targets on tag push
