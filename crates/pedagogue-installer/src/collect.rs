//! Clearing the folders of a build, and collecting the installers that
//! Tauri leaves in them.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use anyhow::{Context as _, bail};

use crate::platform::Platform;
use crate::tauri::Environment;

/// Removes what an earlier build left: the output folder of the target, and
/// Tauri's bundle folder of each format about to be built.
///
/// Each folder then holds the files of one build, so a file from an earlier
/// version cannot be taken for this build's.
pub(crate) fn clear(environment: &Environment, platform: Platform) -> anyhow::Result<()> {
    remove_folder(&environment.output_dir(platform))?;
    for installer in platform.installers {
        remove_folder(&environment.bundle_dir(platform, installer.format))?;
    }
    Ok(())
}

/// Copies each installer of the target out of Tauri's bundle folder into
/// the output folder, under its release name.
///
/// Returns the copies, in the order of the platform's installers.
pub(crate) fn collect(
    environment: &Environment,
    platform: Platform,
) -> anyhow::Result<Vec<PathBuf>> {
    let output = environment.output_dir(platform);
    fs::create_dir_all(&output)
        .with_context(|| format!("create the folder {}", output.display()))?;
    let mut collected = Vec::new();
    for installer in platform.installers {
        let built = single_file(
            &environment.bundle_dir(platform, installer.format),
            installer.format.extension(),
        )?;
        let copy = output.join(installer.file_name());
        fs::copy(&built, &copy)
            .with_context(|| format!("copy {} to {}", built.display(), copy.display()))?;
        collected.push(copy);
    }
    Ok(collected)
}

/// Finds the one file at the top level of a folder whose name ends with
/// `extension`. A folder of that name is passed over.
fn single_file(folder: &Path, extension: &str) -> anyhow::Result<PathBuf> {
    let entries = fs::read_dir(folder)
        .with_context(|| format!("read the bundle folder {}", folder.display()))?;
    let mut found = Vec::new();
    for entry in entries {
        let path = entry
            .with_context(|| format!("read the bundle folder {}", folder.display()))?
            .path();
        let is_named = path
            .file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| name.ends_with(extension));
        if is_named && path.is_file() {
            found.push(path);
        }
    }
    // The order of a folder's entries differs from system to system.
    found.sort();
    match found.as_slice() {
        [only] => Ok(only.clone()),
        [] => bail!(
            "the Tauri build left no `*{extension}` file in {}",
            folder.display()
        ),
        several => {
            let names: Vec<String> = several
                .iter()
                .filter_map(|path| path.file_name())
                .map(|name| name.to_string_lossy().into_owned())
                .collect();
            bail!(
                "expected one `*{extension}` file in {}, and it holds {}: {}",
                folder.display(),
                several.len(),
                names.join(", ")
            )
        }
    }
}

/// Removes a folder and everything in it. A folder that is already gone is
/// fine.
fn remove_folder(folder: &Path) -> anyhow::Result<()> {
    match fs::remove_dir_all(folder) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error).with_context(|| format!("clear the folder {}", folder.display())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::platform::{Format, VERSION};

    /// An environment whose target folder is a fresh temp folder.
    fn environment(temp: &tempfile::TempDir) -> Environment {
        Environment {
            root: temp.path().join("repo"),
            target_dir: temp.path().join("target"),
            cargo: PathBuf::from("cargo"),
            node: PathBuf::from("node"),
            // Collecting reads folders and copies files, on any system.
            system: None,
        }
    }

    fn platform(triple: &str) -> Platform {
        Platform::from_triple(triple).expect("the triple is supported")
    }

    /// Writes a file, with its folders, the way a bundler would.
    fn write(path: &Path, contents: &str) {
        fs::create_dir_all(path.parent().expect("the path has a folder"))
            .expect("the folder is made");
        fs::write(path, contents).expect("the file is written");
    }

    fn message(result: anyhow::Result<Vec<PathBuf>>) -> String {
        format!("{:#}", result.expect_err("the collection fails"))
    }

    #[test]
    fn the_one_installer_is_copied_under_its_release_name() {
        let temp = tempfile::TempDir::new().expect("a temp folder");
        let environment = environment(&temp);
        let windows = platform("x86_64-pc-windows-msvc");
        let bundle = environment.bundle_dir(windows, Format::Nsis);
        // Tauri names the file itself, and the name here comes from this
        // crate, so the test gives the built file another name.
        write(&bundle.join("Built by Tauri-setup.exe"), "the setup");

        let collected = collect(&environment, windows).expect("the installer is collected");

        let expected = environment
            .output_dir(windows)
            .join(format!("Pedagogue_{VERSION}_x64-setup.exe"));
        assert_eq!(collected, [expected.as_path()]);
        assert_eq!(
            fs::read_to_string(&expected).expect("the copy is readable"),
            "the setup"
        );
        assert!(
            bundle.join("Built by Tauri-setup.exe").is_file(),
            "the built file stays"
        );
    }

    #[test]
    fn a_linux_build_yields_the_package_and_the_appimage_past_the_staging_folders() {
        let temp = tempfile::TempDir::new().expect("a temp folder");
        let environment = environment(&temp);
        let linux = platform("x86_64-unknown-linux-gnu");
        let deb = environment.bundle_dir(linux, Format::Deb);
        let appimage = environment.bundle_dir(linux, Format::AppImage);
        write(&deb.join("Pedagogue_9.9.9_amd64.deb"), "the package");
        // The bundlers leave their staging trees beside the files.
        write(
            &deb.join("Pedagogue_9.9.9_amd64").join("control"),
            "staging",
        );
        write(
            &appimage.join("Pedagogue_9.9.9_amd64.AppImage"),
            "the appimage",
        );
        write(&appimage.join("Pedagogue.AppDir").join("AppRun"), "staging");
        fs::create_dir_all(appimage.join("folder.AppImage")).expect("the folder is made");

        let collected = collect(&environment, linux).expect("both installers are collected");

        let output = environment.output_dir(linux);
        assert_eq!(
            collected,
            [
                output.join(format!("Pedagogue_{VERSION}_amd64.deb")),
                output.join(format!("Pedagogue_{VERSION}_amd64.AppImage")),
            ],
        );
        assert_eq!(
            fs::read_to_string(&collected[0]).expect("readable"),
            "the package"
        );
        assert_eq!(
            fs::read_to_string(&collected[1]).expect("readable"),
            "the appimage"
        );
    }

    #[test]
    fn a_bundle_folder_without_its_installer_is_an_error() {
        let temp = tempfile::TempDir::new().expect("a temp folder");
        let environment = environment(&temp);
        let mac = platform("aarch64-apple-darwin");
        let bundle = environment.bundle_dir(mac, Format::Dmg);

        // The folder itself is missing.
        let missing = message(collect(&environment, mac));
        assert!(
            missing.starts_with(&format!("read the bundle folder {}: ", bundle.display())),
            "{missing}"
        );

        // The folder holds other files only.
        write(&bundle.join("bundle_dmg.sh"), "a script");
        assert_eq!(
            message(collect(&environment, mac)),
            format!(
                "the Tauri build left no `*.dmg` file in {}",
                bundle.display()
            ),
        );
    }

    #[test]
    fn two_candidates_are_an_error_that_names_both() {
        let temp = tempfile::TempDir::new().expect("a temp folder");
        let environment = environment(&temp);
        let mac = platform("x86_64-apple-darwin");
        let bundle = environment.bundle_dir(mac, Format::Dmg);
        write(&bundle.join("Pedagogue_0.1.0_x64.dmg"), "one");
        write(&bundle.join("Pedagogue_0.2.0_x64.dmg"), "two");

        assert_eq!(
            message(collect(&environment, mac)),
            format!(
                "expected one `*.dmg` file in {}, and it holds 2: Pedagogue_0.1.0_x64.dmg, \
                 Pedagogue_0.2.0_x64.dmg",
                bundle.display()
            ),
        );
    }

    #[test]
    fn clearing_removes_the_output_and_the_bundle_folders_of_the_target_alone() {
        let temp = tempfile::TempDir::new().expect("a temp folder");
        let environment = environment(&temp);
        let linux = platform("x86_64-unknown-linux-gnu");
        let windows = platform("x86_64-pc-windows-msvc");
        let stale = [
            environment
                .output_dir(linux)
                .join("Pedagogue_0.0.1_amd64.deb"),
            environment
                .bundle_dir(linux, Format::Deb)
                .join("Pedagogue_0.0.1_amd64.deb"),
            environment
                .bundle_dir(linux, Format::AppImage)
                .join("Pedagogue_0.0.1_amd64.AppImage"),
        ];
        let kept = [
            environment
                .output_dir(windows)
                .join("Pedagogue_0.0.1_x64-setup.exe"),
            environment
                .bundle_dir(windows, Format::Nsis)
                .join("Pedagogue_0.0.1_x64-setup.exe"),
            // The release build itself stays, so the next build is fast.
            environment
                .target_dir
                .join(linux.triple)
                .join("release")
                .join("pedagogue"),
        ];
        for path in stale.iter().chain(&kept) {
            write(path, "from an earlier build");
        }

        clear(&environment, linux).expect("the folders are cleared");
        // A second clearing finds nothing to remove, which is fine.
        clear(&environment, linux).expect("cleared folders clear again");

        for path in &stale {
            assert!(!path.exists(), "{} is gone", path.display());
        }
        for path in &kept {
            assert!(path.is_file(), "{} stays", path.display());
        }
    }
}
