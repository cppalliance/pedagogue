//! What the student did, as events for the host.

use crate::lesson::{BlockId, PageNumber, ResponseKind};
use crate::wording;

/// Something the student did.
///
/// Events reach the host in the order they happened, on the stream that
/// [`Session::new`](crate::Session::new) returns.
///
/// # Examples
///
/// ```
/// use pedagogue_session::StudentEvent;
///
/// let event = StudentEvent::UserMessage {
///     text: "Why five columns?".to_owned(),
/// };
/// assert_eq!(event.to_model_text(), "Why five columns?");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StudentEvent {
    /// The student sent a chat message.
    UserMessage {
        /// The message, as the student wrote it.
        text: String,
    },
    /// The student answered a quiz. This fires on every attempt, wrong ones
    /// included, which is how a host learns of a wrong answer.
    QuizAnswered {
        /// The id of the quiz.
        quiz: BlockId,
        /// The number of this attempt at the quiz, from 1.
        attempt: usize,
        /// What the student answered.
        answer: QuizAnswer,
    },
    /// The student pressed the Continue button at the foot of a page.
    ContinuePressed {
        /// The page whose button was pressed.
        page: PageNumber,
        /// The label of the button.
        label: String,
    },
    /// The student asked for the last tutor reply again.
    RegeneratePressed,
}

impl StudentEvent {
    /// Writes the event as one line of plain text for a model to read.
    ///
    /// # Examples
    ///
    /// ```
    /// use pedagogue_session::{Pick, QuizAnswer, StudentEvent};
    ///
    /// let event = StudentEvent::QuizAnswered {
    ///     quiz: serde_json::from_str("\"b7\"")?,
    ///     attempt: 1,
    ///     answer: QuizAnswer::Picks {
    ///         picks: vec![Pick {
    ///             letter: 'B',
    ///             text: "Count the room".to_owned(),
    ///         }],
    ///         correct: false,
    ///     },
    /// };
    /// assert_eq!(
    ///     event.to_model_text(),
    ///     "Quiz b7, attempt 1: chose B \"Count the room\". Incorrect.",
    /// );
    /// # Ok::<(), serde_json::Error>(())
    /// ```
    #[must_use]
    pub fn to_model_text(&self) -> String {
        wording::student_event(self)
    }
}

/// The answer a student gave to a quiz.
///
/// # Examples
///
/// ```
/// use pedagogue_session::{QuizAnswer, ResponseKind};
///
/// let answer = QuizAnswer::Text {
///     kind: ResponseKind::ShortAnswer,
///     text: "It casts to an rvalue.".to_owned(),
/// };
/// assert!(matches!(answer, QuizAnswer::Text { .. }));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QuizAnswer {
    /// Picks on a multiple-choice quiz, which the page marked at once.
    Picks {
        /// The options the student picked, in the order shown.
        picks: Vec<Pick>,
        /// Whether the picks solved the quiz.
        correct: bool,
    },
    /// Text for a free-text quiz, which waits for the host's verdict.
    Text {
        /// The kind of free-text quiz.
        kind: ResponseKind,
        /// The answer, as the student wrote it.
        text: String,
    },
}

/// One option a student picked.
///
/// # Examples
///
/// ```
/// use pedagogue_session::Pick;
///
/// let pick = Pick {
///     letter: 'B',
///     text: "Count the room".to_owned(),
/// };
/// assert_eq!(pick.letter, 'B');
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pick {
    /// The letter the page shows before the option, from `A`.
    pub letter: char,
    /// The text of the option.
    pub text: String,
}
