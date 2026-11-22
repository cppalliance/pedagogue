//! The lesson model: pages of blocks, built up by page commands.
//!
//! Everything here is plain data and logic. A command is checked in full
//! before it changes anything, so a refused command leaves the lesson as it
//! was.

mod block;
mod command;
mod id;
mod page;
mod refusal;

use std::collections::BTreeMap;

use self::block::{BarChart, Block, Body, TreeDiagram};
pub(crate) use self::command::{Command, TOOLS};
pub(crate) use self::id::{BlockId, PageNumber};
pub(crate) use self::page::Page;
pub(crate) use self::refusal::{Flaw, Refusal};

/// What a command did. The sentence a model reads is written from it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Receipt {
    /// The command started a page.
    StartedPage {
        /// The number of the new page.
        page: PageNumber,
    },
    /// The command added a block to the newest page.
    AddedBlock {
        /// The command that made the block.
        tool: &'static str,
        /// The id of the new block.
        id: BlockId,
        /// The page that holds the block.
        page: PageNumber,
    },
    /// The command added a note to the foot of the newest page.
    AddedFootnote {
        /// The number of the note on its page.
        number: usize,
        /// The page that holds the note.
        page: PageNumber,
    },
    /// The command set the Continue button of the newest page.
    SetContinue {
        /// The label of the button.
        label: String,
        /// The page that holds the button.
        page: PageNumber,
    },
}

impl Receipt {
    /// Returns the number of the page the command changed.
    pub(crate) fn page(&self) -> PageNumber {
        match self {
            Self::StartedPage { page }
            | Self::AddedBlock { page, .. }
            | Self::AddedFootnote { page, .. }
            | Self::SetContinue { page, .. } => *page,
        }
    }
}

/// A lesson: its pages, in order, and the counters that number its parts.
#[derive(Debug, Default)]
pub(crate) struct Lesson {
    pages: Vec<Page>,
    /// The id of the block added last. Ids run across the whole lesson.
    last_block: Option<BlockId>,
    /// How many figures each chapter holds so far, by chapter number.
    figures: BTreeMap<Option<u32>, u32>,
}

impl Lesson {
    /// Returns the pages of the lesson, in order.
    pub(crate) fn pages(&self) -> &[Page] {
        &self.pages
    }

    /// Applies one command and says what it did.
    pub(crate) fn apply(&mut self, command: Command) -> Result<Receipt, Refusal> {
        let tool = command.tool();
        match command {
            Command::NewPage(args) => self
                .start_page(&args)
                .map_err(|flaw| Refusal::Flawed { tool, flaw }),
            Command::Exposition(args) => self.add(tool, || args.try_into().map(Body::Exposition)),
            Command::Footnote(args) => self.on_page(tool, |page| {
                let note = block::text("markdown", &args.markdown)?;
                Ok(Receipt::AddedFootnote {
                    number: page.add_footnote(note),
                    page: page.number(),
                })
            }),
            Command::BarChart(args) => self.add_figure(tool, |figure| {
                BarChart::new(args, figure).map(Body::BarChart)
            }),
            Command::TreeDiagram(args) => self.add_figure(tool, |figure| {
                TreeDiagram::new(&args, figure).map(Body::TreeDiagram)
            }),
            Command::Timeline(args) => self.add(tool, || args.try_into().map(Body::Timeline)),
            Command::Tip(args) => self.add(tool, || args.try_into().map(Body::Callout)),
            Command::KeyIdea(args) => self.add(tool, || args.try_into().map(Body::Callout)),
            Command::Remember(args) => self.add(tool, || args.try_into().map(Body::Callout)),
            Command::TailoredForYou(args) => self.add(tool, || args.try_into().map(Body::Callout)),
            Command::Hint(args) => self.add(tool, || args.try_into().map(Body::Callout)),
            Command::Milestone(args) => self.add(tool, || args.try_into().map(Body::Callout)),
            Command::Progress(args) => self.add(tool, || args.try_into().map(Body::Progress)),
            Command::SectionComplete(args) => self.add(tool, || args.try_into().map(Body::Callout)),
            Command::Success(args) => self.add(tool, || args.try_into().map(Body::Success)),
            Command::ContinueButton(args) => self.on_page(tool, |page| {
                let label = block::text("label", &args.label)?;
                page.set_continue(label.clone());
                Ok(Receipt::SetContinue {
                    label,
                    page: page.number(),
                })
            }),
        }
    }

    fn start_page(&mut self, args: &command::NewPage) -> Result<Receipt, Flaw> {
        let page = Page::start(self.pages.last(), args)?;
        let number = page.number();
        self.pages.push(page);
        Ok(Receipt::StartedPage { page: number })
    }

    /// Changes the newest page. The change checks its input first, so a
    /// flaw leaves the page as it was.
    fn on_page(
        &mut self,
        tool: &'static str,
        change: impl FnOnce(&mut Page) -> Result<Receipt, Flaw>,
    ) -> Result<Receipt, Refusal> {
        let Some(page) = self.pages.last_mut() else {
            return Err(Refusal::NoPage { tool });
        };
        change(page).map_err(|flaw| Refusal::Flawed { tool, flaw })
    }

    /// Adds a block to the newest page under the next id.
    fn add(
        &mut self,
        tool: &'static str,
        make: impl FnOnce() -> Result<Body, Flaw>,
    ) -> Result<Receipt, Refusal> {
        let id = self.last_block.map_or(BlockId::FIRST, BlockId::next);
        let receipt = self.on_page(tool, |page| {
            page.push(Block::new(id, make()?));
            Ok(Receipt::AddedBlock {
                tool,
                id,
                page: page.number(),
            })
        })?;
        self.last_block = Some(id);
        Ok(receipt)
    }

    /// Adds a figure to the newest page under the next figure number of its
    /// chapter, such as `3.1`. A page outside any chapter counts `1`, `2`.
    fn add_figure(
        &mut self,
        tool: &'static str,
        make: impl FnOnce(String) -> Result<Body, Flaw>,
    ) -> Result<Receipt, Refusal> {
        let chapter = self.pages.last().and_then(Page::chapter_number);
        let number = self
            .figures
            .get(&chapter)
            .copied()
            .unwrap_or(0)
            .saturating_add(1);
        let figure = chapter.map_or_else(
            || number.to_string(),
            |chapter| format!("{chapter}.{number}"),
        );
        let receipt = self.add(tool, || make(figure))?;
        self.figures.insert(chapter, number);
        Ok(receipt)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde_json::{Value, json};

    #[expect(
        clippy::needless_pass_by_value,
        reason = "the tests pass json! literals"
    )]
    fn run(lesson: &mut Lesson, tool: &str, args: Value) -> Result<Receipt, Refusal> {
        lesson.apply(Command::parse(tool, &args)?)
    }

    fn ok(lesson: &mut Lesson, tool: &str, args: Value) -> Receipt {
        run(lesson, tool, args).expect("the command applies")
    }

    fn lesson_on_page_1() -> Lesson {
        let mut lesson = Lesson::default();
        ok(
            &mut lesson,
            "new_page",
            json!({ "chapter_number": 3, "chapter_title": "Chairing a WG21 Room",
                    "section_number": "3.2", "section_title": "Taking a Straw Poll" }),
        );
        lesson
    }

    fn wire(lesson: &Lesson) -> Value {
        serde_json::to_value(lesson.pages()).expect("pages serialize")
    }

    /// One valid call of each command that needs a page.
    fn calls_on_a_page() -> Vec<(&'static str, Value)> {
        vec![
            (
                "exposition",
                json!({ "markdown": "The chair states the question clearly." }),
            ),
            (
                "footnote",
                json!({ "markdown": "Straw polls are non-binding." }),
            ),
            (
                "bar_chart",
                json!({ "caption": "A poll with consensus in favor", "bars": [
                    { "label": "SF", "value": 10 },
                    { "label": "N", "value": 2, "tone": "neutral" },
                    { "label": "SA", "value": 0, "tone": "negative" },
                ] }),
            ),
            (
                "tree_diagram",
                json!({ "caption": "Value categories",
                        "rows": [["expression"], ["glvalue", "rvalue"]],
                        "edges": [{ "from": "expression", "to": "glvalue" },
                                  { "from": "expression", "to": "rvalue" }] }),
            ),
            (
                "timeline",
                json!({ "stops": [{ "label": "Present", "note": "0 min" }, { "label": "Poll" }] }),
            ),
            (
                "tip",
                json!({ "markdown": "Keep your eye on the timebox.", "title": "Chair's Tip" }),
            ),
            ("key_idea", json!({ "markdown": "Arrive with a question." })),
            (
                "remember",
                json!({ "markdown": "`std::move` is only a cast." }),
            ),
            (
                "tailored_for_you",
                json!({ "markdown": "Start with section 2." }),
            ),
            (
                "hint",
                json!({ "markdown": "Think of `co_await` as a return." }),
            ),
            (
                "milestone",
                json!({ "number": 2, "markdown": "Finish this essay." }),
            ),
            (
                "progress",
                json!({ "done": 8, "total": 12, "label": "Milestone 8 of 12" }),
            ),
            (
                "section_complete",
                json!({ "markdown": "Section 1.3 is done." }),
            ),
            ("success", json!({ "text": "Nice work." })),
            ("continue_button", json!({ "label": "Continue to 3.3" })),
        ]
    }

    #[test]
    fn the_calls_cover_every_command() {
        let mut tools = vec!["new_page"];
        tools.extend(calls_on_a_page().iter().map(|(tool, _)| *tool));
        assert_eq!(tools, TOOLS);
    }

    #[test]
    fn every_command_but_new_page_needs_a_page() {
        for (tool, args) in calls_on_a_page() {
            let mut lesson = Lesson::default();
            let refusal = run(&mut lesson, tool, args).expect_err("the lesson has no page");
            assert!(matches!(refusal, Refusal::NoPage { tool: named } if named == tool));
            assert!(lesson.pages().is_empty());
        }
    }

    #[test]
    fn pages_are_numbered_from_1() {
        let mut lesson = Lesson::default();
        let first = ok(&mut lesson, "new_page", json!({}));
        let second = ok(&mut lesson, "new_page", json!({}));
        assert_eq!(
            first,
            Receipt::StartedPage {
                page: PageNumber::FIRST
            }
        );
        assert_eq!(second.page(), PageNumber::FIRST.next());
        assert_eq!(wire(&lesson)[1]["number"], json!(2));
    }

    #[test]
    fn each_command_draws_its_part_of_the_page() {
        let mut lesson = lesson_on_page_1();
        for (tool, args) in calls_on_a_page() {
            ok(&mut lesson, tool, args);
        }
        let expected = json!([{
            "number": 1,
            "chapter_number": 3,
            "chapter_title": "Chairing a WG21 Room",
            "section_number": "3.2",
            "section_title": "Taking a Straw Poll",
            "chapter_opener": true,
            "blocks": [
                { "id": "b1", "kind": "exposition",
                  "markdown": "The chair states the question clearly." },
                { "id": "b2", "kind": "bar_chart", "figure": "3.1",
                  "caption": "A poll with consensus in favor", "bars": [
                    { "label": "SF", "value": 10.0, "tone": "accent" },
                    { "label": "N", "value": 2.0, "tone": "neutral" },
                    { "label": "SA", "value": 0.0, "tone": "negative" },
                  ] },
                { "id": "b3", "kind": "tree_diagram", "figure": "3.2",
                  "caption": "Value categories",
                  "rows": [["expression"], ["glvalue", "rvalue"]],
                  "edges": [{ "from": "expression", "to": "glvalue" },
                            { "from": "expression", "to": "rvalue" }] },
                { "id": "b4", "kind": "timeline", "stops": [
                    { "label": "Present", "note": "0 min" },
                    { "label": "Poll", "note": "" },
                  ] },
                { "id": "b5", "kind": "callout", "variant": "tip", "title": "Chair's Tip",
                  "markdown": "Keep your eye on the timebox." },
                { "id": "b6", "kind": "callout", "variant": "key_idea",
                  "markdown": "Arrive with a question." },
                { "id": "b7", "kind": "callout", "variant": "remember",
                  "markdown": "`std::move` is only a cast." },
                { "id": "b8", "kind": "callout", "variant": "tailored_for_you",
                  "markdown": "Start with section 2." },
                { "id": "b9", "kind": "callout", "variant": "hint",
                  "markdown": "Think of `co_await` as a return." },
                { "id": "b10", "kind": "callout", "variant": "milestone", "number": 2,
                  "markdown": "Finish this essay." },
                { "id": "b11", "kind": "progress", "done": 8, "total": 12,
                  "label": "Milestone 8 of 12" },
                { "id": "b12", "kind": "callout", "variant": "section_complete",
                  "markdown": "Section 1.3 is done." },
                { "id": "b13", "kind": "success", "text": "Nice work." },
            ],
            "footnotes": ["Straw polls are non-binding."],
            "continue_label": "Continue to 3.3",
            "continue_pressed": false,
        }]);
        assert_eq!(wire(&lesson), expected);
    }

    #[test]
    fn a_receipt_names_the_block_and_its_page() {
        let mut lesson = lesson_on_page_1();
        let added = ok(&mut lesson, "exposition", json!({ "markdown": "Text." }));
        assert_eq!(
            added,
            Receipt::AddedBlock {
                tool: "exposition",
                id: BlockId::FIRST,
                page: PageNumber::FIRST
            },
        );
        let note = ok(&mut lesson, "footnote", json!({ "markdown": "A note." }));
        assert_eq!(
            note,
            Receipt::AddedFootnote {
                number: 1,
                page: PageNumber::FIRST
            }
        );
        let button = ok(
            &mut lesson,
            "continue_button",
            json!({ "label": " Continue to 3.3 " }),
        );
        assert_eq!(
            button,
            Receipt::SetContinue {
                label: "Continue to 3.3".to_owned(),
                page: PageNumber::FIRST
            },
        );
    }

    #[test]
    fn block_ids_run_across_pages() {
        let mut lesson = lesson_on_page_1();
        ok(&mut lesson, "exposition", json!({ "markdown": "One." }));
        ok(&mut lesson, "new_page", json!({}));
        let added = ok(&mut lesson, "exposition", json!({ "markdown": "Two." }));
        assert_eq!(
            added,
            Receipt::AddedBlock {
                tool: "exposition",
                id: BlockId::FIRST.next(),
                page: PageNumber::FIRST.next(),
            },
        );
    }

    #[test]
    fn chapter_fields_repeat_until_a_page_names_new_ones() {
        let mut lesson = lesson_on_page_1();
        ok(
            &mut lesson,
            "new_page",
            json!({ "section_number": "3.3", "section_title": "After the Poll" }),
        );
        ok(
            &mut lesson,
            "new_page",
            json!({ "chapter_number": 4, "chapter_title": "Papers" }),
        );
        let pages = wire(&lesson);
        assert_eq!(pages[1]["chapter_number"], json!(3));
        assert_eq!(pages[1]["chapter_title"], json!("Chairing a WG21 Room"));
        assert_eq!(pages[1]["section_title"], json!("After the Poll"));
        assert_eq!(pages[1]["chapter_opener"], json!(false));
        assert_eq!(pages[2]["chapter_title"], json!("Papers"));
        assert_eq!(pages[2]["chapter_opener"], json!(true));
        // A section belongs to its own page, so the next page starts bare.
        assert_eq!(pages[2].get("section_title"), None);
    }

    #[test]
    fn figures_are_numbered_by_chapter() {
        let chart = json!({ "caption": "A poll", "bars": [{ "label": "SF", "value": 1 }] });
        let figures = |lesson: &Lesson| -> Vec<Value> {
            wire(lesson)
                .as_array()
                .expect("pages are a list")
                .iter()
                .flat_map(|page| {
                    page["blocks"]
                        .as_array()
                        .expect("blocks are a list")
                        .clone()
                })
                .map(|block| block["figure"].clone())
                .collect()
        };

        let mut lesson = lesson_on_page_1();
        ok(&mut lesson, "bar_chart", chart.clone());
        ok(&mut lesson, "new_page", json!({}));
        ok(&mut lesson, "bar_chart", chart.clone());
        ok(&mut lesson, "new_page", json!({ "chapter_number": 4 }));
        ok(&mut lesson, "bar_chart", chart.clone());
        assert_eq!(figures(&lesson), [json!("3.1"), json!("3.2"), json!("4.1")]);

        let mut bare = Lesson::default();
        ok(&mut bare, "new_page", json!({}));
        ok(&mut bare, "bar_chart", chart.clone());
        ok(&mut bare, "bar_chart", chart);
        assert_eq!(figures(&bare), [json!("1"), json!("2")]);
    }

    #[test]
    fn footnotes_are_numbered_on_each_page() {
        let mut lesson = lesson_on_page_1();
        ok(&mut lesson, "footnote", json!({ "markdown": "One." }));
        let second = ok(&mut lesson, "footnote", json!({ "markdown": "Two." }));
        ok(&mut lesson, "new_page", json!({}));
        let fresh = ok(&mut lesson, "footnote", json!({ "markdown": "Three." }));
        assert_eq!(
            second,
            Receipt::AddedFootnote {
                number: 2,
                page: PageNumber::FIRST
            }
        );
        assert_eq!(
            fresh,
            Receipt::AddedFootnote {
                number: 1,
                page: PageNumber::FIRST.next()
            },
        );
    }

    #[test]
    fn continue_button_replaces_the_earlier_label() {
        let mut lesson = lesson_on_page_1();
        ok(
            &mut lesson,
            "continue_button",
            json!({ "label": "Continue to 3.3" }),
        );
        ok(
            &mut lesson,
            "continue_button",
            json!({ "label": "Try the quiz again" }),
        );
        assert_eq!(
            wire(&lesson)[0]["continue_label"],
            json!("Try the quiz again")
        );
    }

    #[test]
    fn a_flawed_command_leaves_the_lesson_as_it_was() {
        let mut lesson = lesson_on_page_1();
        let before = wire(&lesson);
        let flawed = [
            ("exposition", json!({ "markdown": "  " })),
            ("footnote", json!({ "markdown": "" })),
            ("bar_chart", json!({ "caption": "A poll", "bars": [] })),
            ("continue_button", json!({ "label": "" })),
            ("new_page", json!({ "section_title": " " })),
        ];
        for (tool, args) in flawed {
            let refusal = run(&mut lesson, tool, args).expect_err("the arguments break a rule");
            assert!(matches!(refusal, Refusal::Flawed { tool: named, .. } if named == tool));
        }
        assert_eq!(wire(&lesson), before);

        // The refused chart used up neither a block id nor a figure number.
        let chart = json!({ "caption": "A poll", "bars": [{ "label": "SF", "value": 1 }] });
        let added = ok(&mut lesson, "bar_chart", chart);
        assert_eq!(
            added,
            Receipt::AddedBlock {
                tool: "bar_chart",
                id: BlockId::FIRST,
                page: PageNumber::FIRST
            },
        );
        assert_eq!(wire(&lesson)[0]["blocks"][0]["figure"], json!("3.1"));
    }
}
