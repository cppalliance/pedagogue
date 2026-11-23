//! The lesson model, the chat transcript and the session handle of TeachPad.
//!
//! A host fills textbook pages through page commands, talks with the student
//! through the chat, and hears back what the student did. This crate holds
//! that state and its rules, and it is the one crate a host depends on.
//! ARCHITECTURE.md at the repository root holds the code map.
//!
//! # Examples
//!
//! ```
//! # #[tokio::main(flavor = "current_thread")]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! use teachpad_session::{Session, Version};
//! use serde_json::json;
//!
//! let (session, mut events) = Session::new();
//!
//! // A host writes a page and a chat message.
//! session.apply("new_page", &json!({ "section_title": "Taking a Straw Poll" }))?;
//! session.apply("exposition", &json!({ "markdown": "The chair states the question." }))?;
//! session.assistant_message("Read this section, then ask me anything.");
//!
//! // A server sends the UI what changed, and passes in what the student does.
//! let changes = session.changes_since(Version::ZERO);
//! assert_eq!(changes.frames().len(), 2);
//! session.say("Why five columns?").await?;
//!
//! // The host hears it, in a form ready for a model.
//! let event = events.recv().await.ok_or("the session is alive")?;
//! assert_eq!(event.to_model_text(), "Why five columns?");
//! # Ok(())
//! # }
//! ```

mod changes;
mod chat;
mod error;
mod event;
mod frames;
mod lesson;
mod session;
mod wording;

pub use crate::changes::{Changes, Version};
pub use crate::chat::ChatState;
pub use crate::error::{ActionError, CommandError, CommandErrorKind};
pub use crate::event::{Pick, QuizAnswer, StudentEvent};
pub use crate::frames::{Frame, StudentFrame};
pub use crate::lesson::{BlockId, PageNumber, ResponseKind};
pub use crate::session::{Session, StudentEvents};
