//! Text a model reads: the receipt of each command and the sentence of each
//! refusal.
//!
//! Every sentence is short and stands on its own, and a refusal says what to
//! fix. This file is the one home of that wording in this crate.

use crate::lesson::{Flaw, Receipt, Refusal, TOOLS};

/// Writes the receipt a model reads after a command applied.
///
/// A receipt for a new block names the block's id, which is how a later
/// command points at it.
pub(crate) fn receipt(receipt: &Receipt) -> String {
    match receipt {
        Receipt::StartedPage { page } => format!("Started page {page}."),
        Receipt::AddedBlock { tool, id, page } => {
            format!("Added {} {id} to page {page}.", noun(tool))
        }
        Receipt::AddedFootnote { number, page } => {
            format!("Added footnote {number} to page {page}.")
        }
        Receipt::SetContinue { label, page } => {
            format!("Set the Continue button of page {page} to \"{label}\".")
        }
    }
}

/// Names the block a command adds.
fn noun(tool: &str) -> &str {
    match tool {
        "bar_chart" => "bar chart",
        "tree_diagram" => "tree diagram",
        "key_idea" => "key idea",
        "remember" => "remember box",
        "tailored_for_you" => "tailored-for-you box",
        "progress" => "progress box",
        "section_complete" => "section-complete box",
        "success" => "success strip",
        plain => plain,
    }
}

/// Writes the sentence a model reads after a command was refused.
pub(crate) fn refusal(refusal: &Refusal) -> String {
    match refusal {
        Refusal::UnknownTool { tool } => {
            format!(
                "Unknown tool \"{tool}\". The page tools are: {}.",
                TOOLS.join(", ")
            )
        }
        Refusal::InvalidArgs {
            tool,
            field: Some(field),
            source,
        } => format!(
            "Invalid arguments for {tool}, in \"{field}\": {source}. Fix that field and call {tool} again."
        ),
        Refusal::InvalidArgs {
            tool,
            field: None,
            source,
        } => format!("Invalid arguments for {tool}: {source}. Fix them and call {tool} again."),
        Refusal::NoPage { .. } => "No page yet. Call new_page first.".to_owned(),
        Refusal::Flawed { tool, flaw } => flaw_sentence(tool, flaw),
    }
}

fn flaw_sentence(tool: &str, flaw: &Flaw) -> String {
    match flaw {
        Flaw::Empty { field } => format!("The \"{field}\" of {tool} is empty. Give it text."),
        Flaw::Count {
            field,
            min,
            max,
            actual,
        } => format!("{tool} takes {min} to {max} {field}, and this call has {actual}."),
        Flaw::Zero { field } => format!("The \"{field}\" of {tool} is 0. Make it 1 or more."),
        Flaw::NegativeBar { label } => format!(
            "Bar \"{label}\" has a value below zero. Give each bar a value of zero or more."
        ),
        Flaw::DuplicateLabel { label } => format!(
            "The label \"{label}\" appears twice in the tree. Give each node a label of its own."
        ),
        Flaw::UnknownLabel { label } => format!(
            "An edge names \"{label}\", which is in none of the rows. Use the labels from \"rows\"."
        ),
        Flaw::EdgeNotDownward { from, to } => format!(
            "The edge from \"{from}\" to \"{to}\" must end in a lower row than it starts in."
        ),
        Flaw::DoneOverTotal { done, total } => format!(
            "\"done\" is {done} and \"total\" is {total}. Keep \"done\" at or under \"total\"."
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde_json::{Value, json};

    use crate::lesson::{Command, Lesson};

    /// Applies a list of calls to a fresh lesson and words each outcome.
    fn replies(calls: &[(&str, Value)]) -> Vec<String> {
        let mut lesson = Lesson::default();
        calls
            .iter()
            .map(|(tool, args)| {
                let outcome = Command::parse(tool, args).and_then(|command| lesson.apply(command));
                match outcome {
                    Ok(done) => receipt(&done),
                    Err(refused) => refusal(&refused),
                }
            })
            .collect()
    }

    #[test]
    fn a_receipt_is_one_sentence_that_names_what_was_added() {
        let said = replies(&[
            ("new_page", json!({})),
            ("exposition", json!({ "markdown": "Text." })),
            ("tailored_for_you", json!({ "markdown": "For you." })),
            (
                "bar_chart",
                json!({ "caption": "A poll", "bars": [{ "label": "SF", "value": 1 }] }),
            ),
            ("success", json!({ "text": "Nice work." })),
            ("footnote", json!({ "markdown": "A note." })),
            ("continue_button", json!({ "label": "Continue to 3.3" })),
        ]);
        assert_eq!(
            said,
            [
                "Started page 1.",
                "Added exposition b1 to page 1.",
                "Added tailored-for-you box b2 to page 1.",
                "Added bar chart b3 to page 1.",
                "Added success strip b4 to page 1.",
                "Added footnote 1 to page 1.",
                "Set the Continue button of page 1 to \"Continue to 3.3\".",
            ],
        );
    }

    #[test]
    fn a_command_before_the_first_page_is_told_to_start_one() {
        let said = replies(&[("exposition", json!({ "markdown": "Text." }))]);
        assert_eq!(said, ["No page yet. Call new_page first."]);
    }

    #[test]
    fn an_unknown_tool_is_told_the_tools_there_are() {
        let said = replies(&[("paragraph", json!({}))]);
        assert_eq!(
            said,
            [
                "Unknown tool \"paragraph\". The page tools are: new_page, exposition, footnote, \
              bar_chart, tree_diagram, timeline, tip, key_idea, remember, tailored_for_you, hint, \
              milestone, progress, section_complete, success, continue_button."
            ],
        );
    }

    #[test]
    fn arguments_of_the_wrong_shape_are_told_what_to_fix() {
        let said = replies(&[
            (
                "exposition",
                json!({ "markdown": "Text.", "colour": "red" }),
            ),
            (
                "milestone",
                json!({ "number": "two", "markdown": "Finish." }),
            ),
        ]);
        assert_eq!(
            said,
            [
                "Invalid arguments for exposition: unknown field `colour`, expected `markdown`. \
                 Fix them and call exposition again.",
                "Invalid arguments for milestone, in \"number\": invalid type: string \"two\", \
                 expected u32. Fix that field and call milestone again.",
            ],
        );
    }

    #[test]
    fn a_broken_rule_is_told_in_one_sentence() {
        let tree = |rows: Value, edges: Value| {
            (
                "tree_diagram",
                json!({ "caption": "Kinds", "rows": rows, "edges": edges }),
            )
        };
        let said = replies(&[
            ("new_page", json!({})),
            ("exposition", json!({ "markdown": " " })),
            ("timeline", json!({ "stops": [{ "label": "Start" }] })),
            ("progress", json!({ "done": 0, "total": 0, "label": "x" })),
            ("progress", json!({ "done": 5, "total": 4, "label": "x" })),
            (
                "bar_chart",
                json!({ "caption": "A poll", "bars": [{ "label": "A", "value": -1 }] }),
            ),
            tree(json!([["a"], ["a"]]), json!([])),
            tree(json!([["a"], ["b"]]), json!([{ "from": "a", "to": "c" }])),
            tree(json!([["a"], ["b"]]), json!([{ "from": "b", "to": "a" }])),
        ]);
        assert_eq!(
            said[1..],
            [
                "The \"markdown\" of exposition is empty. Give it text.",
                "timeline takes 2 to 8 stops, and this call has 1.",
                "The \"total\" of progress is 0. Make it 1 or more.",
                "\"done\" is 5 and \"total\" is 4. Keep \"done\" at or under \"total\".",
                "Bar \"A\" has a value below zero. Give each bar a value of zero or more.",
                "The label \"a\" appears twice in the tree. Give each node a label of its own.",
                "An edge names \"c\", which is in none of the rows. Use the labels from \"rows\".",
                "The edge from \"b\" to \"a\" must end in a lower row than it starts in.",
            ],
        );
    }
}
