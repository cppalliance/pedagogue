//! Text the stand-in writes into the chat and onto the page.
//!
//! The stand-in holds the tutor's place, so each line says so. This file is
//! the one home of that wording in this crate.

/// The stand-in's one reply to a message from the student.
pub(crate) const REPLY: &str =
    "I am a stand-in. A real tutor answers here once Runvil is connected.";

/// The stand-in's feedback on a first free-text answer, which it marks
/// incorrect so the student can see the box reopen.
pub(crate) const FIRST_TRY: &str = "The stand-in marks every first answer this way, so you can see the box reopen. Press Try again.";

/// The stand-in's feedback on a later free-text answer, which it marks
/// correct.
pub(crate) const LATER_TRY: &str =
    "The stand-in marks every later answer as correct. A real tutor reads what you wrote.";

/// Writes the chat line that shows what a tutor would be sent.
///
/// `report` is the exact text a model gets for one thing the student did.
pub(crate) fn sent_to_the_tutor(report: &str) -> String {
    format!("**Sent to the tutor:** {report}")
}
