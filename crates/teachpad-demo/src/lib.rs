//! A sample script player and a stand-in tutor for TeachPad.
//!
//! This crate stands where a real tutor will: it plays a JSON script of page
//! commands and chat lines into a session, then answers the student with a
//! fixed reply. It uses the session crate exactly as a real host would.
//!
//! # Examples
//!
//! ```
//! # #[tokio::main(flavor = "current_thread")]
//! # async fn main() -> Result<(), teachpad_demo::ScriptError> {
//! use teachpad_demo::{Pace, Script, play};
//! use teachpad_session::{Session, Version};
//!
//! let script = Script::parse(
//!     r#"[
//!         { "tool": "new_page", "args": { "section_title": "Taking a Straw Poll" } },
//!         { "tool": "pop_quiz", "as": "poll", "args": {
//!             "question": "What should the chair do first?",
//!             "options": [
//!                 { "text": "Restate the question", "correct": true },
//!                 { "text": "Count the room", "correct": false }
//!             ] } },
//!         { "answer": { "quiz": "@poll", "picks": [1] } }
//!     ]"#,
//! )?;
//! let (session, mut events) = Session::new();
//! play(&session, &mut events, &script, Pace::Instant).await?;
//!
//! let changes = session.changes_since(Version::ZERO);
//! let wire = serde_json::to_value(changes.frames()).expect("frames serialize");
//! assert_eq!(wire[0]["page"]["blocks"][0]["options"][1]["state"], "disabled");
//! # Ok(())
//! # }
//! ```

mod error;
mod pace;
mod player;
mod script;
mod stand_in;
mod wording;

pub use crate::error::ScriptError;
pub use crate::pace::Pace;
pub use crate::player::{play, run};
pub use crate::script::{STRAW_POLL, Script};
