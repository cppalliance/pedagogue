//! The command line of `cargo installer`.

use std::ffi::OsString;

/// What `--help` prints, and what follows the message of a usage error.
pub(crate) const USAGE: &str = "\
Build the TeachPad installers for one target with the Tauri CLI: an NSIS
setup for Windows, a disk image for macOS, or a Debian package and an
AppImage for Linux. They land in target/installer/<triple>/ under the names
a release publishes them by, and each path is printed.

USAGE:
    cargo installer [--target <triple>]

OPTIONS:
    --target <triple>   Build for this target in place of the host. One of:
                        x86_64-pc-windows-msvc, aarch64-apple-darwin,
                        x86_64-apple-darwin, x86_64-unknown-linux-gnu
    -h, --help          Print this help

The build needs Node on PATH and one `npm ci` in ui/, which installs the
pinned Tauri CLI. The installers of a system are built on that system.
";

/// What the command line asks for.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Request {
    /// Print the usage text.
    Help,
    /// Build the installers of one target.
    Build {
        /// The target triple, when the command line names one.
        target: Option<String>,
    },
}

/// Reads the arguments that follow the program name.
///
/// A help flag anywhere on the line wins. An argument this tool does not
/// take, a flag given twice or a flag with no value comes back as a message
/// for the user.
pub(crate) fn parse(args: impl IntoIterator<Item = OsString>) -> Result<Request, String> {
    let args: Vec<OsString> = args.into_iter().collect();
    if args.iter().any(|arg| arg == "-h" || arg == "--help") {
        return Ok(Request::Help);
    }
    let mut target = None;
    let mut args = args.into_iter();
    while let Some(arg) = args.next() {
        if arg != "--target" {
            return Err(format!("unknown argument `{}`", arg.to_string_lossy()));
        }
        if target.is_some() {
            return Err("`--target` is given twice".to_owned());
        }
        let value = args
            .next()
            .and_then(|value| value.into_string().ok())
            .filter(|value| !value.is_empty() && !value.starts_with('-'))
            .ok_or("`--target` needs a target triple after it")?;
        target = Some(value);
    }
    Ok(Request::Build { target })
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::platform::PLATFORMS;

    fn parse_line(line: &[&str]) -> Result<Request, String> {
        parse(line.iter().map(OsString::from))
    }

    #[test]
    fn no_argument_builds_for_the_host() {
        assert_eq!(parse_line(&[]), Ok(Request::Build { target: None }));
    }

    #[test]
    fn a_target_is_taken_from_the_flag() {
        assert_eq!(
            parse_line(&["--target", "x86_64-apple-darwin"]),
            Ok(Request::Build {
                target: Some("x86_64-apple-darwin".to_owned())
            }),
        );
    }

    #[test]
    fn a_help_flag_anywhere_on_the_line_wins() {
        for line in [
            &["-h"][..],
            &["--help"],
            &["--target", "x86_64-apple-darwin", "--help"],
            &["--frobnicate", "-h"],
        ] {
            assert_eq!(parse_line(line), Ok(Request::Help), "{line:?}");
        }
    }

    #[test]
    fn a_line_this_tool_cannot_read_is_refused_with_its_reason() {
        let cases = [
            (&["--frobnicate"][..], "unknown argument `--frobnicate`"),
            (
                &["x86_64-apple-darwin"],
                "unknown argument `x86_64-apple-darwin`",
            ),
            (
                &["--target=x86_64-apple-darwin"],
                "unknown argument `--target=x86_64-apple-darwin`",
            ),
            (&["--target"], "`--target` needs a target triple after it"),
            (
                &["--target", ""],
                "`--target` needs a target triple after it",
            ),
            (
                &["--target", "--target"],
                "`--target` needs a target triple after it",
            ),
            (
                &[
                    "--target",
                    "x86_64-apple-darwin",
                    "--target",
                    "aarch64-apple-darwin",
                ],
                "`--target` is given twice",
            ),
        ];
        for (line, expected) in cases {
            assert_eq!(parse_line(line), Err(expected.to_owned()), "{line:?}");
        }
    }

    #[test]
    fn the_usage_text_names_every_target() {
        for platform in PLATFORMS {
            assert!(USAGE.contains(platform.triple), "{}", platform.triple);
        }
    }
}
