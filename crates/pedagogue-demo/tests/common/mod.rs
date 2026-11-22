//! Shared helpers for the demo integration tests: a session on a clock that
//! stands still, the sample files, and a reader that returns every frame as
//! JSON.

// clippy.toml's allow-expect-in-tests covers #[test] functions and
// #[cfg(test)] modules only, so these helpers ask for it themselves.
#![expect(
    clippy::expect_used,
    reason = "test helpers fail by panicking with the invariant named"
)]

use std::path::PathBuf;
use std::time::{Duration, UNIX_EPOCH};

use pedagogue_demo::Script;
use pedagogue_session::{Session, StudentEvents, Version};
use serde_json::Value;

/// Makes a session whose clock stands still, so the frames of a sample are
/// the same on every run.
pub(crate) fn session() -> (Session, StudentEvents) {
    Session::with_clock(|| UNIX_EPOCH + Duration::from_secs(1_795_300_000))
}

/// Returns the path of a file in the repository's `samples` folder.
pub(crate) fn sample_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../samples")
        .join(name)
}

/// Reads and parses a sample script.
pub(crate) fn sample(name: &str) -> Script {
    let text = std::fs::read_to_string(sample_path(name)).expect("the sample is on disk");
    Script::parse(&text).expect("the sample parses")
}

/// Returns every frame of a session as JSON: its pages, then the chat.
pub(crate) fn frames(session: &Session) -> Vec<Value> {
    session
        .changes_since(Version::ZERO)
        .frames()
        .iter()
        .map(|frame| serde_json::to_value(frame).expect("a frame serializes"))
        .collect()
}
