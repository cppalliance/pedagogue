//! Quiz blocks: what is stored, and the part of it the UI receives.
//!
//! A quiz keeps its answer key and every attempt. What the page shows is
//! worked out from those by the rules in `quiz_rules.rs`, so the key and the
//! feedback of options the student has yet to pick stay in this crate.

use serde::{Deserialize, Serialize, Serializer};

use super::block::{optional_text, text};
use super::command;
use super::quiz_rules::{self, ChoiceStatus, OptionState, ResponseStatus};
use super::refusal::Flaw;
use crate::event::Pick;

/// The fewest options a choice quiz holds.
const MIN_OPTIONS: usize = 2;
/// The most options a choice quiz holds.
const MAX_OPTIONS: usize = 6;

/// A multiple-choice quiz that the page marks at once.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ChoiceQuiz {
    pub(super) question: String,
    /// Whether the student selects all that apply, with checkboxes.
    pub(super) multiple: bool,
    pub(super) options: Vec<ChoiceOption>,
    /// Every attempt so far, oldest first.
    pub(super) attempts: Vec<ChoiceAttempt>,
}

/// One option of a choice quiz, with its part of the answer key.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ChoiceOption {
    pub(super) text: String,
    pub(super) correct: bool,
    /// What the student reads after picking this option.
    pub(super) feedback: Option<String>,
}

/// One attempt at a choice quiz.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ChoiceAttempt {
    /// The options the student picked, by index, in rising order.
    pub(super) picks: Vec<usize>,
    /// Whether the picks solved the quiz.
    pub(super) correct: bool,
}

impl ChoiceQuiz {
    /// Returns how many attempts the quiz has seen.
    pub(crate) fn attempts(&self) -> usize {
        self.attempts.len()
    }

    /// Records an attempt that the rules judged.
    pub(crate) fn record(&mut self, attempt: ChoiceAttempt) {
        self.attempts.push(attempt);
    }

    /// Names the options an attempt picked, as the page letters them.
    pub(crate) fn name_picks(&self, picks: &[usize]) -> Vec<Pick> {
        picks
            .iter()
            .filter_map(|&index| {
                let option = self.options.get(index)?;
                let letter = ('A'..='Z').nth(index)?;
                Some(Pick {
                    letter,
                    text: option.text.clone(),
                })
            })
            .collect()
    }
}

impl TryFrom<command::PopQuiz> for ChoiceQuiz {
    type Error = Flaw;

    fn try_from(args: command::PopQuiz) -> Result<Self, Flaw> {
        let actual = args.options.len();
        if !(MIN_OPTIONS..=MAX_OPTIONS).contains(&actual) {
            return Err(Flaw::Count {
                field: "options",
                min: MIN_OPTIONS,
                max: MAX_OPTIONS,
                actual,
            });
        }
        let multiple = args.multiple.unwrap_or(false);
        let right = args.options.iter().filter(|option| option.correct).count();
        if right == 0 || (!multiple && right > 1) {
            return Err(Flaw::RightOptions {
                multiple,
                actual: right,
            });
        }
        let options = args
            .options
            .iter()
            .map(|option| {
                Ok(ChoiceOption {
                    text: text("text", &option.text)?,
                    correct: option.correct,
                    feedback: optional_text("feedback", option.feedback.as_deref())?,
                })
            })
            .collect::<Result<_, _>>()?;
        Ok(Self {
            question: text("question", &args.question)?,
            multiple,
            options,
            attempts: Vec::new(),
        })
    }
}

/// The part of a choice quiz the UI receives.
#[derive(Serialize)]
struct ChoiceView<'a> {
    question: &'a str,
    multiple: bool,
    status: ChoiceStatus,
    attempts: usize,
    /// Whether the latest attempt left a right option unpicked.
    missing: bool,
    options: Vec<OptionView<'a>>,
    /// The feedback the latest attempt released.
    feedback: Vec<&'a str>,
}

#[derive(Serialize)]
struct OptionView<'a> {
    text: &'a str,
    state: OptionState,
    checked: bool,
}

impl Serialize for ChoiceQuiz {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let shown = quiz_rules::show_choice(self);
        let options = self
            .options
            .iter()
            .zip(&shown.options)
            .map(|(option, shown)| OptionView {
                text: &option.text,
                state: shown.state,
                checked: shown.checked,
            })
            .collect();
        ChoiceView {
            question: &self.question,
            multiple: self.multiple,
            status: shown.status,
            attempts: self.attempts.len(),
            missing: shown.missing,
            options,
            feedback: shown.feedback,
        }
        .serialize(serializer)
    }
}

/// The kind of a free-text quiz.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum ResponseKind {
    /// A one-line answer.
    ShortAnswer,
    /// A few paragraphs.
    ShortEssay,
    /// One line of code to edit and run.
    TryIt,
}

/// The verdict a host gives on a free-text answer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum Verdict {
    /// The answer is right.
    Correct,
    /// The answer is wrong, and the student may try again.
    Incorrect,
}

/// A free-text quiz that the host marks.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ResponseQuiz {
    pub(super) kind: ResponseKind,
    pub(super) question: String,
    /// The length a short essay aims for, in words.
    pub(super) words: Option<u32>,
    /// The line of code a try-it quiz starts from.
    pub(super) code: Option<String>,
    /// The language of that code.
    pub(super) language: Option<String>,
    /// Every answer so far, oldest first.
    pub(super) attempts: Vec<ResponseAttempt>,
}

/// One answer to a free-text quiz.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ResponseAttempt {
    pub(super) answer: String,
    /// The host's verdict and feedback, once it has marked the answer.
    pub(super) mark: Option<Mark>,
}

/// A host's verdict on one answer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Mark {
    pub(super) verdict: Verdict,
    pub(super) feedback: String,
}

impl ResponseQuiz {
    /// Returns the kind of the quiz.
    pub(crate) fn kind(&self) -> ResponseKind {
        self.kind
    }

    /// Returns how many answers the quiz has seen.
    pub(crate) fn attempts(&self) -> usize {
        self.attempts.len()
    }

    /// Records an answer that the rules accepted.
    pub(crate) fn record(&mut self, attempt: ResponseAttempt) {
        self.attempts.push(attempt);
    }

    /// Marks the latest answer. A later mark replaces an earlier one.
    pub(crate) fn mark(&mut self, verdict: Verdict, feedback: String) {
        if let Some(attempt) = self.attempts.last_mut() {
            attempt.mark = Some(Mark { verdict, feedback });
        }
    }

    /// Makes a quiz with no answer yet. `field` names the argument that
    /// holds the question.
    fn open(kind: ResponseKind, field: &'static str, question: &str) -> Result<Self, Flaw> {
        Ok(Self {
            kind,
            question: text(field, question)?,
            words: None,
            code: None,
            language: None,
            attempts: Vec::new(),
        })
    }
}

impl TryFrom<command::ShortAnswer> for ResponseQuiz {
    type Error = Flaw;

    fn try_from(args: command::ShortAnswer) -> Result<Self, Flaw> {
        Self::open(ResponseKind::ShortAnswer, "question", &args.question)
    }
}

impl TryFrom<command::ShortEssay> for ResponseQuiz {
    type Error = Flaw;

    fn try_from(args: command::ShortEssay) -> Result<Self, Flaw> {
        if args.words == 0 {
            return Err(Flaw::Zero { field: "words" });
        }
        Ok(Self {
            words: Some(args.words),
            ..Self::open(ResponseKind::ShortEssay, "question", &args.question)?
        })
    }
}

impl TryFrom<command::TryIt> for ResponseQuiz {
    type Error = Flaw;

    fn try_from(args: command::TryIt) -> Result<Self, Flaw> {
        Ok(Self {
            code: Some(text("code", &args.code)?),
            language: optional_text("language", args.language.as_deref())?,
            ..Self::open(ResponseKind::TryIt, "task", &args.task)?
        })
    }
}

/// The part of a free-text quiz the UI receives.
#[derive(Serialize)]
struct ResponseView<'a> {
    variant: ResponseKind,
    question: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    words: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language: Option<&'a str>,
    status: ResponseStatus,
    attempts: usize,
    /// The latest answer, as the student sent it.
    #[serde(skip_serializing_if = "Option::is_none")]
    answer: Option<&'a str>,
    /// The host's feedback on the latest answer.
    #[serde(skip_serializing_if = "Option::is_none")]
    feedback: Option<&'a str>,
}

impl Serialize for ResponseQuiz {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let latest = self.attempts.last();
        ResponseView {
            variant: self.kind,
            question: &self.question,
            words: self.words,
            code: self.code.as_deref(),
            language: self.language.as_deref(),
            status: quiz_rules::response_status(self),
            attempts: self.attempts.len(),
            answer: latest.map(|attempt| attempt.answer.as_str()),
            feedback: latest
                .and_then(|attempt| attempt.mark.as_ref())
                .map(|mark| mark.feedback.as_str()),
        }
        .serialize(serializer)
    }
}
