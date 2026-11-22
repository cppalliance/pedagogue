//! Why the lesson refuses a command.
//!
//! These types say what went wrong in a form the code can match on. The
//! sentence a model reads is written from them in `wording.rs`.

use thiserror::Error;

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
}
