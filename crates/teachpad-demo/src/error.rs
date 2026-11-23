//! The error of a script that cannot play.

use teachpad_session::{ActionError, CommandError};
use thiserror::Error;

/// A script that cannot be read, or a step of it that cannot be played.
///
/// `Display` names the step, counted from 1, and the cause is attached as a
/// source.
///
/// # Examples
///
/// ```
/// use teachpad_demo::Script;
///
/// let error = Script::parse(r#"[{ "tool": "new_page" }, { "shout": "Hello." }]"#)
///     .expect_err("the second step has a key that no step takes");
/// assert_eq!(error.to_string(), "step 2 is not a step");
/// ```
#[derive(Debug, Error)]
#[error(transparent)]
pub struct ScriptError(#[from] Problem);

/// What stops a script.
#[derive(Debug, Error)]
pub(crate) enum Problem {
    /// The text is not a JSON list.
    #[error("the script is not a JSON list of steps")]
    NotAList(#[source] serde_json::Error),
    /// A step has a key that no step takes, or a value of the wrong type.
    #[error("step {step} is not a step")]
    Misshapen {
        step: usize,
        #[source]
        source: serde_json::Error,
    },
    /// A step says nothing to do.
    #[error(
        "step {step} holds none of `tool`, `assistant`, `notice`, `suggestions`, `student`, `answer` and `wait`"
    )]
    Idle { step: usize },
    /// A step says two things to do.
    #[error("step {step} holds both `{first}` and `{second}`")]
    Double {
        step: usize,
        first: &'static str,
        second: &'static str,
    },
    /// A step holds a part of a page command, with the command left out.
    #[error("step {step} holds `{field}` without `tool`")]
    Stray { step: usize, field: &'static str },
    /// An answer step holds both picks and text, or neither.
    #[error("step {step} answers with neither picks alone nor text alone")]
    AnswerShape { step: usize },
    /// A step points at a quiz with text that is no id and no name.
    #[error("step {step} points at the quiz `{quiz}`, which is neither a block id nor an `@name`")]
    QuizRef { step: usize, quiz: String },
    /// A step uses a name that no earlier step gave to a block.
    #[error("step {step} uses `@{name}`, which no earlier step names with `as`")]
    UnknownName { step: usize, name: String },
    /// Two steps give the same name to their blocks.
    #[error("step {step} names its block `{name}`, and an earlier step took that name")]
    DuplicateName { step: usize, name: String },
    /// A step names the block of a command that adds none.
    #[error("step {step} names its block `{name}`, and `{tool}` adds no block")]
    NoBlock {
        step: usize,
        name: String,
        tool: String,
    },
    /// The session refused the page command of a step.
    #[error("step {step} (`{tool}`) was refused")]
    Refused {
        step: usize,
        tool: String,
        #[source]
        source: CommandError,
    },
    /// The session refused the student action of a step.
    #[error("step {step} was refused")]
    ActionRefused {
        step: usize,
        #[source]
        source: ActionError,
    },
    /// The session refused the stand-in's verdict on a live answer.
    #[error("the stand-in's verdict on quiz `{quiz}` was refused")]
    Verdict {
        quiz: String,
        #[source]
        source: CommandError,
    },
}
