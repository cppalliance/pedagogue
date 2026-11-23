//! The page commands: one argument struct for each tool a model can call.
//!
//! Each struct is the shape of one tool's arguments. Its doc comments are
//! written for a model to read, and they are the text a tool schema is made
//! from. Arguments are strict: a field that a struct lacks is refused.

use serde::Deserialize;
use serde::de::DeserializeOwned;
use serde_json::{Map, Value};

use super::block::Tone;
use super::id::BlockId;
use super::quiz::Verdict;
use super::refusal::Refusal;

/// The name of every page command.
pub(crate) const TOOLS: &[&str] = &[
    "new_page",
    "exposition",
    "footnote",
    "bar_chart",
    "tree_diagram",
    "timeline",
    "tip",
    "key_idea",
    "remember",
    "tailored_for_you",
    "hint",
    "milestone",
    "pop_quiz",
    "short_answer",
    "short_essay",
    "try_it",
    "quiz_result",
    "progress",
    "section_complete",
    "success",
    "continue_button",
];

/// One page command, parsed and ready to apply.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Command {
    NewPage(NewPage),
    Exposition(Exposition),
    Footnote(Footnote),
    BarChart(BarChart),
    TreeDiagram(TreeDiagram),
    Timeline(Timeline),
    Tip(Tip),
    KeyIdea(KeyIdea),
    Remember(Remember),
    TailoredForYou(TailoredForYou),
    Hint(Hint),
    Milestone(Milestone),
    PopQuiz(PopQuiz),
    ShortAnswer(ShortAnswer),
    ShortEssay(ShortEssay),
    TryIt(TryIt),
    QuizResult(QuizResult),
    Progress(Progress),
    SectionComplete(SectionComplete),
    Success(Success),
    ContinueButton(ContinueButton),
}

impl Command {
    /// Parses one tool call: the name of a tool and its JSON arguments.
    pub(crate) fn parse(tool: &str, args: &Value) -> Result<Self, Refusal> {
        let command = match tool {
            "new_page" => Self::NewPage(read(tool, args)?),
            "exposition" => Self::Exposition(read(tool, args)?),
            "footnote" => Self::Footnote(read(tool, args)?),
            "bar_chart" => Self::BarChart(read(tool, args)?),
            "tree_diagram" => Self::TreeDiagram(read(tool, args)?),
            "timeline" => Self::Timeline(read(tool, args)?),
            "tip" => Self::Tip(read(tool, args)?),
            "key_idea" => Self::KeyIdea(read(tool, args)?),
            "remember" => Self::Remember(read(tool, args)?),
            "tailored_for_you" => Self::TailoredForYou(read(tool, args)?),
            "hint" => Self::Hint(read(tool, args)?),
            "milestone" => Self::Milestone(read(tool, args)?),
            "pop_quiz" => Self::PopQuiz(read(tool, args)?),
            "short_answer" => Self::ShortAnswer(read(tool, args)?),
            "short_essay" => Self::ShortEssay(read(tool, args)?),
            "try_it" => Self::TryIt(read(tool, args)?),
            "quiz_result" => Self::QuizResult(read(tool, args)?),
            "progress" => Self::Progress(read(tool, args)?),
            "section_complete" => Self::SectionComplete(read(tool, args)?),
            "success" => Self::Success(read(tool, args)?),
            "continue_button" => Self::ContinueButton(read(tool, args)?),
            _ => {
                return Err(Refusal::UnknownTool {
                    tool: tool.to_owned(),
                });
            }
        };
        Ok(command)
    }

    /// Returns the name of the tool this command came from.
    pub(crate) fn tool(&self) -> &'static str {
        match self {
            Self::NewPage(_) => "new_page",
            Self::Exposition(_) => "exposition",
            Self::Footnote(_) => "footnote",
            Self::BarChart(_) => "bar_chart",
            Self::TreeDiagram(_) => "tree_diagram",
            Self::Timeline(_) => "timeline",
            Self::Tip(_) => "tip",
            Self::KeyIdea(_) => "key_idea",
            Self::Remember(_) => "remember",
            Self::TailoredForYou(_) => "tailored_for_you",
            Self::Hint(_) => "hint",
            Self::Milestone(_) => "milestone",
            Self::PopQuiz(_) => "pop_quiz",
            Self::ShortAnswer(_) => "short_answer",
            Self::ShortEssay(_) => "short_essay",
            Self::TryIt(_) => "try_it",
            Self::QuizResult(_) => "quiz_result",
            Self::Progress(_) => "progress",
            Self::SectionComplete(_) => "section_complete",
            Self::Success(_) => "success",
            Self::ContinueButton(_) => "continue_button",
        }
    }
}

/// Reads the arguments of one command.
fn read<T: DeserializeOwned>(tool: &str, args: &Value) -> Result<T, Refusal> {
    T::deserialize(args).map_err(|source| Refusal::InvalidArgs {
        tool: tool.to_owned(),
        field: faulty_field::<T>(args, &source),
        source,
    })
}

/// Finds the top-level field whose value the parser refused.
///
/// The parser reports a wrong type without the name of its field. A probe
/// that holds one field alone fails the same way only for the field at
/// fault, so the first such field is the answer.
fn faulty_field<T: DeserializeOwned>(args: &Value, error: &serde_json::Error) -> Option<String> {
    let message = error.to_string();
    // The parser names a missing or an unknown field itself, and a probe of
    // one field would only report the same missing field again.
    if message.starts_with("missing field") || message.starts_with("unknown field") {
        return None;
    }
    args.as_object()?.iter().find_map(|(name, value)| {
        let probe = Value::Object(Map::from_iter([(name.clone(), value.clone())]));
        let found = T::deserialize(&probe).err()?;
        (found.to_string() == message).then(|| name.clone())
    })
}

/// Starts the next page of the lesson and shows it to the student.
///
/// Call this first: every other command adds to the newest page. A page is
/// one section of a chapter, drawn as a single column that scrolls.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct NewPage {
    /// The number of the chapter this page belongs to, such as 3. Leave it
    /// out to stay in the chapter of the previous page.
    pub(crate) chapter_number: Option<u32>,
    /// The title of the chapter, such as "Chairing a WG21 Room". Leave it out
    /// to stay in the chapter of the previous page.
    pub(crate) chapter_title: Option<String>,
    /// The number of the section, as text, such as "3.2".
    pub(crate) section_number: Option<String>,
    /// The title of the section, such as "Taking a Straw Poll". It becomes
    /// the heading at the top of the page.
    pub(crate) section_title: Option<String>,
}

/// Adds running text to the page: paragraphs, lists, tables, code blocks,
/// quotes and sub-headings.
///
/// Write the text in markdown. Use `##` and `###` for sub-headings, a fenced
/// block with a language name for code, and `[^1]` where the marker of
/// footnote 1 belongs. A single plain paragraph is an `exposition` call too.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Exposition {
    /// The text, in markdown.
    pub(crate) markdown: String,
}

/// Adds a numbered note at the foot of the page.
///
/// Notes are numbered from 1 on each page, in the order they are added. Put
/// `[^1]` in exposition text where the marker of note 1 belongs.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Footnote {
    /// The note, in markdown: one short paragraph.
    pub(crate) markdown: String,
}

/// Adds a bar chart as a numbered figure, such as the result of a poll.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct BarChart {
    /// The caption under the chart, in markdown: one short line.
    pub(crate) caption: String,
    /// The bars from left to right. A chart holds 1 to 12 bars.
    pub(crate) bars: Vec<Bar>,
}

/// One bar of a bar chart.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Bar {
    /// The label under the bar, such as "SF".
    pub(crate) label: String,
    /// The height of the bar: a number of zero or more.
    pub(crate) value: f64,
    /// The color of the bar: `accent` for what the chart is about, `neutral`
    /// for the middle ground, or `negative` for the other side. Leave it out
    /// for `accent`.
    pub(crate) tone: Option<Tone>,
}

/// Adds a tree diagram as a numbered figure, such as a family of terms.
///
/// The tree is drawn in rows, the root row first. Each label names one
/// node, so a node with two parents appears once and gets two edges.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct TreeDiagram {
    /// The caption under the diagram, in markdown: one short line.
    pub(crate) caption: String,
    /// The rows from top to bottom, each a list of labels from left to
    /// right. A tree holds 1 to 6 rows of 1 to 8 labels.
    pub(crate) rows: Vec<Vec<String>>,
    /// The lines of the tree, each from a label to a label in a lower row.
    pub(crate) edges: Vec<Edge>,
}

/// One line of a tree diagram.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Edge {
    /// The label of the upper node.
    pub(crate) from: String,
    /// The label of the lower node.
    pub(crate) to: String,
}

/// Adds a timeline: a row of stops joined by a line, such as the phases of
/// a discussion.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Timeline {
    /// The stops from left to right. A timeline holds 2 to 8 stops.
    pub(crate) stops: Vec<Stop>,
}

/// One stop of a timeline.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Stop {
    /// The name of the stop, such as "Discuss".
    pub(crate) label: String,
    /// A short note under the stop, such as "~ half time". Leave it out for
    /// a bare stop.
    pub(crate) note: Option<String>,
}

/// Adds a blue information box with a practical tip.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Tip {
    /// The tip, in markdown.
    pub(crate) markdown: String,
    /// The title of the box, such as "Chair's Tip". Leave it out for "Tip".
    pub(crate) title: Option<String>,
}

/// Adds a blue information box that states the one idea to take away.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct KeyIdea {
    /// The idea, in markdown: a sentence or two.
    pub(crate) markdown: String,
}

/// Adds a blue information box with a fact worth keeping in mind.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Remember {
    /// The fact, in markdown.
    pub(crate) markdown: String,
}

/// Adds a yellow personal box that fits the lesson to this student.
///
/// Use what you know about the student: their background, their goals and
/// their earlier answers.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct TailoredForYou {
    /// The note to the student, in markdown.
    pub(crate) markdown: String,
}

/// Adds a yellow personal box with a hint that builds on what this student
/// already knows.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Hint {
    /// The hint, in markdown.
    pub(crate) markdown: String,
}

/// Adds a yellow personal box that names the student's next milestone.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Milestone {
    /// The number of the milestone, such as 2.
    pub(crate) number: u32,
    /// What the student does to reach it, in markdown.
    pub(crate) markdown: String,
}

/// Adds a multiple-choice quiz that the page marks at once.
///
/// The student picks an option and presses Check answer. A wrong pick is
/// disabled and shows its feedback, and the student picks again from the
/// options that remain, so the right answer stays hidden until the student
/// finds it. Every attempt, right or wrong, comes back to you as a student
/// event.
///
/// Write the feedback of a wrong option so that it explains the mistake and
/// keeps the right answer to itself.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PopQuiz {
    /// The question, in markdown: one or two lines.
    pub(crate) question: String,
    /// The options, in the order shown. A quiz holds 2 to 6 options.
    pub(crate) options: Vec<QuizOption>,
    /// Set this to true for a select-all-that-apply quiz with checkboxes.
    /// Leave it out for a quiz with one right option.
    pub(crate) multiple: Option<bool>,
}

/// One option of a multiple-choice quiz.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct QuizOption {
    /// The text of the option, in markdown: a few words.
    pub(crate) text: String,
    /// Whether this option is right. A quiz with one right option marks
    /// exactly one option, and a select-all quiz marks one or more.
    pub(crate) correct: bool,
    /// What the student reads after picking this option, in markdown. For a
    /// wrong option, explain the mistake and keep the right answer back.
    /// Leave it out for an option that needs no comment.
    pub(crate) feedback: Option<String>,
}

/// Adds a one-line free-text question, titled "Check Your Understanding".
///
/// The student's answer comes back to you as a student event. Mark it with
/// `quiz_result`.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ShortAnswer {
    /// The question, in markdown.
    pub(crate) question: String,
}

/// Adds a free-text question with room for a few paragraphs.
///
/// The student's essay comes back to you as a student event. Mark it with
/// `quiz_result`.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ShortEssay {
    /// The question, in markdown.
    pub(crate) question: String,
    /// The length to aim for, in words, such as 100.
    pub(crate) words: u32,
}

/// Adds a one-line code exercise: the student edits a line of code and
/// presses Run.
///
/// The edited line comes back to you as a student event. Mark it with
/// `quiz_result`.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct TryIt {
    /// What to change, in markdown.
    pub(crate) task: String,
    /// The line of code the student starts from.
    pub(crate) code: String,
    /// The language of the code, such as "cpp". Leave it out for plain
    /// text.
    pub(crate) language: Option<String>,
}

/// Marks a free-text answer and shows your feedback under it.
///
/// Use it after the student answered a `short_answer`, `short_essay` or
/// `try_it` quiz. A verdict of `incorrect` lets the student try again.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct QuizResult {
    /// The id of the quiz, from the receipt that added it, such as "b9".
    pub(crate) quiz: BlockId,
    /// Your verdict: `correct` or `incorrect`.
    pub(crate) verdict: Verdict,
    /// What the student reads under the answer, in markdown: a line or two.
    pub(crate) feedback: String,
}

/// Adds a green progress box with a bar.
///
/// Use it rarely: green marks real progress.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Progress {
    /// The count of finished parts.
    pub(crate) done: u32,
    /// The count of all parts: 1 or more.
    pub(crate) total: u32,
    /// The line under the bar, in markdown, such as "Milestone 8 of 12".
    pub(crate) label: String,
}

/// Adds a green box that marks the end of a section.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct SectionComplete {
    /// What the student finished, in markdown.
    pub(crate) markdown: String,
}

/// Adds a green one-line strip that praises a success, such as a solved
/// quiz.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Success {
    /// One short line, in markdown.
    pub(crate) text: String,
}

/// Sets the button at the foot of the page that the student presses to
/// move on.
///
/// Calling it again replaces the label. A press arrives as a student event,
/// and the usual answer to it is `new_page`.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ContinueButton {
    /// The label of the button, such as "Continue to 3.3".
    pub(crate) label: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde_json::json;

    fn refusal_of(tool: &str, args: &Value) -> Refusal {
        Command::parse(tool, args).expect_err("the call is refused")
    }

    #[test]
    fn every_listed_tool_parses_its_own_arguments() {
        let calls = [
            (
                "new_page",
                json!({ "chapter_number": 3, "section_title": "Polls" }),
            ),
            ("exposition", json!({ "markdown": "Text." })),
            ("footnote", json!({ "markdown": "A note." })),
            (
                "bar_chart",
                json!({ "caption": "A poll", "bars": [{ "label": "SF", "value": 10, "tone": "accent" }] }),
            ),
            (
                "tree_diagram",
                json!({ "caption": "Kinds", "rows": [["a"], ["b"]], "edges": [{ "from": "a", "to": "b" }] }),
            ),
            (
                "timeline",
                json!({ "stops": [{ "label": "Start" }, { "label": "End", "note": "late" }] }),
            ),
            (
                "tip",
                json!({ "markdown": "Watch the clock.", "title": "Chair's Tip" }),
            ),
            ("key_idea", json!({ "markdown": "One idea." })),
            ("remember", json!({ "markdown": "One fact." })),
            ("tailored_for_you", json!({ "markdown": "For you." })),
            ("hint", json!({ "markdown": "A hint." })),
            (
                "milestone",
                json!({ "number": 2, "markdown": "Finish the essay." }),
            ),
            (
                "pop_quiz",
                json!({ "question": "Which?", "options": [
                    { "text": "This", "correct": true, "feedback": "Yes." },
                    { "text": "That", "correct": false },
                ] }),
            ),
            ("short_answer", json!({ "question": "Why?" })),
            ("short_essay", json!({ "question": "Why?", "words": 100 })),
            (
                "try_it",
                json!({ "task": "Double it.", "code": "x + 1", "language": "cpp" }),
            ),
            (
                "quiz_result",
                json!({ "quiz": "b9", "verdict": "correct", "feedback": "Exactly." }),
            ),
            (
                "progress",
                json!({ "done": 8, "total": 12, "label": "8 of 12" }),
            ),
            ("section_complete", json!({ "markdown": "Done." })),
            ("success", json!({ "text": "Nice work." })),
            ("continue_button", json!({ "label": "Continue to 3.3" })),
        ];
        let parsed: Vec<&str> = calls
            .iter()
            .map(|(tool, args)| Command::parse(tool, args).expect("the call parses").tool())
            .collect();
        assert_eq!(parsed, TOOLS);
    }

    #[test]
    fn an_unknown_tool_is_refused_by_name() {
        let refusal = refusal_of("paragraph", &json!({}));
        assert!(matches!(refusal, Refusal::UnknownTool { tool } if tool == "paragraph"));
    }

    #[test]
    fn an_unknown_field_is_refused() {
        let refusal = refusal_of(
            "exposition",
            &json!({ "markdown": "Text.", "colour": "red" }),
        );
        let Refusal::InvalidArgs {
            tool,
            field,
            source,
        } = refusal
        else {
            panic!("expected invalid arguments");
        };
        assert_eq!(tool, "exposition");
        assert_eq!(field, None);
        assert_eq!(
            source.to_string(),
            "unknown field `colour`, expected `markdown`"
        );
    }

    #[test]
    fn a_missing_field_is_refused() {
        let refusal = refusal_of("tip", &json!({ "title": "Chair's Tip" }));
        let Refusal::InvalidArgs { field, source, .. } = refusal else {
            panic!("expected invalid arguments");
        };
        assert_eq!(field, None);
        assert_eq!(source.to_string(), "missing field `markdown`");
    }

    #[test]
    fn a_wrong_type_names_its_field() {
        let refusal = refusal_of(
            "milestone",
            &json!({ "number": "two", "markdown": "Finish." }),
        );
        let Refusal::InvalidArgs { field, source, .. } = refusal else {
            panic!("expected invalid arguments");
        };
        assert_eq!(field.as_deref(), Some("number"));
        assert_eq!(
            source.to_string(),
            "invalid type: string \"two\", expected u32"
        );
    }

    #[test]
    fn a_wrong_value_deep_in_a_list_names_the_list() {
        let args =
            json!({ "caption": "A poll", "bars": [{ "label": "SF", "value": 1, "tone": "blue" }] });
        let Refusal::InvalidArgs { field, .. } = refusal_of("bar_chart", &args) else {
            panic!("expected invalid arguments");
        };
        assert_eq!(field.as_deref(), Some("bars"));
    }

    #[test]
    fn arguments_that_are_not_an_object_are_refused() {
        let Refusal::InvalidArgs { field, .. } = refusal_of("exposition", &json!("Text.")) else {
            panic!("expected invalid arguments");
        };
        assert_eq!(field, None);
    }
}
