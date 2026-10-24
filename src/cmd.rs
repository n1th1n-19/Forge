use anyhow::{bail, Result};
use std::process::{Command, Output, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

static SUDO_WARM: AtomicBool = AtomicBool::new(false);

/// Run a program with captured output so spinners stay clean.
/// Fails with the tail of stderr/stdout when the exit code is not in `ok_codes`.
pub fn run_codes(args: &[&str], ok_codes: &[i32]) -> Result<()> {
    let (prog, rest) = args.split_first().expect("empty command");
    let out = Command::new(prog)
        .args(rest)
        .env("NONINTERACTIVE", "1")
        .stdin(Stdio::null())
        .output()?;
    check(&out, &args.join(" "), ok_codes)
}

pub fn run(args: &[&str]) -> Result<()> {
    run_codes(args, &[0])
}

/// Run a program and return its stdout (fails like `run`).
pub fn output(args: &[&str]) -> Result<String> {
    let (prog, rest) = args.split_first().expect("empty command");
    let out = Command::new(prog).args(rest).stdin(Stdio::null()).output()?;
    check(&out, &args.join(" "), &[0])?;
    Ok(String::from_utf8_lossy(&out.stdout).into_owned())
}

/// Run a bash snippet with `set -eo pipefail` so a failed step (e.g. curl in a pipe) fails the whole thing.
pub fn sh(script: &str) -> Result<()> {
    let out = Command::new("bash")
        .arg("-c")
        .arg(format!("set -eo pipefail\n{SHELL_LIB}\n{script}"))
        .env("NONINTERACTIVE", "1")
        .stdin(Stdio::null())
        .output()?;
    check(&out, script.trim().lines().next().unwrap_or(""), &[0])
}

/// Prelude for every `sh` snippet:
/// - refuse to run with an empty/relative/"/" HOME, so `rm -rf "$HOME/.x"` can't hit `/.x` or the cwd
/// - `verify_sha256 FILE HASH` fails loudly on mismatch (sha256sum on Linux, shasum on macOS).
///   Explicit `return 1`s keep it safe even when called where `set -e` is off (`if`, `||`, `!`).
const SHELL_LIB: &str = r#"case "${HOME:-}" in /?*) ;; *) echo "refusing to run: HOME is unset, relative or /" >&2; exit 1 ;; esac
# also catch HOME=/./ or /tmp/.. that resolve to /
[ "$(cd "$HOME" 2>/dev/null && pwd -P)" != / ] && [ -d "$HOME" ] || { echo "refusing to run: HOME resolves to / or doesn't exist" >&2; exit 1; }
verify_sha256() {
  want=$(printf '%s' "$2" | tr 'A-F' 'a-f')
  got=$( { sha256sum "$1" 2>/dev/null || shasum -a 256 "$1"; } | cut -d' ' -f1 ) || return 1
  case "$want" in *[!0-9a-f]* | "") echo "no valid checksum for $(basename "$1")" >&2; return 1 ;; esac
  [ "${#want}" -eq 64 ] || { echo "no valid checksum for $(basename "$1")" >&2; return 1; }
  [ "$got" = "$want" ] || { echo "checksum mismatch for $(basename "$1"): expected $want, got $got" >&2; return 1; }
}"#;

/// Download an installer script to a temp file, check its pinned SHA-256, then run it.
/// Never pipes a half-downloaded or modified script straight into a shell.
pub fn run_remote(url: &str, sha256: &str, interp: &str, args: &str) -> Result<()> {
    sh(&format!(
        r#"tmp=$(mktemp); trap 'rm -f "$tmp"' EXIT
curl -fsSL --proto '=https' --tlsv1.2 '{url}' -o "$tmp"
verify_sha256 "$tmp" '{sha256}'
{interp} "$tmp" {args}"#
    ))
}

fn check(out: &Output, what: &str, ok_codes: &[i32]) -> Result<()> {
    if out.status.code().is_some_and(|c| ok_codes.contains(&c)) {
        return Ok(());
    }
    bail!("{} — {}", what, tail(out))
}

fn tail(out: &Output) -> String {
    let text = if out.stderr.iter().any(|b| !b.is_ascii_whitespace()) {
        String::from_utf8_lossy(&out.stderr)
    } else {
        String::from_utf8_lossy(&out.stdout)
    };
    let lines: Vec<&str> = text.lines().map(str::trim).filter(|l| !l.is_empty()).collect();
    match lines.len() {
        0 => format!("exit {}", out.status.code().unwrap_or(-1)),
        n => lines[n.saturating_sub(3)..].join(" | "),
    }
}

/// Ask for the sudo password once, up front (spinners would hide the prompt),
/// then keep the credential cache warm while long installs run.
pub fn sudo_keepalive() -> Result<()> {
    let is_root = Command::new("id")
        .arg("-u")
        .output()
        .is_ok_and(|o| String::from_utf8_lossy(&o.stdout).trim() == "0");
    if is_root || which::which("sudo").is_err() {
        return Ok(());
    }
    if SUDO_WARM.swap(true, Ordering::SeqCst) {
        return Ok(());
    }
    if !Command::new("sudo").arg("-v").status()?.success() {
        SUDO_WARM.store(false, Ordering::SeqCst);
        bail!("sudo authentication failed");
    }
    std::thread::spawn(|| loop {
        std::thread::sleep(Duration::from_secs(60));
        let _ = Command::new("sudo")
            .args(["-n", "-v"])
            .stdin(Stdio::null())
            .stderr(Stdio::null())
            .status();
    });
    Ok(())
}

/// Drop the cached sudo credential on exit, so forge doesn't leave
/// passwordless sudo behind for the rest of the timestamp window.
pub fn sudo_reset() {
    if SUDO_WARM.load(Ordering::SeqCst) {
        let _ = Command::new("sudo").arg("-k").status();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sh_reports_stderr_tail_and_honours_pipefail() {
        let err = sh("echo noise; echo boom >&2; exit 3").unwrap_err().to_string();
        assert!(err.contains("boom"), "{err}");
        assert!(sh("false | true").is_err(), "pipefail must catch a failed pipe stage");
        assert!(sh("true").is_ok());
    }

    #[test]
    fn verify_sha256_rejects_empty_or_garbage_expected_even_without_set_e() {
        for bad in ["''", "'<html>'", "'abc'"] {
            let r = sh(&format!("f=$(mktemp); printf abc > $f; if verify_sha256 $f {bad}; then exit 0; else exit 7; fi"));
            assert!(r.is_err(), "accepted expected hash {bad}");
        }
        let upper = "BA7816BF8F01CFEA414140DE5DAE2223B00361A396177A9CB410FF61F20015AD";
        assert!(sh(&format!("f=$(mktemp); printf abc > $f; verify_sha256 $f {upper}")).is_ok());
    }

    #[test]
    fn verify_sha256_accepts_match_rejects_mismatch() {
        // sha256("abc")
        let ok = "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad";
        assert!(sh(&format!("f=$(mktemp); printf abc > $f; verify_sha256 $f {ok}")).is_ok());
        let err = sh(&format!("f=$(mktemp); printf abd > $f; verify_sha256 $f {ok}")).unwrap_err();
        assert!(err.to_string().contains("checksum mismatch"), "{err}");
    }

    #[test]
    fn run_codes_accepts_listed_exit_codes() {
        assert!(run_codes(&["sh", "-c", "exit 100"], &[0, 100]).is_ok());
        assert!(run_codes(&["sh", "-c", "exit 1"], &[0, 100]).is_err());
    }
}
