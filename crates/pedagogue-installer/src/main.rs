//! Entry point of `cargo installer`.

use std::process::ExitCode;

fn main() -> ExitCode {
    pedagogue_installer::run()
}
