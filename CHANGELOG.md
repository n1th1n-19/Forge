# Changelog

All notable changes to forge are documented here.

Format: [Conventional Commits](https://www.conventionalcommits.org). Versions follow [Semantic Versioning](https://semver.org).

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
