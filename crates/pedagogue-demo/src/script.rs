//! A sample script: a JSON list of steps to play into a session.

use std::collections::BTreeSet;

use pedagogue_session::BlockId;
use serde::Deserialize;
use serde_json::{Map, Value};

use crate::error::{Problem, ScriptError};

/// The built-in sample script, as JSON text: a two-page lesson on taking a
/// straw poll, with a wait for Continue between the pages.
///
/// # Examples
///
/// ```
/// use pedagogue_demo::{STRAW_POLL, Script};
///
/// let script = Script::parse(STRAW_POLL)?;
/// # let _ = script;
/// # Ok::<(), pedagogue_demo::ScriptError>(())
/// ```
pub const STRAW_POLL: &str = include_str!("../../../samples/straw-poll.json");

/// A sample script: a list of steps to play into a session.
///
/// A script is a JSON list, and each step is an object of one of these
/// shapes:
///
/// - `{ "tool": "pop_quiz", "args": { ... }, "as": "poll" }` is a page
///   command. `args` may be left out when the command takes none. The
///   optional `as` names the block the command adds, and a later step
///   writes `"@poll"` where a quiz id goes.
/// - `{ "assistant": "..." }`, `{ "notice": "..." }` and
///   `{ "suggestions": ["..."] }` write into the chat.
/// - `{ "student": "..." }` is a message from the student, as if typed.
/// - `{ "answer": { "quiz": "@poll", "picks": [1] } }` and
///   `{ "answer": { "quiz": "@essay", "text": "..." } }` answer a quiz for
///   the student.
/// - `{ "wait": "continue" }` holds the script until the student presses
///   Continue.
///
/// # Examples
///
/// ```
/// use pedagogue_demo::Script;
///
/// let script = Script::parse(
///     r#"[
///         { "tool": "new_page", "args": { "section_title": "Taking a Straw Poll" } },
///         { "tool": "short_answer", "args": { "question": "Why five columns?" }, "as": "why" },
///         { "answer": { "quiz": "@why", "text": "To show how strong each view is." } },
///         { "tool": "quiz_result",
///           "args": { "quiz": "@why", "verdict": "correct", "feedback": "Exactly." } }
///     ]"#,
/// )?;
/// # let _ = script;
/// # Ok::<(), pedagogue_demo::ScriptError>(())
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Script {
    pub(crate) steps: Vec<Step>,
}

impl Script {
    /// Reads a script from JSON text.
    ///
    /// # Errors
    ///
    /// Returns a [`ScriptError`] that names the step when the text is not a
    /// JSON list, when a step has none or several of the shapes a step can
    /// have, or when a step uses an `@name` that no earlier step gave to a
    /// block.
    ///
    /// # Examples
    ///
    /// ```
    /// use pedagogue_demo::Script;
    ///
    /// let error = Script::parse(r#"[{ "answer": { "quiz": "@poll", "picks": [0] } }]"#)
    ///     .expect_err("no step names a block `poll`");
    /// assert_eq!(
    ///     error.to_string(),
    ///     "step 1 uses `@poll`, which no earlier step names with `as`",
    /// );
    /// ```
    pub fn parse(json: &str) -> Result<Self, ScriptError> {
        let raw: Vec<Value> = serde_json::from_str(json).map_err(Problem::NotAList)?;
        let mut names = BTreeSet::new();
        let mut steps = Vec::with_capacity(raw.len());
        for (index, value) in raw.into_iter().enumerate() {
            let step = index + 1;
            let raw: RawStep = serde_json::from_value(value)
                .map_err(|source| Problem::Misshapen { step, source })?;
            let parsed = raw.into_step(step)?;
            parsed.check_names(step, &mut names)?;
            steps.push(parsed);
        }
        Ok(Self { steps })
    }
}

/// One step of a script.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Step {
    /// A page command, with the name it gives the block it adds.
    Command {
        tool: String,
        args: Value,
        name: Option<String>,
    },
    /// A tutor reply.
    Assistant(String),
    /// A centered line in the chat.
    Notice(String),
    /// The chips above the composer.
    Suggestions(Vec<String>),
    /// A message from the student.
    Student(String),
    /// A student's answer to a quiz.
    Answer { quiz: QuizRef, answer: Answer },
    /// A hold until the student does something.
    Wait(Wait),
}

/// How a step points at a quiz.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum QuizRef {
    /// By the name an earlier step gave it with `as`.
    Name(String),
    /// By its id.
    Id(BlockId),
}

/// What a scripted student answers.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Answer {
    /// Picks on a multiple-choice quiz, as option indexes from 0.
    Picks(Vec<usize>),
    /// Text for a free-text quiz.
    Text(String),
}

/// What a script waits for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum Wait {
    /// A press of the Continue button.
    Continue,
}

impl Step {
    /// Checks the names this step uses and records the one it gives.
    fn check_names(&self, step: usize, names: &mut BTreeSet<String>) -> Result<(), Problem> {
        let used = match self {
            Self::Command { args, .. } => named_quiz(args),
            Self::Answer {
                quiz: QuizRef::Name(name),
                ..
            } => Some(name.as_str()),
            _ => None,
        };
        if let Some(name) = used
            && !names.contains(name)
        {
            return Err(Problem::UnknownName {
                step,
                name: name.to_owned(),
            });
        }
        if let Self::Command {
            name: Some(name), ..
        } = self
            && !names.insert(name.clone())
        {
            return Err(Problem::DuplicateName {
                step,
                name: name.clone(),
            });
        }
        Ok(())
    }
}

/// Returns the name in the `quiz` argument of a page command, when the
/// argument is an `@name`.
pub(crate) fn named_quiz(args: &Value) -> Option<&str> {
    args.get("quiz")?.as_str()?.strip_prefix('@')
}

/// A step as it is written, before its shape is checked.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawStep {
    tool: Option<String>,
    args: Option<Map<String, Value>>,
    #[serde(rename = "as")]
    name: Option<String>,
    assistant: Option<String>,
    notice: Option<String>,
    suggestions: Option<Vec<String>>,
    student: Option<String>,
    answer: Option<RawAnswer>,
    wait: Option<Wait>,
}

/// An answer as it is written.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawAnswer {
    quiz: String,
    picks: Option<Vec<usize>>,
    text: Option<String>,
}

impl RawStep {
    /// Turns the written step into the one thing it says to do.
    fn into_step(self, step: usize) -> Result<Step, Problem> {
        let held = [
            ("tool", self.tool.is_some()),
            ("assistant", self.assistant.is_some()),
            ("notice", self.notice.is_some()),
            ("suggestions", self.suggestions.is_some()),
            ("student", self.student.is_some()),
            ("answer", self.answer.is_some()),
            ("wait", self.wait.is_some()),
        ];
        let mut verbs = held
            .into_iter()
            .filter_map(|(verb, is_held)| is_held.then_some(verb));
        let first = verbs.next().ok_or(Problem::Idle { step })?;
        if let Some(second) = verbs.next() {
            return Err(Problem::Double {
                step,
                first,
                second,
            });
        }
        if self.tool.is_none() {
            let stray = [("args", self.args.is_some()), ("as", self.name.is_some())];
            if let Some((field, _)) = stray.into_iter().find(|(_, is_held)| *is_held) {
                return Err(Problem::Stray { step, field });
            }
        }
        let parsed = match self {
            Self {
                tool: Some(tool),
                args,
                name,
                ..
            } => Step::Command {
                tool,
                args: Value::Object(args.unwrap_or_default()),
                name,
            },
            Self {
                assistant: Some(text),
                ..
            } => Step::Assistant(text),
            Self {
                notice: Some(text), ..
            } => Step::Notice(text),
            Self {
                suggestions: Some(chips),
                ..
            } => Step::Suggestions(chips),
            Self {
                student: Some(text),
                ..
            } => Step::Student(text),
            Self {
                answer: Some(answer),
                ..
            } => answer.into_step(step)?,
            Self {
                wait: Some(wait), ..
            } => Step::Wait(wait),
            _ => return Err(Problem::Idle { step }),
        };
        Ok(parsed)
    }
}

impl RawAnswer {
    fn into_step(self, step: usize) -> Result<Step, Problem> {
        let answer = match (self.picks, self.text) {
            (Some(picks), None) => Answer::Picks(picks),
            (None, Some(text)) => Answer::Text(text),
            _ => return Err(Problem::AnswerShape { step }),
        };
        let quiz = match self.quiz.strip_prefix('@') {
            Some(name) => QuizRef::Name(name.to_owned()),
            None => match serde_json::from_value(Value::String(self.quiz.clone())) {
                Ok(id) => QuizRef::Id(id),
                Err(_) => {
                    return Err(Problem::QuizRef {
                        step,
                        quiz: self.quiz,
                    });
                }
            },
        };
        Ok(Step::Answer { quiz, answer })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::error::Error;

    use serde_json::json;

    fn problem_of(script: &Value) -> String {
        Script::parse(&script.to_string())
            .expect_err("the script has a problem")
            .to_string()
    }

    #[test]
    fn a_script_holds_one_step_of_each_shape() {
        let script = json!([
            { "tool": "new_page" },
            { "tool": "pop_quiz", "args": { "question": "Which?" }, "as": "poll" },
            { "assistant": "Read this section." },
            { "notice": "**Milestone 2 unlocked**" },
            { "suggestions": ["Take the quiz"] },
            { "student": "Why five columns?" },
            { "answer": { "quiz": "@poll", "picks": [1] } },
            { "answer": { "quiz": "b7", "text": "Because." } },
            { "tool": "quiz_result", "args": { "quiz": "@poll" } },
            { "wait": "continue" },
        ]);
        let parsed = Script::parse(&script.to_string()).expect("the script parses");
        let b7 = serde_json::from_value(json!("b7")).expect("b7 is a block id");
        assert_eq!(
            parsed.steps,
            [
                Step::Command {
                    tool: "new_page".to_owned(),
                    args: json!({}),
                    name: None
                },
                Step::Command {
                    tool: "pop_quiz".to_owned(),
                    args: json!({ "question": "Which?" }),
                    name: Some("poll".to_owned()),
                },
                Step::Assistant("Read this section.".to_owned()),
                Step::Notice("**Milestone 2 unlocked**".to_owned()),
                Step::Suggestions(vec!["Take the quiz".to_owned()]),
                Step::Student("Why five columns?".to_owned()),
                Step::Answer {
                    quiz: QuizRef::Name("poll".to_owned()),
                    answer: Answer::Picks(vec![1]),
                },
                Step::Answer {
                    quiz: QuizRef::Id(b7),
                    answer: Answer::Text("Because.".to_owned()),
                },
                Step::Command {
                    tool: "quiz_result".to_owned(),
                    args: json!({ "quiz": "@poll" }),
                    name: None,
                },
                Step::Wait(Wait::Continue),
            ],
        );
    }

    #[test]
    fn the_built_in_sample_parses() {
        let script = Script::parse(STRAW_POLL).expect("the built-in sample parses");
        assert!(script.steps.contains(&Step::Wait(Wait::Continue)));
    }

    #[test]
    fn text_that_is_no_list_is_refused_with_its_cause() {
        let error = Script::parse("{ \"tool\": \"new_page\" }").expect_err("an object is no list");
        assert_eq!(error.to_string(), "the script is not a JSON list of steps");
        assert!(error.source().is_some());
    }

    #[test]
    fn a_step_of_no_known_shape_is_named_by_its_number() {
        let cases = [
            (
                json!([{ "tool": "new_page" }, {}]),
                "step 2 holds none of `tool`, `assistant`, `notice`, `suggestions`, `student`, `answer` and `wait`",
            ),
            (json!([{ "shout": "Hello." }]), "step 1 is not a step"),
            (json!([{ "assistant": 7 }]), "step 1 is not a step"),
            (json!([{ "wait": "forever" }]), "step 1 is not a step"),
            (
                json!([{ "tool": "tip", "args": "Text." }]),
                "step 1 is not a step",
            ),
            (
                json!([{ "assistant": "One.", "notice": "Two." }]),
                "step 1 holds both `assistant` and `notice`",
            ),
            (
                json!([{ "assistant": "One.", "as": "name" }]),
                "step 1 holds `as` without `tool`",
            ),
            (
                json!([{ "student": "One.", "args": {} }]),
                "step 1 holds `args` without `tool`",
            ),
            (
                json!([{ "answer": { "quiz": "b1" } }]),
                "step 1 answers with neither picks alone nor text alone",
            ),
            (
                json!([{ "answer": { "quiz": "b1", "picks": [0], "text": "Both." } }]),
                "step 1 answers with neither picks alone nor text alone",
            ),
            (
                json!([{ "answer": { "quiz": "poll", "picks": [0] } }]),
                "step 1 points at the quiz `poll`, which is neither a block id nor an `@name`",
            ),
        ];
        for (script, expected) in cases {
            assert_eq!(problem_of(&script), expected);
        }
    }

    #[test]
    fn a_name_is_given_once_and_used_only_after() {
        let quiz = |name: &str| json!({ "tool": "pop_quiz", "as": name });
        assert_eq!(
            problem_of(&json!([{ "answer": { "quiz": "@poll", "picks": [0] } }, quiz("poll")])),
            "step 1 uses `@poll`, which no earlier step names with `as`",
        );
        assert_eq!(
            problem_of(
                &json!([quiz("poll"), { "tool": "quiz_result", "args": { "quiz": "@essay" } }])
            ),
            "step 2 uses `@essay`, which no earlier step names with `as`",
        );
        assert_eq!(
            problem_of(&json!([quiz("poll"), quiz("poll")])),
            "step 2 names its block `poll`, and an earlier step took that name",
        );
    }
}
