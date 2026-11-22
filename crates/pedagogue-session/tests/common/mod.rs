//! Shared helpers for the session integration tests: a session on a clock
//! that stands still, short ways to name a quiz and a page, and a reader
//! that returns what changed as JSON.

// clippy.toml's allow-expect-in-tests covers #[test] functions and
// #[cfg(test)] modules only, so these helpers ask for it themselves.
#![expect(
    clippy::expect_used,
    reason = "test helpers fail by panicking with the invariant named"
)]

use std::time::{Duration, UNIX_EPOCH};

use pedagogue_session::{BlockId, PageNumber, Session, StudentEvents, Version};
use serde_json::{Value, json};

/// The time on the clock of a test session, in milliseconds since the Unix
/// epoch.
pub(crate) const NOW_MS: u64 = 1_795_300_000_000;

/// Makes a session whose clock stands still at [`NOW_MS`].
pub(crate) fn session() -> (Session, StudentEvents) {
    Session::with_clock(|| UNIX_EPOCH + Duration::from_millis(NOW_MS))
}

/// Applies page commands in order and returns their receipts. Each one has
/// to apply.
pub(crate) fn apply_all(session: &Session, calls: &[(&str, Value)]) -> Vec<String> {
    calls
        .iter()
        .map(|(tool, args)| {
            session
                .apply(tool, args)
                .unwrap_or_else(|error| panic!("{tool} applies: {}", error.refusal()))
        })
        .collect()
}

/// Returns the id a block has on the wire, such as `b3`.
pub(crate) fn quiz(id: &str) -> BlockId {
    serde_json::from_value(json!(id)).expect("the text is a block id")
}

/// Returns the number of a page.
pub(crate) fn page(number: u32) -> PageNumber {
    serde_json::from_value(json!(number)).expect("the number is a page number")
}

/// Reads what changed since a cursor: the frames as JSON, and the reader's
/// next cursor.
pub(crate) fn read(session: &Session, cursor: Version) -> (Vec<Value>, Version) {
    let changes = session.changes_since(cursor);
    let frames = changes
        .frames()
        .iter()
        .map(|frame| serde_json::to_value(frame).expect("a frame serializes"))
        .collect();
    (frames, changes.version())
}

/// Says what kind each frame is, such as `page 2` or `chat`.
pub(crate) fn kinds(frames: &[Value]) -> Vec<String> {
    frames
        .iter()
        .map(|frame| match frame["type"].as_str() {
            Some("page") => format!("page {}", frame["page"]["number"]),
            Some(other) => other.to_owned(),
            None => "untyped".to_owned(),
        })
        .collect()
}
