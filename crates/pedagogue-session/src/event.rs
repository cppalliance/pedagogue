//! What the student did, as events for the host.

use crate::lesson::{BlockId, PageNumber, ResponseKind};

/// Something the student did.
///
/// Events reach the host in the order they happened, on one stream.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum StudentEvent {
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
}

/// The answer a student gave to a quiz.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum QuizAnswer {
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Pick {
    /// The letter the page shows before the option, from `A`.
    pub(crate) letter: char,
    /// The text of the option.
    pub(crate) text: String,
}
