//! Entry point of `cargo installer`.

use std::process::ExitCode;

fn main() -> ExitCode {
    teachpad_installer::run()
}
