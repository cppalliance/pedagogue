//! Builds the TeachPad installers: an NSIS setup for Windows, a disk image
//! for macOS, and a Debian package and an AppImage for Linux.
//!
//! This crate is a build tool for this repository, run as `cargo installer`.
//! It starts the Tauri CLI, which builds the desktop app in release mode and
//! runs the bundlers of the target's system. Then it copies each installer
//! into one folder, under the name a release publishes it by.
//!
//! # Examples
//!
//! ```no_run
//! fn main() -> std::process::ExitCode {
//!     teachpad_installer::run()
//! }
//! ```

mod args;
mod collect;
mod platform;
mod run;
mod tauri;

pub use crate::run::run;
