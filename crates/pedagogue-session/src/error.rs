//! The errors a session returns.

use thiserror::Error;

use crate::lesson::{Misstep, Refusal};
use crate::wording;

/// A page command that the session refused.
///
/// `Display` gives a short phrase for a log. [`CommandError::refusal`] gives
/// the full sentence for a model, which says what to fix.
///
/// # Examples
///
/// ```
/// use pedagogue_session::{CommandErrorKind, Session};
/// use serde_json::json;
///
/// let (session, _events) = Session::new();
/// let error = session
///     .apply("exposition", &json!({ "markdown": "Text." }))
///     .expect_err("a command before the first page is refused");
/// assert_eq!(error.to_string(), "`exposition` came before the first page");
/// assert_eq!(error.refusal(), "No page yet. Call new_page first.");
/// assert_eq!(error.kind(), CommandErrorKind::Refused);
/// ```
#[derive(Debug, Error)]
#[error(transparent)]
pub struct CommandError(#[from] Refusal);

impl CommandError {
    /// Returns the sentence a model reads: what went wrong, and what to
    /// fix.
    ///
    /// # Examples
    ///
    /// ```
    /// use pedagogue_session::Session;
    /// use serde_json::json;
    ///
    /// let (session, _events) = Session::new();
    /// session.apply("new_page", &json!({}))?;
    /// let error = session
    ///     .apply("timeline", &json!({ "stops": [{ "label": "Start" }] }))
    ///     .expect_err("a timeline of one stop is refused");
    /// assert_eq!(
    ///     error.refusal(),
    ///     "timeline takes 2 to 8 stops, and this call has 1.",
    /// );
    /// # Ok::<(), pedagogue_session::CommandError>(())
    /// ```
    #[must_use]
    pub fn refusal(&self) -> String {
        wording::refusal(&self.0)
    }

    /// Returns the kind of the refusal, for a caller that maps it to a
    /// status.
    ///
    /// # Examples
    ///
    /// ```
    /// use pedagogue_session::{CommandErrorKind, Session};
    /// use serde_json::json;
    ///
    /// let (session, _events) = Session::new();
    /// let error = session
    ///     .apply("paragraph", &json!({}))
    ///     .expect_err("no command has that name");
    /// assert_eq!(error.kind(), CommandErrorKind::UnknownTool);
    /// ```
    #[must_use]
    pub fn kind(&self) -> CommandErrorKind {
        match self.0 {
            Refusal::UnknownTool { .. } => CommandErrorKind::UnknownTool,
            Refusal::InvalidArgs { .. } | Refusal::Flawed { .. } => {
                CommandErrorKind::InvalidArguments
            }
            Refusal::NoPage { .. }
            | Refusal::UnknownQuiz { .. }
            | Refusal::MarksItself { .. }
            | Refusal::NoAnswerYet { .. } => CommandErrorKind::Refused,
        }
    }
}

/// The kind of a [`CommandError`].
///
/// # Examples
///
/// ```
/// use pedagogue_session::{CommandErrorKind, Session};
/// use serde_json::json;
///
/// let (session, _events) = Session::new();
/// session.apply("new_page", &json!({}))?;
/// let error = session
///     .apply("exposition", &json!({ "text": "Text." }))
///     .expect_err("exposition takes markdown");
/// assert_eq!(error.kind(), CommandErrorKind::InvalidArguments);
/// # Ok::<(), pedagogue_session::CommandError>(())
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CommandErrorKind {
    /// No command has the name the caller gave.
    UnknownTool,
    /// The arguments have the wrong shape, or they break a rule of the
    /// command.
    InvalidArguments,
    /// The command is sound, and the lesson as it stands has no place for
    /// it.
    Refused,
}

/// A student action that the session refused.
///
/// The session stays as it was. `Display` gives a short phrase for a log.
///
/// # Examples
///
/// ```
/// # #[tokio::main(flavor = "current_thread")]
/// # async fn main() {
/// use pedagogue_session::Session;
///
/// let (session, _events) = Session::new();
/// let error = session
///     .say("  ")
///     .await
///     .expect_err("an empty message is refused");
/// assert_eq!(error.to_string(), "the message is empty");
/// # }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error(transparent)]
pub struct ActionError(#[from] Misstep);
