//! The blocks of a page.
//!
//! A block is made from one command's arguments through a checked
//! conversion, so a stored block always follows the rules of its kind. A
//! block travels to the UI in the shape it is stored in, tagged by `kind`.

use serde::{Deserialize, Serialize};

use super::command;
use super::id::BlockId;
use super::refusal::Flaw;

/// The most bars a bar chart holds.
const MAX_BARS: usize = 12;
/// The most rows a tree diagram holds.
const MAX_TREE_ROWS: usize = 6;
/// The most labels one row of a tree diagram holds.
const MAX_ROW_LABELS: usize = 8;
/// The fewest stops a timeline holds.
const MIN_STOPS: usize = 2;
/// The most stops a timeline holds.
const MAX_STOPS: usize = 8;

/// One block of a page.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub(crate) struct Block {
    id: BlockId,
    #[serde(flatten)]
    body: Body,
}

impl Block {
    /// Makes a block from its id and its content.
    pub(crate) fn new(id: BlockId, body: Body) -> Self {
        Self { id, body }
    }
}

/// The content of a block, by kind.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub(crate) enum Body {
    Exposition(Exposition),
    Callout(Callout),
    BarChart(BarChart),
    TreeDiagram(TreeDiagram),
    Timeline(Timeline),
    Progress(Progress),
    Success(Success),
}

/// Running text, in markdown.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub(crate) struct Exposition {
    markdown: String,
}

impl TryFrom<command::Exposition> for Exposition {
    type Error = Flaw;

    fn try_from(args: command::Exposition) -> Result<Self, Flaw> {
        Ok(Self {
            markdown: text("markdown", &args.markdown)?,
        })
    }
}

/// Which of the callout commands made a callout. The UI maps each variant
/// to its color, its icon and its title.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum CalloutVariant {
    Tip,
    KeyIdea,
    Remember,
    TailoredForYou,
    Hint,
    Milestone,
    SectionComplete,
}

/// A titled box of markdown: information, a personal note or progress.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub(crate) struct Callout {
    variant: CalloutVariant,
    /// A title that replaces the variant's own.
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    /// The number of a milestone.
    #[serde(skip_serializing_if = "Option::is_none")]
    number: Option<u32>,
    markdown: String,
}

impl Callout {
    fn plain(variant: CalloutVariant, markdown: &str) -> Result<Self, Flaw> {
        Ok(Self {
            variant,
            title: None,
            number: None,
            markdown: text("markdown", markdown)?,
        })
    }
}

impl TryFrom<command::Tip> for Callout {
    type Error = Flaw;

    fn try_from(args: command::Tip) -> Result<Self, Flaw> {
        Ok(Self {
            title: optional_text("title", args.title.as_deref())?,
            ..Self::plain(CalloutVariant::Tip, &args.markdown)?
        })
    }
}

impl TryFrom<command::KeyIdea> for Callout {
    type Error = Flaw;

    fn try_from(args: command::KeyIdea) -> Result<Self, Flaw> {
        Self::plain(CalloutVariant::KeyIdea, &args.markdown)
    }
}

impl TryFrom<command::Remember> for Callout {
    type Error = Flaw;

    fn try_from(args: command::Remember) -> Result<Self, Flaw> {
        Self::plain(CalloutVariant::Remember, &args.markdown)
    }
}

impl TryFrom<command::TailoredForYou> for Callout {
    type Error = Flaw;

    fn try_from(args: command::TailoredForYou) -> Result<Self, Flaw> {
        Self::plain(CalloutVariant::TailoredForYou, &args.markdown)
    }
}

impl TryFrom<command::Hint> for Callout {
    type Error = Flaw;

    fn try_from(args: command::Hint) -> Result<Self, Flaw> {
        Self::plain(CalloutVariant::Hint, &args.markdown)
    }
}

impl TryFrom<command::Milestone> for Callout {
    type Error = Flaw;

    fn try_from(args: command::Milestone) -> Result<Self, Flaw> {
        Ok(Self {
            number: Some(args.number),
            ..Self::plain(CalloutVariant::Milestone, &args.markdown)?
        })
    }
}

impl TryFrom<command::SectionComplete> for Callout {
    type Error = Flaw;

    fn try_from(args: command::SectionComplete) -> Result<Self, Flaw> {
        Self::plain(CalloutVariant::SectionComplete, &args.markdown)
    }
}

/// The color of one bar of a bar chart.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum Tone {
    /// What the chart is about.
    #[default]
    Accent,
    /// The middle ground.
    Neutral,
    /// The other side.
    Negative,
}

/// A bar chart, drawn as a numbered figure.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub(crate) struct BarChart {
    /// The number of the figure, such as `3.1`.
    figure: String,
    caption: String,
    bars: Vec<Bar>,
}

/// One bar of a bar chart.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub(crate) struct Bar {
    label: String,
    value: f64,
    tone: Tone,
}

impl BarChart {
    /// Makes a bar chart from its arguments and its figure number.
    pub(crate) fn new(args: command::BarChart, figure: String) -> Result<Self, Flaw> {
        count("bars", 1, MAX_BARS, args.bars.len())?;
        let caption = text("caption", &args.caption)?;
        let bars = args
            .bars
            .into_iter()
            .map(|bar| {
                let label = text("label", &bar.label)?;
                if bar.value < 0.0 {
                    return Err(Flaw::NegativeBar { label });
                }
                Ok(Bar {
                    label,
                    value: bar.value,
                    tone: bar.tone.unwrap_or_default(),
                })
            })
            .collect::<Result<_, _>>()?;
        Ok(Self {
            figure,
            caption,
            bars,
        })
    }
}

/// A tree diagram, drawn as a numbered figure.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub(crate) struct TreeDiagram {
    /// The number of the figure, such as `3.2`.
    figure: String,
    caption: String,
    rows: Vec<Vec<String>>,
    edges: Vec<Edge>,
}

/// One line of a tree diagram, from a label to a label in a lower row.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub(crate) struct Edge {
    from: String,
    to: String,
}

impl TreeDiagram {
    /// Makes a tree diagram from its arguments and its figure number.
    pub(crate) fn new(args: &command::TreeDiagram, figure: String) -> Result<Self, Flaw> {
        count("rows", 1, MAX_TREE_ROWS, args.rows.len())?;
        let caption = text("caption", &args.caption)?;
        let mut rows: Vec<Vec<String>> = Vec::with_capacity(args.rows.len());
        for row in &args.rows {
            count("labels in a row", 1, MAX_ROW_LABELS, row.len())?;
            let mut labels = Vec::with_capacity(row.len());
            for label in row {
                let label = text("rows", label)?;
                if row_of(&rows, &label).is_some() || labels.contains(&label) {
                    return Err(Flaw::DuplicateLabel { label });
                }
                labels.push(label);
            }
            rows.push(labels);
        }
        let edges = args
            .edges
            .iter()
            .map(|edge| {
                let from = edge.from.trim();
                let to = edge.to.trim();
                let upper = row_of(&rows, from).ok_or_else(|| unknown(from))?;
                let lower = row_of(&rows, to).ok_or_else(|| unknown(to))?;
                if upper >= lower {
                    return Err(Flaw::EdgeNotDownward {
                        from: from.to_owned(),
                        to: to.to_owned(),
                    });
                }
                Ok(Edge {
                    from: from.to_owned(),
                    to: to.to_owned(),
                })
            })
            .collect::<Result<_, _>>()?;
        Ok(Self {
            figure,
            caption,
            rows,
            edges,
        })
    }
}

/// Returns the index of the row that holds a label.
fn row_of(rows: &[Vec<String>], label: &str) -> Option<usize> {
    rows.iter()
        .position(|row| row.iter().any(|held| held == label))
}

fn unknown(label: &str) -> Flaw {
    Flaw::UnknownLabel {
        label: label.to_owned(),
    }
}

/// A row of stops joined by a line.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub(crate) struct Timeline {
    stops: Vec<Stop>,
}

/// One stop of a timeline. A bare stop has an empty note.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub(crate) struct Stop {
    label: String,
    note: String,
}

impl TryFrom<command::Timeline> for Timeline {
    type Error = Flaw;

    fn try_from(args: command::Timeline) -> Result<Self, Flaw> {
        count("stops", MIN_STOPS, MAX_STOPS, args.stops.len())?;
        let stops = args
            .stops
            .iter()
            .map(|stop| {
                Ok(Stop {
                    label: text("label", &stop.label)?,
                    note: stop.note.as_deref().unwrap_or_default().trim().to_owned(),
                })
            })
            .collect::<Result<_, _>>()?;
        Ok(Self { stops })
    }
}

/// A progress bar with a line under it.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub(crate) struct Progress {
    done: u32,
    total: u32,
    label: String,
}

impl TryFrom<command::Progress> for Progress {
    type Error = Flaw;

    fn try_from(args: command::Progress) -> Result<Self, Flaw> {
        if args.total == 0 {
            return Err(Flaw::Zero { field: "total" });
        }
        if args.done > args.total {
            return Err(Flaw::DoneOverTotal {
                done: args.done,
                total: args.total,
            });
        }
        Ok(Self {
            done: args.done,
            total: args.total,
            label: text("label", &args.label)?,
        })
    }
}

/// A one-line strip that praises a success.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub(crate) struct Success {
    text: String,
}

impl TryFrom<command::Success> for Success {
    type Error = Flaw;

    fn try_from(args: command::Success) -> Result<Self, Flaw> {
        Ok(Self {
            text: text("text", &args.text)?,
        })
    }
}

/// Returns the text of a field without the whitespace around it.
pub(crate) fn text(field: &'static str, value: &str) -> Result<String, Flaw> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(Flaw::Empty { field });
    }
    Ok(trimmed.to_owned())
}

/// Returns the text of a field that may be left out.
pub(crate) fn optional_text(
    field: &'static str,
    value: Option<&str>,
) -> Result<Option<String>, Flaw> {
    value.map(|value| text(field, value)).transpose()
}

/// Checks that a list holds a number of items within its limits.
fn count(field: &'static str, min: usize, max: usize, actual: usize) -> Result<(), Flaw> {
    if (min..=max).contains(&actual) {
        return Ok(());
    }
    Err(Flaw::Count {
        field,
        min,
        max,
        actual,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde_json::{Value, json};

    fn args<T: serde::de::DeserializeOwned>(value: Value) -> T {
        serde_json::from_value(value).expect("the arguments have the right shape")
    }

    #[test]
    fn text_is_trimmed_and_empty_text_is_a_flaw() {
        let made = Exposition::try_from(args::<command::Exposition>(
            json!({ "markdown": "  Text.\n" }),
        ));
        assert_eq!(
            made,
            Ok(Exposition {
                markdown: "Text.".to_owned()
            })
        );
        let empty =
            Exposition::try_from(args::<command::Exposition>(json!({ "markdown": " \n " })));
        assert_eq!(empty, Err(Flaw::Empty { field: "markdown" }));
    }

    #[test]
    fn a_tip_keeps_its_title_and_a_milestone_keeps_its_number() {
        let tip = Callout::try_from(args::<command::Tip>(
            json!({ "markdown": "Watch the clock.", "title": "Chair's Tip" }),
        ))
        .expect("the tip is valid");
        assert_eq!(tip.title.as_deref(), Some("Chair's Tip"));
        assert_eq!(tip.number, None);
        let milestone = Callout::try_from(args::<command::Milestone>(
            json!({ "number": 2, "markdown": "Finish the essay." }),
        ))
        .expect("the milestone is valid");
        assert_eq!(milestone.number, Some(2));
        assert_eq!(milestone.variant, CalloutVariant::Milestone);
    }

    #[test]
    fn an_empty_title_is_a_flaw() {
        let tip = Callout::try_from(args::<command::Tip>(
            json!({ "markdown": "Text.", "title": " " }),
        ));
        assert_eq!(tip, Err(Flaw::Empty { field: "title" }));
    }

    #[test]
    fn a_bar_takes_the_accent_tone_unless_it_names_another() {
        let chart = BarChart::new(
            args(json!({ "caption": "A poll", "bars": [
                { "label": "SF", "value": 10 },
                { "label": "N", "value": 2.5, "tone": "neutral" },
            ] })),
            "3.1".to_owned(),
        )
        .expect("the chart is valid");
        let tones: Vec<Tone> = chart.bars.iter().map(|bar| bar.tone).collect();
        assert_eq!(tones, [Tone::Accent, Tone::Neutral]);
        assert_eq!(chart.figure, "3.1");
    }

    #[test]
    fn a_bar_chart_checks_its_bars() {
        let none = BarChart::new(
            args(json!({ "caption": "A poll", "bars": [] })),
            "1".to_owned(),
        );
        assert_eq!(
            none,
            Err(Flaw::Count {
                field: "bars",
                min: 1,
                max: 12,
                actual: 0
            })
        );
        let negative = BarChart::new(
            args(json!({ "caption": "A poll", "bars": [{ "label": "A", "value": -1 }] })),
            "1".to_owned(),
        );
        assert_eq!(
            negative,
            Err(Flaw::NegativeBar {
                label: "A".to_owned()
            })
        );
        let unnamed = BarChart::new(
            args(json!({ "caption": "A poll", "bars": [{ "label": "", "value": 1 }] })),
            "1".to_owned(),
        );
        assert_eq!(unnamed, Err(Flaw::Empty { field: "label" }));
        let uncaptioned = BarChart::new(
            args(json!({ "caption": "", "bars": [{ "label": "A", "value": 1 }] })),
            "1".to_owned(),
        );
        assert_eq!(uncaptioned, Err(Flaw::Empty { field: "caption" }));
    }

    #[expect(
        clippy::needless_pass_by_value,
        reason = "the tests pass json! literals"
    )]
    fn tree(rows: Value, edges: Value) -> Result<TreeDiagram, Flaw> {
        TreeDiagram::new(
            &args(json!({ "caption": "Kinds", "rows": rows, "edges": edges })),
            "3.2".to_owned(),
        )
    }

    #[test]
    fn a_tree_keeps_a_node_with_two_parents_as_one_label() {
        let made = tree(
            json!([
                ["expression"],
                ["glvalue", "rvalue"],
                ["lvalue", "xvalue", "prvalue"]
            ]),
            json!([
                { "from": "glvalue", "to": "xvalue" },
                { "from": "rvalue", "to": "xvalue" },
            ]),
        )
        .expect("the tree is valid");
        assert_eq!(made.rows.len(), 3);
        assert_eq!(made.edges.len(), 2);
    }

    #[test]
    fn a_tree_checks_its_rows_and_edges() {
        assert_eq!(
            tree(json!([]), json!([])),
            Err(Flaw::Count {
                field: "rows",
                min: 1,
                max: 6,
                actual: 0
            }),
        );
        assert_eq!(
            tree(json!([[]]), json!([])),
            Err(Flaw::Count {
                field: "labels in a row",
                min: 1,
                max: 8,
                actual: 0
            }),
        );
        assert_eq!(
            tree(json!([["a"], ["b", "a"]]), json!([])),
            Err(Flaw::DuplicateLabel {
                label: "a".to_owned()
            }),
        );
        assert_eq!(
            tree(json!([["a", "a"]]), json!([])),
            Err(Flaw::DuplicateLabel {
                label: "a".to_owned()
            }),
        );
        assert_eq!(
            tree(json!([["a"], ["b"]]), json!([{ "from": "a", "to": "c" }])),
            Err(Flaw::UnknownLabel {
                label: "c".to_owned()
            }),
        );
        assert_eq!(
            tree(json!([["a"], ["b"]]), json!([{ "from": "b", "to": "a" }])),
            Err(Flaw::EdgeNotDownward {
                from: "b".to_owned(),
                to: "a".to_owned()
            }),
        );
    }

    #[test]
    fn a_timeline_takes_two_to_eight_stops() {
        let one = Timeline::try_from(args::<command::Timeline>(
            json!({ "stops": [{ "label": "Start" }] }),
        ));
        assert_eq!(
            one,
            Err(Flaw::Count {
                field: "stops",
                min: 2,
                max: 8,
                actual: 1
            })
        );
        let two = Timeline::try_from(args::<command::Timeline>(
            json!({ "stops": [{ "label": "Start" }, { "label": "End", "note": " late " }] }),
        ))
        .expect("the timeline is valid");
        let notes: Vec<&str> = two.stops.iter().map(|stop| stop.note.as_str()).collect();
        assert_eq!(notes, ["", "late"]);
    }

    #[test]
    fn progress_counts_within_its_total() {
        let zero = Progress::try_from(args::<command::Progress>(
            json!({ "done": 0, "total": 0, "label": "x" }),
        ));
        assert_eq!(zero, Err(Flaw::Zero { field: "total" }));
        let over = Progress::try_from(args::<command::Progress>(
            json!({ "done": 5, "total": 4, "label": "x" }),
        ));
        assert_eq!(over, Err(Flaw::DoneOverTotal { done: 5, total: 4 }));
        let full = Progress::try_from(args::<command::Progress>(
            json!({ "done": 4, "total": 4, "label": "x" }),
        ));
        assert!(full.is_ok());
    }

    #[test]
    fn a_block_travels_with_its_id_and_its_kind() {
        let body = Body::Callout(
            Callout::try_from(args::<command::Tip>(
                json!({ "markdown": "Keep your eye on the timebox.", "title": "Chair's Tip" }),
            ))
            .expect("the tip is valid"),
        );
        let wire =
            serde_json::to_value(Block::new(BlockId::FIRST.next(), body)).expect("serializes");
        assert_eq!(
            wire,
            json!({
                "id": "b2",
                "kind": "callout",
                "variant": "tip",
                "title": "Chair's Tip",
                "markdown": "Keep your eye on the timebox.",
            }),
        );
    }
}
