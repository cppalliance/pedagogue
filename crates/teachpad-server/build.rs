//! Builds the UI bundle before the Rust compile.
//!
//! It runs `node build.mjs` in the `ui/` package at the repository root and
//! has it write `index.html`, `app.js` and `app.css` to `$OUT_DIR/ui-dist/`,
//! so the build writes nothing into the repository. It needs Node.js on
//! `PATH` and one `npm install` in `ui/` per checkout.

use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

/// What changes the bundle, relative to `ui/`. Cargo watches a folder
/// with everything under it.
const UI_INPUTS: [&str; 7] = [
    "src",
    "index.html",
    "build.mjs",
    "sprites.mjs",
    "package.json",
    "package-lock.json",
    "tsconfig.json",
];

/// The design files that `build.mjs` copies the icon sprites out of,
/// relative to the repository root.
const DESIGN_INPUTS: [&str; 2] = ["design/page-contact-sheet.html", "design/chat-panel.html"];

fn main() -> ExitCode {
    match build() {
        Ok(()) => ExitCode::SUCCESS,
        Err(problem) => {
            eprintln!("{problem}");
            ExitCode::FAILURE
        }
    }
}

fn build() -> Result<(), String> {
    let crate_dir = PathBuf::from(from_cargo("CARGO_MANIFEST_DIR")?);
    let out_dir = PathBuf::from(from_cargo("OUT_DIR")?);
    let root = crate_dir.join("..").join("..");
    let ui_dir = root.join("ui");

    for input in UI_INPUTS {
        watch(&ui_dir.join(input));
    }
    for input in DESIGN_INPUTS {
        watch(&root.join(input));
    }

    if !ui_dir.join("node_modules").is_dir() {
        return Err(format!(
            "the UI package has no node_modules. Run `npm install` in {} and build again.",
            ui_dir.display()
        ));
    }
    let dist_dir = out_dir.join("ui-dist");
    let output = Command::new("node")
        .arg("build.mjs")
        .arg("--out")
        .arg(&dist_dir)
        .current_dir(&ui_dir)
        .output()
        .map_err(|error| {
            format!("node could not be started: {error}. Install Node.js so it is on PATH.")
        })?;
    if output.status.success() {
        return Ok(());
    }
    Err(format!(
        "the UI bundle failed ({}):\n{}\n{}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    ))
}

/// Reads a variable that Cargo sets for a build script.
fn from_cargo(name: &str) -> Result<std::ffi::OsString, String> {
    std::env::var_os(name).ok_or_else(|| format!("{name} is not set. Run this through cargo."))
}

/// Tells Cargo to run this script again when a path changes. A path that
/// is missing would rerun the script on every build, so it is left out.
fn watch(path: &Path) {
    if path.exists() {
        println!("cargo::rerun-if-changed={}", path.display());
    }
}
