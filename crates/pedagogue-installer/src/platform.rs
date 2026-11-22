//! The targets this tool builds for, the installers of each, and the names a
//! release publishes them by.
//!
//! Every name of a release file comes from [`Installer::file_name`], so the
//! build, its tests and the release workflow agree on them.

/// The product name, as `productName` in the desktop crate's
/// `tauri.conf.json` spells it. Tauri starts each file name with it.
pub(crate) const PRODUCT: &str = "Pedagogue";

/// The version in each file name. The crates of this workspace share one
/// version, so the version of this crate is the version of the app.
pub(crate) const VERSION: &str = env!("CARGO_PKG_VERSION");

/// An operating system that installers are built on and for. The Tauri
/// bundler makes the installers of a system on that system.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum System {
    /// Windows.
    Windows,
    /// macOS.
    MacOs,
    /// Linux.
    Linux,
}

impl System {
    /// Returns the system this tool runs on, when it is one of the three.
    pub(crate) fn current() -> Option<Self> {
        match std::env::consts::OS {
            "windows" => Some(Self::Windows),
            "macos" => Some(Self::MacOs),
            "linux" => Some(Self::Linux),
            _ => None,
        }
    }

    /// Returns the name of the system, for a message.
    pub(crate) fn name(self) -> &'static str {
        match self {
            Self::Windows => "Windows",
            Self::MacOs => "macOS",
            Self::Linux => "Linux",
        }
    }
}

/// A kind of installer that the Tauri bundler makes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Format {
    /// A setup program for Windows, made with NSIS.
    Nsis,
    /// A disk image for macOS that holds the app.
    Dmg,
    /// A package for Debian and Ubuntu.
    Deb,
    /// One file that runs on most Linux distributions.
    AppImage,
}

impl Format {
    /// Returns the name the Tauri CLI knows the format by. The folder the
    /// bundler writes into has the same name.
    pub(crate) fn bundle(self) -> &'static str {
        match self {
            Self::Nsis => "nsis",
            Self::Dmg => "dmg",
            Self::Deb => "deb",
            Self::AppImage => "appimage",
        }
    }

    /// Returns the ending of the format's file names, with its dot.
    pub(crate) fn extension(self) -> &'static str {
        match self {
            Self::Nsis => ".exe",
            Self::Dmg => ".dmg",
            Self::Deb => ".deb",
            Self::AppImage => ".AppImage",
        }
    }
}

/// One installer of a target: its format, and how its file name spells the
/// processor architecture.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Installer {
    /// The kind of installer.
    pub(crate) format: Format,
    arch: &'static str,
}

impl Installer {
    /// Returns the name a release publishes this installer by, such as
    /// `Pedagogue_0.1.0_x64-setup.exe`. Tauri gives the file the same name.
    pub(crate) fn file_name(self) -> String {
        let arch = self.arch;
        match self.format {
            Format::Nsis => format!("{PRODUCT}_{VERSION}_{arch}-setup.exe"),
            Format::Dmg | Format::Deb | Format::AppImage => {
                format!("{PRODUCT}_{VERSION}_{arch}{}", self.format.extension())
            }
        }
    }
}

/// A target this tool builds installers for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Platform {
    /// The Rust target triple.
    pub(crate) triple: &'static str,
    /// The system the target runs on, which is also where its installers
    /// are built.
    pub(crate) system: System,
    /// The installers of the target, in the order they are collected.
    pub(crate) installers: &'static [Installer],
}

/// The targets, one row each. Tauri spells the architecture the way each
/// system does: `x64` on Windows, `aarch64` or `x64` on macOS, and `amd64`
/// on Linux.
pub(crate) const PLATFORMS: [Platform; 4] = [
    Platform {
        triple: "x86_64-pc-windows-msvc",
        system: System::Windows,
        installers: &[Installer {
            format: Format::Nsis,
            arch: "x64",
        }],
    },
    Platform {
        triple: "aarch64-apple-darwin",
        system: System::MacOs,
        installers: &[Installer {
            format: Format::Dmg,
            arch: "aarch64",
        }],
    },
    Platform {
        triple: "x86_64-apple-darwin",
        system: System::MacOs,
        installers: &[Installer {
            format: Format::Dmg,
            arch: "x64",
        }],
    },
    Platform {
        triple: "x86_64-unknown-linux-gnu",
        system: System::Linux,
        installers: &[
            Installer {
                format: Format::Deb,
                arch: "amd64",
            },
            Installer {
                format: Format::AppImage,
                arch: "amd64",
            },
        ],
    },
];

impl Platform {
    /// Finds the platform of a target triple.
    pub(crate) fn from_triple(triple: &str) -> Option<Self> {
        PLATFORMS
            .into_iter()
            .find(|platform| platform.triple == triple)
    }

    /// Returns the list that the `--bundles` flag of the Tauri CLI takes,
    /// such as `deb,appimage`.
    pub(crate) fn bundles(self) -> String {
        let formats: Vec<&str> = self
            .installers
            .iter()
            .map(|installer| installer.format.bundle())
            .collect();
        formats.join(",")
    }
}

/// Lists the supported target triples, for a message.
pub(crate) fn supported_triples() -> String {
    PLATFORMS.map(|platform| platform.triple).join(", ")
}

/// Reads the host triple out of what `cargo -vV` prints.
pub(crate) fn host_triple(version_output: &str) -> Option<&str> {
    version_output
        .lines()
        .find_map(|line| line.strip_prefix("host: "))
        .map(str::trim)
        .filter(|triple| !triple.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::path::Path;

    fn names(triple: &str) -> Vec<String> {
        let platform = Platform::from_triple(triple).expect("the triple is supported");
        platform
            .installers
            .iter()
            .map(|installer| installer.file_name())
            .collect()
    }

    #[test]
    fn each_target_has_its_formats_and_release_names() {
        assert_eq!(
            names("x86_64-pc-windows-msvc"),
            [format!("Pedagogue_{VERSION}_x64-setup.exe")]
        );
        assert_eq!(
            names("aarch64-apple-darwin"),
            [format!("Pedagogue_{VERSION}_aarch64.dmg")]
        );
        assert_eq!(
            names("x86_64-apple-darwin"),
            [format!("Pedagogue_{VERSION}_x64.dmg")]
        );
        assert_eq!(
            names("x86_64-unknown-linux-gnu"),
            [
                format!("Pedagogue_{VERSION}_amd64.deb"),
                format!("Pedagogue_{VERSION}_amd64.AppImage"),
            ],
        );
    }

    #[test]
    fn each_target_is_built_on_the_system_it_runs_on() {
        let systems = PLATFORMS.map(|platform| (platform.triple, platform.system));
        assert_eq!(
            systems,
            [
                ("x86_64-pc-windows-msvc", System::Windows),
                ("aarch64-apple-darwin", System::MacOs),
                ("x86_64-apple-darwin", System::MacOs),
                ("x86_64-unknown-linux-gnu", System::Linux),
            ],
        );
        assert_eq!(
            [System::Windows, System::MacOs, System::Linux].map(System::name),
            ["Windows", "macOS", "Linux"],
        );
    }

    #[test]
    fn the_current_system_is_the_one_this_test_was_built_for() {
        let current = System::current();
        assert_eq!(
            current == Some(System::Windows),
            cfg!(target_os = "windows")
        );
        assert_eq!(current == Some(System::MacOs), cfg!(target_os = "macos"));
        assert_eq!(current == Some(System::Linux), cfg!(target_os = "linux"));
    }

    #[test]
    fn each_release_name_ends_with_the_extension_of_its_format() {
        for platform in PLATFORMS {
            for installer in platform.installers {
                assert!(
                    installer
                        .file_name()
                        .ends_with(installer.format.extension()),
                    "{}",
                    installer.file_name()
                );
            }
        }
    }

    #[test]
    fn the_bundles_flag_lists_the_formats_of_a_target() {
        let bundles = |triple: &str| {
            Platform::from_triple(triple)
                .expect("the triple is supported")
                .bundles()
        };
        assert_eq!(bundles("x86_64-pc-windows-msvc"), "nsis");
        assert_eq!(bundles("aarch64-apple-darwin"), "dmg");
        assert_eq!(bundles("x86_64-apple-darwin"), "dmg");
        assert_eq!(bundles("x86_64-unknown-linux-gnu"), "deb,appimage");
    }

    #[test]
    fn a_triple_outside_the_list_has_no_platform() {
        for triple in [
            "aarch64-pc-windows-msvc",
            "x86_64-pc-windows-gnu",
            "aarch64-unknown-linux-gnu",
            "x86_64-unknown-linux-musl",
            "X86_64-PC-WINDOWS-MSVC",
            "x86_64-pc-windows-msvc ",
            "",
        ] {
            assert_eq!(Platform::from_triple(triple), None, "{triple:?}");
        }
        assert_eq!(
            supported_triples(),
            "x86_64-pc-windows-msvc, aarch64-apple-darwin, x86_64-apple-darwin, \
             x86_64-unknown-linux-gnu",
        );
    }

    #[test]
    fn the_host_triple_is_read_from_the_version_report_of_cargo() {
        let report = "cargo 1.99.1 (5f94df478 2026-08-27)\nrelease: 1.99.1\n\
                      host: aarch64-apple-darwin\nos: Mac OS 15.0.0 [64-bit]\n";
        assert_eq!(host_triple(report), Some("aarch64-apple-darwin"));
        assert_eq!(
            host_triple("host: x86_64-pc-windows-msvc\r\n"),
            Some("x86_64-pc-windows-msvc")
        );
        assert_eq!(host_triple("cargo 1.99.1\nrelease: 1.99.1\n"), None);
        assert_eq!(host_triple("host: \n"), None);
    }

    /// Reads a file of the desktop crate, which sits beside this crate.
    fn desktop_file(name: &str) -> String {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("pedagogue-desktop")
            .join(name);
        std::fs::read_to_string(&path)
            .unwrap_or_else(|error| panic!("read {}: {error}", path.display()))
    }

    #[test]
    fn the_product_name_is_the_one_tauri_bundles_under() {
        let config: serde_json::Value =
            serde_json::from_str(&desktop_file("tauri.conf.json")).expect("the config is JSON");
        assert_eq!(config["productName"], PRODUCT);
        // With no version of its own, the config takes the version of the
        // desktop crate, which is the workspace's and so this crate's.
        assert_eq!(config.get("version"), None);
    }

    #[test]
    fn the_readme_names_every_release_file() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("..")
            .join("README.md");
        let readme = std::fs::read_to_string(&path)
            .unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
        for platform in PLATFORMS {
            for installer in platform.installers {
                let name = installer.file_name();
                // The Install section names each file of the current
                // release, so a new version changes it too.
                assert!(readme.contains(&format!("`{name}`")), "{name}");
            }
        }
    }

    #[test]
    fn the_desktop_crate_shares_the_version_of_this_crate() {
        let shares = |manifest: &str| {
            manifest
                .lines()
                .any(|line| line.trim() == "version.workspace = true")
        };
        assert!(shares(&desktop_file("Cargo.toml")));
        assert!(shares(include_str!("../Cargo.toml")));
    }
}
