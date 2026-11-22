//! Entry point of the Pedagogue desktop app.

// A release build on Windows opens no console window beside the app.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::ExitCode;

fn main() -> ExitCode {
    pedagogue_desktop::run()
}
