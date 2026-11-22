//! Where a build finds its files, and the two child processes it starts:
//! the question to cargo about the host, and the Tauri CLI.

use std::ffi::OsString;
use std::path::{Path, PathBuf};

use anyhow::Context as _;

use crate::platform::{Format, Platform, System};

/// One child process to start.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CommandSpec {
    /// The program, as a path or as a name to find on `PATH`.
    pub(crate) program: PathBuf,
    /// The arguments after the program.
    pub(crate) args: Vec<OsString>,
    /// The folder the child runs in.
    pub(crate) current_dir: PathBuf,
    /// Variables added to the environment the child inherits.
    pub(crate) envs: Vec<(OsString, OsString)>,
    /// Whether the caller reads what the child prints. Otherwise the child
    /// writes straight to the terminal.
    pub(crate) capture: bool,
}

/// How a child process ended.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Finished {
    /// Whether the child reported success.
    pub(crate) success: bool,
    /// What the child printed, when its command captures it.
    pub(crate) stdout: String,
}

/// Where a build finds its inputs and puts its outputs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Environment {
    /// The root of the repository.
    pub(crate) root: PathBuf,
    /// The target folder of cargo, as an absolute path.
    pub(crate) target_dir: PathBuf,
    /// Whether the caller chose the target folder with `CARGO_TARGET_DIR`.
    pub(crate) target_dir_is_chosen: bool,
    /// The cargo that asks for the host.
    pub(crate) cargo: PathBuf,
    /// The Node that runs the Tauri CLI.
    pub(crate) node: PathBuf,
    /// The system this machine runs, when it is one that installers are
    /// built on.
    pub(crate) system: Option<System>,
}

impl Environment {
    /// Reads where things are from the environment of this process.
    pub(crate) fn discover() -> anyhow::Result<Self> {
        // This crate sits at crates/pedagogue-installer, two folders below
        // the root.
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .ancestors()
            .nth(2)
            .context("find the repository root above this crate")?
            .to_path_buf();
        let chosen = std::env::var_os("CARGO_TARGET_DIR").filter(|folder| !folder.is_empty());
        let target_dir = match &chosen {
            // Cargo reads a relative folder against the folder it runs in.
            Some(folder) => std::env::current_dir()
                .context("read the current folder")?
                .join(folder),
            None => root.join("target"),
        };
        // Cargo names itself in `CARGO` when it runs this tool.
        let cargo = std::env::var_os("CARGO")
            .filter(|cargo| !cargo.is_empty())
            .map_or_else(|| PathBuf::from("cargo"), PathBuf::from);
        Ok(Self {
            root,
            target_dir,
            target_dir_is_chosen: chosen.is_some(),
            cargo,
            node: PathBuf::from("node"),
            system: System::current(),
        })
    }

    /// Returns the folder of the UI package, where `npm ci` installs the
    /// Tauri CLI.
    pub(crate) fn ui_dir(&self) -> PathBuf {
        self.root.join("ui")
    }

    /// Returns the entry script of the Tauri CLI.
    pub(crate) fn tauri_cli(&self) -> PathBuf {
        self.ui_dir()
            .join("node_modules")
            .join("@tauri-apps")
            .join("cli")
            .join("tauri.js")
    }

    /// Returns the folder of the desktop crate, which holds
    /// `tauri.conf.json`.
    pub(crate) fn desktop_dir(&self) -> PathBuf {
        self.root.join("crates").join("pedagogue-desktop")
    }

    /// Returns the folder Tauri writes the bundles of one format into.
    pub(crate) fn bundle_dir(&self, platform: Platform, format: Format) -> PathBuf {
        self.target_dir
            .join(platform.triple)
            .join("release")
            .join("bundle")
            .join(format.bundle())
    }

    /// Returns the folder this tool collects the installers of a target
    /// into.
    pub(crate) fn output_dir(&self, platform: Platform) -> PathBuf {
        self.target_dir.join("installer").join(platform.triple)
    }
}

/// The command that asks cargo which target this machine builds for.
pub(crate) fn host_command(environment: &Environment) -> CommandSpec {
    CommandSpec {
        program: environment.cargo.clone(),
        args: vec![OsString::from("-vV")],
        current_dir: environment.root.clone(),
        envs: Vec::new(),
        capture: true,
    }
}

/// The Tauri build of one target: the desktop app in release mode, then the
/// bundler of each format of the target.
///
/// The Tauri CLI runs in the desktop crate's folder, where it finds
/// `tauri.conf.json`. `--ci` keeps it from asking questions.
pub(crate) fn build_command(environment: &Environment, platform: Platform) -> CommandSpec {
    let mut args = vec![environment.tauri_cli().into_os_string()];
    args.extend(["build", "--ci", "--target", platform.triple, "--bundles"].map(OsString::from));
    args.push(OsString::from(platform.bundles()));
    // The Tauri CLI starts cargo in the desktop crate's folder, where a
    // relative target folder would name another place than it names here.
    let envs = if environment.target_dir_is_chosen {
        vec![(
            OsString::from("CARGO_TARGET_DIR"),
            environment.target_dir.clone().into_os_string(),
        )]
    } else {
        Vec::new()
    };
    CommandSpec {
        program: environment.node.clone(),
        args,
        current_dir: environment.desktop_dir(),
        envs,
        capture: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::platform::PLATFORMS;

    fn environment() -> Environment {
        Environment {
            root: PathBuf::from("repo"),
            target_dir: PathBuf::from("repo").join("target"),
            target_dir_is_chosen: false,
            cargo: PathBuf::from("the-cargo"),
            node: PathBuf::from("the-node"),
            system: Some(System::Linux),
        }
    }

    fn os(words: &[&str]) -> Vec<OsString> {
        words.iter().map(OsString::from).collect()
    }

    #[test]
    fn the_tauri_cli_builds_and_bundles_each_target_from_the_desktop_folder() {
        let environment = environment();
        let cli = PathBuf::from("repo")
            .join("ui")
            .join("node_modules")
            .join("@tauri-apps")
            .join("cli")
            .join("tauri.js");
        let expected = [
            ("x86_64-pc-windows-msvc", "nsis"),
            ("aarch64-apple-darwin", "dmg"),
            ("x86_64-apple-darwin", "dmg"),
            ("x86_64-unknown-linux-gnu", "deb,appimage"),
        ];
        assert_eq!(expected.len(), PLATFORMS.len());
        for (triple, bundles) in expected {
            let platform = Platform::from_triple(triple).expect("the triple is supported");
            let mut args = vec![cli.clone().into_os_string()];
            args.extend(os(&[
                "build",
                "--ci",
                "--target",
                triple,
                "--bundles",
                bundles,
            ]));
            assert_eq!(
                build_command(&environment, platform),
                CommandSpec {
                    program: PathBuf::from("the-node"),
                    args,
                    current_dir: PathBuf::from("repo")
                        .join("crates")
                        .join("pedagogue-desktop"),
                    envs: Vec::new(),
                    capture: false,
                },
                "{triple}",
            );
        }
    }

    #[test]
    fn a_chosen_target_folder_reaches_the_tauri_cli_as_an_absolute_path() {
        let chosen = Environment {
            target_dir: PathBuf::from("elsewhere").join("target"),
            target_dir_is_chosen: true,
            ..environment()
        };
        let platform = PLATFORMS[0];
        assert_eq!(
            build_command(&chosen, platform).envs,
            [(
                OsString::from("CARGO_TARGET_DIR"),
                PathBuf::from("elsewhere").join("target").into_os_string()
            )],
        );
        assert_eq!(
            chosen.bundle_dir(platform, Format::Nsis),
            PathBuf::from("elsewhere")
                .join("target")
                .join("x86_64-pc-windows-msvc")
                .join("release")
                .join("bundle")
                .join("nsis"),
        );
        assert_eq!(
            chosen.output_dir(platform),
            PathBuf::from("elsewhere")
                .join("target")
                .join("installer")
                .join("x86_64-pc-windows-msvc"),
        );
    }

    #[test]
    fn the_host_question_goes_to_cargo_from_the_root_and_is_captured() {
        assert_eq!(
            host_command(&environment()),
            CommandSpec {
                program: PathBuf::from("the-cargo"),
                args: os(&["-vV"]),
                current_dir: PathBuf::from("repo"),
                envs: Vec::new(),
                capture: true,
            },
        );
    }

    #[test]
    fn this_process_finds_the_repository_it_was_built_in() {
        let environment = Environment::discover().expect("the environment is readable");
        assert!(environment.root.join("Cargo.toml").is_file());
        assert!(environment.desktop_dir().join("tauri.conf.json").is_file());
        assert!(environment.ui_dir().join("package.json").is_file());
        assert!(environment.target_dir.is_absolute());
        assert_eq!(environment.node, PathBuf::from("node"));
        assert_eq!(environment.system, System::current());
    }
}
