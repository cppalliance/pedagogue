//! The Pedagogue desktop app.
//!
//! It starts the in-process server, plays a lesson script into a session,
//! and opens one window on the server's loopback address.
//!
//! # Examples
//!
//! ```no_run
//! fn main() -> std::process::ExitCode {
//!     pedagogue_desktop::run()
//! }
//! ```

mod navigation;
mod run;

pub use crate::run::run;
