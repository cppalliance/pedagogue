//! Why the lesson refuses a command.
//!
//! These types say what went wrong in a form the code can match on. The
//! sentence a model reads is written from them in `wording.rs`.

use thiserror::Error;

use super::id::{BlockId, PageNumber};

/// A rule that a command's arguments break.
#[derive(Debug, Clone, PartialEq, Error)]
pub(crate) enum Flaw {
    /// A text field holds only whitespace.
    #[error("`{field}` is empty")]
    Empty {
        /// The field that needs text.
        field: &'static str,
    },
    /// A list holds too few or too many items.
    #[error("`{field}` holds {actual} items, outside {min} to {max}")]
    Count {
        /// The field that holds the list.
        field: &'static str,
        /// The fewest items the list may hold.
        min: usize,
        /// The most items the list may hold.
        max: usize,
        /// The number of items the list holds.
        actual: usize,
    },
    /// A number that has to be above zero is zero.
    #[error("`{field}` is zero")]
    Zero {
        /// The field that holds the number.
        field: &'static str,
    },
    /// A bar's value is below zero.
    #[error("bar `{label}` has a value below zero")]
    NegativeBar {
        /// The label of the bar.
        label: String,
    },
    /// A tree label appears more than once.
    #[error("tree label `{label}` appears twice")]
    DuplicateLabel {
        /// The label that appears twice.
        label: String,
    },
    /// A tree edge names a label that no row holds.
    #[error("a tree edge names the missing label `{label}`")]
    UnknownLabel {
        /// The label that no row holds.
        label: String,
    },
    /// A tree edge runs sideways or upward.
    #[error("the tree edge from `{from}` to `{to}` does not run to a lower row")]
    EdgeNotDownward {
        /// The label the edge starts at.
        from: String,
        /// The label the edge ends at.
        to: String,
    },
    /// A progress box counts more done than there is in total.
    #[error("`done` {done} is over `total` {total}")]
    DoneOverTotal {
        /// The count of finished parts.
        done: u32,
        /// The count of all parts.
        total: u32,
    },
    /// A choice quiz marks the wrong number of options as right: none at
    /// all, or more than one on a quiz with one right option.
    #[error("{actual} options are marked right")]
    RightOptions {
        /// Whether the quiz is a select-all quiz.
        multiple: bool,
        /// The number of options marked right.
        actual: usize,
    },
}

/// Why a command was refused.
#[derive(Debug, Error)]
pub(crate) enum Refusal {
    /// No command has this name.
    #[error("unknown tool `{tool}`")]
    UnknownTool {
        /// The name the caller gave.
        tool: String,
    },
    /// The arguments do not have the shape the command takes.
    #[error("invalid arguments for `{tool}`")]
    InvalidArgs {
        /// The command.
        tool: String,
        /// The top-level field that holds the problem, when it can be told.
        field: Option<String>,
        /// What the parser found.
        source: serde_json::Error,
    },
    /// The command needs a page, and the lesson has yet to start one.
    #[error("`{tool}` came before the first page")]
    NoPage {
        /// The command.
        tool: &'static str,
    },
    /// The arguments have the right shape and break a rule of the command.
    #[error("the arguments of `{tool}` break a rule")]
    Flawed {
        /// The command.
        tool: &'static str,
        /// The rule the arguments break.
        #[source]
        flaw: Flaw,
    },
    /// A verdict names an id that no quiz has.
    #[error("no quiz has the id `{quiz}`")]
    UnknownQuiz {
        /// The id the command named.
        quiz: BlockId,
    },
    /// A verdict names a choice quiz, which the page marks itself.
    #[error("quiz `{quiz}` is marked by the page")]
    MarksItself {
        /// The id of the choice quiz.
        quiz: BlockId,
    },
    /// A verdict names a free-text quiz that holds no answer.
    #[error("quiz `{quiz}` has no answer to mark")]
    NoAnswerYet {
        /// The id of the quiz.
        quiz: BlockId,
    },
}

/// A rule of a quiz that a student's answer breaks.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub(crate) enum Fault {
    /// The quiz is solved and takes no more answers.
    #[error("the quiz is solved")]
    Solved,
    /// The answer picks no option.
    #[error("the answer picks no option")]
    NoPick,
    /// The answer picks several options on a quiz with one right option.
    #[error("the quiz takes one pick")]
    OnePick,
    /// The answer picks an option the quiz does not have.
    #[error("the quiz has no option {pick}")]
    NoSuchOption {
        /// The index the answer named, from 0.
        pick: usize,
    },
    /// The answer picks an option that an earlier wrong pick disabled.
    #[error("option {pick} is disabled")]
    OptionDisabled {
        /// The index the answer named, from 0.
        pick: usize,
    },
    /// The answer holds no text.
    #[error("the answer is empty")]
    EmptyAnswer,
    /// The quiz holds an answer that waits for its verdict.
    #[error("the earlier answer awaits its verdict")]
    AwaitingVerdict,
}

/// Why a student's action was refused.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub(crate) enum Misstep {
    /// The action names an id that no quiz has.
    #[error("no quiz has the id `{quiz}`")]
    UnknownQuiz {
        /// The id the action named.
        quiz: BlockId,
    },
    /// The action sends text to a quiz that takes picks.
    #[error("quiz `{quiz}` takes picks")]
    TakesPicks {
        /// The id of the quiz.
        quiz: BlockId,
    },
    /// The action sends picks to a quiz that takes text.
    #[error("quiz `{quiz}` takes text")]
    TakesText {
        /// The id of the quiz.
        quiz: BlockId,
    },
    /// The answer breaks a rule of its quiz.
    #[error("the answer to quiz `{quiz}` breaks a rule")]
    Quiz {
        /// The id of the quiz.
        quiz: BlockId,
        /// The rule the answer breaks.
        #[source]
        fault: Fault,
    },
    /// The action names a page the lesson does not have.
    #[error("no page has the number {page}")]
    UnknownPage {
        /// The number the action named.
        page: PageNumber,
    },
    /// The page has no Continue button to press.
    #[error("page {page} has no Continue button")]
    NoContinue {
        /// The number of the page.
        page: PageNumber,
    },
    /// The Continue button belongs to a page that newer pages follow.
    #[error("page {page} is not the newest page")]
    NotNewest {
        /// The number of the page.
        page: PageNumber,
    },
    /// The Continue button was pressed already.
    #[error("the Continue button of page {page} was pressed already")]
    AlreadyPressed {
        /// The number of the page.
        page: PageNumber,
    },
}
