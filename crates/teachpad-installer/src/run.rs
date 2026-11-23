//! The steps of a build, from the command line to the exit code.

use std::io;
use std::path::PathBuf;
use std::process::{Command, ExitCode, Stdio};

use anyhow::{Context as _, bail};

use crate::args::{self, Request, USAGE};
use crate::collect;
use crate::platform::{self, Platform};
use crate::tauri::{self, CommandSpec, Environment, Finished};

/// The exit code of a command line this tool cannot read.
const USAGE_ERROR: u8 = 2;

/// Runs `cargo installer` with the arguments of this process and returns
/// its exit code.
///
/// It builds the installers of one target and prints the path of each. A
/// failure is printed to the standard error with its chain of causes.
///
/// # Examples
///
/// ```no_run
/// fn main() -> std::process::ExitCode {
///     teachpad_installer::run()
/// }
/// ```
#[must_use]
pub fn run() -> ExitCode {
    let request = match args::parse(std::env::args_os().skip(1)) {
        Ok(request) => request,
        Err(message) => {
            eprintln!("error: {message}\n\n{USAGE}");
            return ExitCode::from(USAGE_ERROR);
        }
    };
    let target = match request {
        Request::Help => {
            print!("{USAGE}");
            return ExitCode::SUCCESS;
        }
        Request::Build { target } => target,
    };
    let built = Environment::discover()
        .and_then(|environment| build(target.as_deref(), &environment, &mut spawn));
    match built {
        Ok(installers) => {
            for installer in installers {
                println!("installer: {}", installer.display());
            }
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("error: {error:#}");
            ExitCode::FAILURE
        }
    }
}

/// Builds the installers of one target and returns their paths.
///
/// Everything that can be checked first is checked before the Tauri CLI
/// starts: the target, the system it is built on, and the CLI itself. `start` starts each child
/// process, so a test can stand in for cargo and for the Tauri CLI.
fn build(
    target: Option<&str>,
    environment: &Environment,
    start: &mut impl FnMut(&CommandSpec) -> io::Result<Finished>,
) -> anyhow::Result<Vec<PathBuf>> {
    let triple = match target {
        Some(triple) => triple.to_owned(),
        None => host(environment, start)?,
    };
    let Some(platform) = Platform::from_triple(&triple) else {
        bail!(
            "the target `{triple}` is not supported; the supported targets are {}",
            platform::supported_triples()
        );
    };
    if environment.system != Some(platform.system) {
        bail!(
            "the installers for `{triple}` are built on {}; run `cargo installer` there",
            platform.system.name()
        );
    }
    let cli = environment.tauri_cli();
    if !cli.is_file() {
        bail!(
            "the Tauri CLI is missing at {}; run `npm ci` in {} first",
            cli.display(),
            environment.ui_dir().display()
        );
    }
    collect::clear(environment, platform)?;
    let build = tauri::build_command(environment, platform);
    let finished = start(&build).with_context(|| {
        format!(
            "start the Tauri CLI with `{}`, which has to be on PATH",
            build.program.display()
        )
    })?;
    if !finished.success {
        bail!("the Tauri build for `{triple}` failed");
    }
    collect::collect(environment, platform)
}

/// Asks cargo which target this machine builds for.
fn host(
    environment: &Environment,
    start: &mut impl FnMut(&CommandSpec) -> io::Result<Finished>,
) -> anyhow::Result<String> {
    let question = tauri::host_command(environment);
    let asked = format!("`{} -vV`", question.program.display());
    let finished =
        start(&question).with_context(|| format!("start {asked} to find the host target"))?;
    if !finished.success {
        bail!("{asked} failed, so the host target is unknown; name one with `--target`");
    }
    platform::host_triple(&finished.stdout)
        .map(str::to_owned)
        .with_context(|| format!("{asked} printed no `host:` line; name a target with `--target`"))
}

/// Starts a child process and waits for it to end.
fn spawn(command: &CommandSpec) -> io::Result<Finished> {
    let mut child = Command::new(&command.program);
    child.args(&command.args).current_dir(&command.current_dir);
    for (name, value) in &command.envs {
        child.env(name, value);
    }
    if command.capture {
        // The child's errors still go straight to the terminal.
        let output = child.stderr(Stdio::inherit()).output()?;
        Ok(Finished {
            success: output.status.success(),
            stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
        })
    } else {
        let status = child.status()?;
        Ok(Finished {
            success: status.success(),
            stdout: String::new(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs;
    use std::path::Path;

    use crate::platform::{Format, System, VERSION};

    /// A repository in a temp folder on a machine that runs `system`, as
    /// far as a build looks at one: the installed Tauri CLI and the desktop
    /// crate's folder.
    fn repository(temp: &tempfile::TempDir, system: System) -> Environment {
        let environment = Environment {
            root: temp.path().join("repo"),
            target_dir: temp.path().join("repo").join("target"),
            cargo: PathBuf::from("the-cargo"),
            node: PathBuf::from("the-node"),
            system: Some(system),
        };
        write(&environment.tauri_cli(), "");
        fs::create_dir_all(environment.desktop_dir()).expect("the desktop folder is made");
        environment
    }

    fn write(path: &Path, contents: &str) {
        fs::create_dir_all(path.parent().expect("the path has a folder"))
            .expect("the folder is made");
        fs::write(path, contents).expect("the file is written");
    }

    fn platform(triple: &str) -> Platform {
        Platform::from_triple(triple).expect("the triple is supported")
    }

    /// A child that reported success and printed `stdout`.
    fn done(stdout: &str) -> Finished {
        Finished {
            success: true,
            stdout: stdout.to_owned(),
        }
    }

    /// A child that reported failure.
    fn failed() -> Finished {
        Finished {
            success: false,
            stdout: String::new(),
        }
    }

    /// Stands in for the Tauri CLI: writes the file each bundler of the
    /// target would leave, under a name of Tauri's own.
    fn bundle(environment: &Environment, platform: Platform) {
        for installer in platform.installers {
            let name = format!("Built by Tauri{}", installer.format.extension());
            write(
                &environment
                    .bundle_dir(platform, installer.format)
                    .join(name),
                installer.format.bundle(),
            );
        }
    }

    #[test]
    fn a_build_clears_the_old_files_runs_the_tauri_cli_and_collects_the_installer() {
        let temp = tempfile::TempDir::new().expect("a temp folder");
        let environment = repository(&temp, System::Windows);
        let windows = platform("x86_64-pc-windows-msvc");
        let stale_output = environment
            .output_dir(windows)
            .join("TeachPad_0.0.1_x64-setup.exe");
        let stale_bundle = environment
            .bundle_dir(windows, Format::Nsis)
            .join("TeachPad_0.0.1_x64-setup.exe");
        write(&stale_output, "stale");
        write(&stale_bundle, "stale");
        let mut started = Vec::new();

        let installers = build(Some(windows.triple), &environment, &mut |command| {
            started.push(command.clone());
            // The old files are gone by the time the Tauri CLI starts.
            assert!(!stale_output.exists() && !stale_bundle.exists());
            bundle(&environment, windows);
            Ok(done(""))
        })
        .expect("the build succeeds");

        assert_eq!(started, [tauri::build_command(&environment, windows)]);
        let expected = environment
            .output_dir(windows)
            .join(format!("TeachPad_{VERSION}_x64-setup.exe"));
        assert_eq!(installers, [expected.as_path()]);
        assert_eq!(
            fs::read_to_string(expected).expect("the installer is readable"),
            "nsis"
        );
    }

    #[test]
    fn with_no_target_named_the_build_is_for_the_host_cargo_reports() {
        let temp = tempfile::TempDir::new().expect("a temp folder");
        let environment = repository(&temp, System::Linux);
        let linux = platform("x86_64-unknown-linux-gnu");
        let mut started = Vec::new();

        let installers = build(None, &environment, &mut |command| {
            started.push(command.clone());
            if command.capture {
                let report = "cargo 1.99.1\nhost: x86_64-unknown-linux-gnu\nos: Ubuntu 22.04\n";
                return Ok(done(report));
            }
            bundle(&environment, linux);
            Ok(done(""))
        })
        .expect("the build succeeds");

        assert_eq!(
            started,
            [
                tauri::host_command(&environment),
                tauri::build_command(&environment, linux),
            ],
        );
        let output = environment.output_dir(linux);
        assert_eq!(
            installers,
            [
                output.join(format!("TeachPad_{VERSION}_amd64.deb")),
                output.join(format!("TeachPad_{VERSION}_amd64.AppImage")),
            ],
        );
    }

    /// Runs a build that has to fail before or at its first child process,
    /// and returns the message with how many children were started.
    fn failure(
        target: Option<&str>,
        environment: &Environment,
        outcome: impl Fn() -> io::Result<Finished>,
    ) -> (String, usize) {
        let mut started = 0;
        let error = build(target, environment, &mut |_command| {
            started += 1;
            outcome()
        })
        .expect_err("the build fails");
        (format!("{error:#}"), started)
    }

    #[test]
    fn a_target_outside_the_list_is_refused_before_anything_starts() {
        let temp = tempfile::TempDir::new().expect("a temp folder");
        let environment = repository(&temp, System::Linux);

        let (message, started) = failure(Some("aarch64-unknown-linux-gnu"), &environment, || {
            Ok(done(""))
        });

        assert_eq!(
            message,
            "the target `aarch64-unknown-linux-gnu` is not supported; the supported targets are \
             x86_64-pc-windows-msvc, aarch64-apple-darwin, x86_64-apple-darwin, \
             x86_64-unknown-linux-gnu",
        );
        assert_eq!(started, 0);

        // A host outside the list is refused the same way, after the one
        // question to cargo.
        let (message, started) = failure(None, &environment, || {
            Ok(done("host: riscv64gc-unknown-linux-gnu\n"))
        });
        assert!(message.starts_with("the target `riscv64gc-unknown-linux-gnu` is not supported; "));
        assert_eq!(started, 1);
    }

    #[test]
    fn a_target_of_another_system_is_refused_before_anything_starts() {
        let temp = tempfile::TempDir::new().expect("a temp folder");
        let environment = repository(&temp, System::Windows);
        let earlier = environment
            .output_dir(platform("x86_64-apple-darwin"))
            .join("TeachPad_0.0.1_x64.dmg");
        write(&earlier, "from an earlier build");

        for (triple, expected) in [
            (
                "x86_64-apple-darwin",
                "the installers for `x86_64-apple-darwin` are built on macOS; run `cargo \
                 installer` there",
            ),
            (
                "x86_64-unknown-linux-gnu",
                "the installers for `x86_64-unknown-linux-gnu` are built on Linux; run `cargo \
                 installer` there",
            ),
        ] {
            let (message, started) = failure(Some(triple), &environment, || Ok(done("")));
            assert_eq!(message, expected);
            assert_eq!(started, 0);
        }
        assert!(earlier.is_file(), "nothing is cleared for a refused target");

        // A machine that runs none of the three systems builds no target.
        let elsewhere = Environment {
            system: None,
            ..environment
        };
        let (message, started) =
            failure(Some("x86_64-pc-windows-msvc"), &elsewhere, || Ok(done("")));
        assert_eq!(
            message,
            "the installers for `x86_64-pc-windows-msvc` are built on Windows; run `cargo \
             installer` there",
        );
        assert_eq!(started, 0);
    }

    #[test]
    fn a_missing_tauri_cli_names_the_fix_and_leaves_everything_as_it_was() {
        let temp = tempfile::TempDir::new().expect("a temp folder");
        let environment = repository(&temp, System::Windows);
        let windows = platform("x86_64-pc-windows-msvc");
        fs::remove_file(environment.tauri_cli()).expect("the CLI is removed");
        let earlier = environment
            .output_dir(windows)
            .join("TeachPad_0.0.1_x64-setup.exe");
        write(&earlier, "from an earlier build");

        let (message, started) = failure(Some(windows.triple), &environment, || Ok(done("")));

        assert_eq!(
            message,
            format!(
                "the Tauri CLI is missing at {}; run `npm ci` in {} first",
                environment.tauri_cli().display(),
                environment.ui_dir().display()
            ),
        );
        assert_eq!(started, 0);
        assert!(
            earlier.is_file(),
            "nothing is cleared before the checks pass"
        );
    }

    #[test]
    fn a_tauri_build_that_fails_or_cannot_start_is_an_error() {
        let temp = tempfile::TempDir::new().expect("a temp folder");
        let environment = repository(&temp, System::MacOs);
        let mac = "aarch64-apple-darwin";

        let (message, started) = failure(Some(mac), &environment, || Ok(failed()));
        assert_eq!(message, "the Tauri build for `aarch64-apple-darwin` failed");
        assert_eq!(started, 1);

        let (message, _) = failure(Some(mac), &environment, || {
            Err(io::Error::new(io::ErrorKind::NotFound, "program not found"))
        });
        assert_eq!(
            message,
            "start the Tauri CLI with `the-node`, which has to be on PATH: program not found",
        );

        // A build that reports success and leaves no installer is caught by
        // the collection.
        let (message, _) = failure(Some(mac), &environment, || Ok(done("")));
        assert!(message.starts_with("read the bundle folder "), "{message}");
    }

    #[test]
    fn a_host_question_that_fails_asks_for_a_target() {
        let temp = tempfile::TempDir::new().expect("a temp folder");
        let environment = repository(&temp, System::Linux);

        let (message, _) = failure(None, &environment, || Ok(failed()));
        assert_eq!(
            message,
            "`the-cargo -vV` failed, so the host target is unknown; name one with `--target`",
        );

        let (message, _) = failure(None, &environment, || Ok(done("cargo 1.99.1\n")));
        assert_eq!(
            message,
            "`the-cargo -vV` printed no `host:` line; name a target with `--target`",
        );

        let (message, _) = failure(None, &environment, || {
            Err(io::Error::new(io::ErrorKind::NotFound, "program not found"))
        });
        assert_eq!(
            message,
            "start `the-cargo -vV` to find the host target: program not found",
        );
    }

    #[test]
    fn a_real_child_process_reports_its_success_and_what_it_printed() {
        // Cargo names itself in `CARGO` while it runs the tests, and it is
        // the one program this test can count on.
        let environment = Environment::discover().expect("the environment is readable");
        let finished = spawn(&tauri::host_command(&environment)).expect("cargo starts");
        assert!(finished.success);
        let host = platform::host_triple(&finished.stdout).expect("cargo reports its host");
        assert!(host.contains('-'), "{host}");

        let missing = CommandSpec {
            program: PathBuf::from("no-such-program-for-the-teachpad-installer"),
            ..tauri::host_command(&environment)
        };
        assert!(spawn(&missing).is_err());
    }
}
