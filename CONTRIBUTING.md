# Contributing to forge

## Adding a New Tool

Each tool is a Rust struct implementing the `Tool` trait. A simple one is ~15 lines.

### 1. Create `src/tools/<toolname>.rs`

```rust
use super::{version_of, Category, Tool};
use crate::package_manager::PackageManager;
use anyhow::Result;

pub struct MyTool;

impl Tool for MyTool {
    fn name(&self) -> &str { "mytool" }
    fn description(&self) -> &str { "what it does (source URL)" }
    fn category(&self) -> Category { Category::CliTools }  // pick the right category
    fn is_installed(&self) -> bool { which::which("mytool").is_ok() }
    fn version(&self) -> Option<String> { version_of("mytool", &["--version"]) }
    fn install(&self, pm: &PackageManager) -> Result<()> { pm.install(&["mytool"]) }
    fn uninstall(&self, pm: &PackageManager) -> Result<()> { pm.remove(&["mytool"]) }
}
```

For tools whose package name differs by platform, use one `pkg(pm)` helper so install and
uninstall can't drift apart:

```rust
fn install(&self, pm: &PackageManager) -> Result<()> { pm.install(&[pkg(pm)]) }
fn uninstall(&self, pm: &PackageManager) -> Result<()> { pm.remove(&[pkg(pm)]) }

fn pkg(pm: &PackageManager) -> &'static str {
    match pm {
        PackageManager::Apt => "mytool-apt-name",
        _ => "mytool",
    }
}
```

Optional trait methods:

| Method | Use when |
|--------|----------|
| `protected() -> true` | The OS may depend on it (git, curl, python3). `uninstall --all` skips it. |
| `post_install_note(pm)` | The user must know something afterwards (e.g. docker group = root access). |

### 2. Register in `src/tools/mod.rs`

```rust
pub mod mytool;            // module declaration
Box::new(mytool::MyTool),  // in all_tools(), in category order — that's the display order
```

### 3. Add to README tool table

Add a row to the Tools table in `README.md` with the official source.

### 4. Build and verify

```sh
cargo clippy --all-targets -- -D warnings
cargo test
./target/debug/forge list   # confirm the tool appears
./target/debug/forge check  # confirm detection works
```

## Running Commands

Never use `std::process::Command` for installs, and never write your own `run_sh`. Use the shared
helpers. They capture output so the spinner stays clean, run without stdin, and report the tail of
stderr on failure.

| Helper | For |
|--------|-----|
| `pm.install(&[..])` / `pm.remove(&[..])` | Packages. Maps to apt/dnf/yum/pacman/zypper/brew. Prefer this. |
| `pm.install_cask(&[..])` | macOS GUI apps (`brew install --cask`). |
| `sh("...")` | A bash snippet. Runs with `set -eo pipefail` and refuses an empty/relative/`/` `HOME`. |
| `run_remote(url, sha256, interp, args)` | Running a vendor installer script. Downloads to a temp file and checks the SHA-256 first. |
| `add_rc_line(line)` / `remove_rc_lines(pred)` | Editing `~/.profile`, `~/.bash_profile`, `~/.bashrc`, `~/.zshrc`. |

Rules for `sh` snippets:
- **Temp files:** use `mktemp -d` plus `trap 'rm -rf "$dir"' EXIT`. Never a fixed `/tmp/...` path.
- **Downloads:** check them with `verify_sha256 FILE HASH`. It's available in every snippet.
- **Network values:** validate anything fetched from the network (e.g. a "latest version" string)
  before it reaches a URL or a `sudo` path, e.g. `[[ $V =~ ^[0-9]+(\.[0-9]+)*$ ]] || exit 1`.
- **Interpolation:** only put constants or validated values into `format!`ed scripts.
- **apt keys:** pipe them straight into `sudo gpg --batch --yes --dearmor -o /etc/apt/keyrings/...`.
- **Uninstall:** only delete what forge installed. Check the path first (see `node.rs`, `go.rs`,
  `flutter.rs`) and `bail!` otherwise.
- **rc files:** remove only exact lines forge or the installer wrote, never substrings.

## Pinned Installers

Vendor install scripts and binaries live in [`src/tools/pins.rs`](src/tools/pins.rs) as URL + SHA-256,
pinned to a tag or commit, never a branch.

To add or bump one:

```sh
curl -fsSL <url-at-tag-or-commit> | sha256sum   # paste the hash into pins.rs
cargo test -- --ignored                          # downloads every pin and checks its hash
```

## Categories

| Category | Use for |
|----------|---------|
| `Category::Core` | Essential CLI tools (git, curl, make) |
| `Category::Editors` | Code editors and IDEs |
| `Category::VersionControl` | VCS hosting CLIs (gh, gitlab) |
| `Category::Languages` | Language runtimes and version managers |
| `Category::DevOps` | Containers, cloud CLIs, IaC |
| `Category::CliTools` | Productivity CLI utilities |
| `Category::Shell` | Shell environments and enhancements |
| `Category::Mobile` | Mobile SDKs and tooling |
| `Category::Databases` | Database clients |
| `Category::Build` | Build systems, Kubernetes, container runtimes |

## Source Policy

All tools must install from **official or verified sources only**:

- Distro package managers (apt, dnf, pacman, etc.)
- Official vendor apt/yum repositories with a `signed-by` key
- Official project install scripts — pinned and hashed in `pins.rs`
- Official Homebrew formulae/casks
- Official GitHub releases from the tool's own org, checksum-verified

No third-party mirrors, PPAs from unknown maintainers, or unofficial package names.

## Releasing

1. Bump `version` in `Cargo.toml` and add a `CHANGELOG.md` section.
2. Merge to `master` with CI green (clippy, tests, `cargo audit`).
3. Tag and push: `git tag -a vX.Y.Z -m "forge vX.Y.Z" && git push origin vX.Y.Z`.

The release workflow builds 4 targets, writes `SHA256SUMS`, and publishes them. `install.sh` refuses
any release without `SHA256SUMS`.

## Commit Style

Conventional Commits. Subject ≤ 50 chars, imperative mood.

```
feat(tools): add <toolname> to <category>
fix(tools/<name>): fix uninstall on Arch
```
