//! The command line of the built binary.

// clippy.toml's allow-expect-in-tests covers #[test] functions and
// #[cfg(test)] modules only, so the helper here asks for it itself.
#![expect(
    clippy::expect_used,
    reason = "test helpers fail by panicking with the invariant named"
)]

use std::process::{Command, Output};

/// Runs the installer binary with the given arguments.
fn installer(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_teachpad-installer"))
        .args(args)
        .output()
        .expect("the installer binary starts")
}

fn text(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).into_owned()
}

#[test]
fn help_prints_the_usage_and_succeeds() {
    for flag in ["--help", "-h"] {
        let output = installer(&[flag]);

        assert!(output.status.success(), "{flag}");
        let stdout = text(&output.stdout);
        assert!(
            stdout.contains("USAGE:\n    cargo installer [--target <triple>]"),
            "{stdout}"
        );
        assert_eq!(text(&output.stderr), "");
    }
}

#[test]
fn an_unknown_flag_is_a_usage_error() {
    let output = installer(&["--frobnicate"]);

    assert_eq!(output.status.code(), Some(2));
    let stderr = text(&output.stderr);
    assert!(
        stderr.starts_with("error: unknown argument `--frobnicate`\n"),
        "{stderr}"
    );
    assert!(stderr.contains("USAGE:"), "{stderr}");
    assert_eq!(text(&output.stdout), "");
}

#[test]
fn an_unsupported_target_is_refused_before_a_build_starts() {
    let output = installer(&["--target", "riscv64gc-unknown-linux-gnu"]);

    assert_eq!(output.status.code(), Some(1));
    assert_eq!(
        text(&output.stderr).trim_end(),
        "error: the target `riscv64gc-unknown-linux-gnu` is not supported; the supported targets \
         are x86_64-pc-windows-msvc, aarch64-apple-darwin, x86_64-apple-darwin, \
         x86_64-unknown-linux-gnu",
    );
    assert_eq!(text(&output.stdout), "");
}
