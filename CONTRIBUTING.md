# Contributing to forge

## Adding a New Tool

Each tool is a Rust struct implementing the `Tool` trait. Adding one takes ~30 lines.

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

    fn install(&self, pm: &PackageManager) -> Result<()> {
        pm.install(&["mytool"])  // or use run_sh() for custom installers
    }

    fn uninstall(&self, pm: &PackageManager) -> Result<()> {
        pm.remove(&["mytool"])
    }
}
```

For tools that differ by platform:

```rust
fn install(&self, pm: &PackageManager) -> Result<()> {
    match pm {
        PackageManager::Brew => pm.install(&["mytool"]),
        PackageManager::Apt  => pm.install(&["mytool-apt-name"]),
        _                    => run_sh("curl -fsSL https://mytool.dev/install.sh | sh"),
    }
}
```

### 2. Register in `src/tools/mod.rs`

Add the module declaration:

```rust
pub mod mytool;
```

Add to `all_tools()` in the correct position (tools display in registry order):

```rust
Box::new(mytool::MyTool),
```

### 3. Add to README tool table

Add a row to the Tools table in `README.md` with the official source URL.

### 4. Build and verify

```sh
cargo build
./target/debug/forge list   # confirm tool appears
./target/debug/forge check  # confirm detection works
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

## Package Manager Abstraction

Use `pm.install(&[...])` and `pm.remove(&[...])` whenever possible — they map to the right command on every supported platform automatically. Only call `run_sh()` when a tool requires a custom installer script (e.g. nvm, rustup, bun).

`pm.install_cask(&[...])` is for macOS GUI apps that go through `brew install --cask`.

## Source Policy

All tools must install from **official or verified sources only**:

- Distro package managers (apt, dnf, pacman, etc.)
- Official project install scripts (must be HTTPS)
- Official Homebrew formulae/casks
- Official GitHub releases from the tool's own org

No third-party mirrors, PPAs from unknown maintainers, or unofficial package names.

## Commit Style

Conventional Commits. Subject ≤ 50 chars, imperative mood.

```
feat(tools): add <toolname> to <category>
fix(tools/<name>): fix uninstall on Arch
```
