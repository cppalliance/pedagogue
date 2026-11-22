//! The lesson model: pages of blocks, built up by page commands.
//!
//! Everything here is plain data and logic. A command is checked in full
//! before it changes anything, so a refused command leaves the lesson as it
//! was.

mod block;
mod command;
mod id;
mod page;
mod quiz;
mod quiz_rules;
mod refusal;

use std::collections::BTreeMap;

use self::block::{BarChart, Block, Body, TreeDiagram};
pub(crate) use self::command::{Command, TOOLS};
pub(crate) use self::id::{BlockId, PageNumber};
pub(crate) use self::page::Page;
pub(crate) use self::quiz::{ResponseKind, Verdict};
pub(crate) use self::refusal::{Flaw, Misstep, Refusal};
use crate::event::{QuizAnswer, StudentEvent};

/// What a student's action did: the event the host hears, and the page the
/// action changed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Acted {
    /// What the student did, for the host.
    pub(crate) event: StudentEvent,
    /// The number of the page the action changed.
    pub(crate) page: PageNumber,
}

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
    /// The command marked the latest answer of a free-text quiz.
    MarkedQuiz {
        /// The id of the quiz.
        id: BlockId,
        /// The verdict the command gave.
        verdict: Verdict,
        /// The page that holds the quiz.
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
            | Self::SetContinue { page, .. }
            | Self::MarkedQuiz { page, .. } => *page,
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
            Command::PopQuiz(args) => self.add(tool, || args.try_into().map(Body::ChoiceQuiz)),
            Command::ShortAnswer(args) => {
                self.add(tool, || args.try_into().map(Body::ResponseQuiz))
            }
            Command::ShortEssay(args) => self.add(tool, || args.try_into().map(Body::ResponseQuiz)),
            Command::TryIt(args) => self.add(tool, || args.try_into().map(Body::ResponseQuiz)),
            Command::QuizResult(args) => self.mark(tool, &args),
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

    /// Marks the latest answer of a free-text quiz with the host's verdict.
    fn mark(&mut self, tool: &'static str, args: &command::QuizResult) -> Result<Receipt, Refusal> {
        if self.pages.is_empty() {
            return Err(Refusal::NoPage { tool });
        }
        let feedback = block::text("feedback", &args.feedback)
            .map_err(|flaw| Refusal::Flawed { tool, flaw })?;
        let quiz = args.quiz;
        match self.body_mut(quiz) {
            Some((page, Body::ResponseQuiz(found))) => {
                if !quiz_rules::can_mark(found) {
                    return Err(Refusal::NoAnswerYet { quiz });
                }
                found.mark(args.verdict, feedback);
                Ok(Receipt::MarkedQuiz {
                    id: quiz,
                    verdict: args.verdict,
                    page,
                })
            }
            Some((_, Body::ChoiceQuiz(_))) => Err(Refusal::MarksItself { quiz }),
            Some(_) | None => Err(Refusal::UnknownQuiz { quiz }),
        }
    }

    /// Records a student's picks on a choice quiz. The page marks them at
    /// once, and the event says whether they solved the quiz.
    pub(crate) fn answer_choice(
        &mut self,
        quiz: BlockId,
        picks: &[usize],
    ) -> Result<Acted, Misstep> {
        let (page, found) = match self.body_mut(quiz) {
            Some((page, Body::ChoiceQuiz(found))) => (page, found),
            Some((_, Body::ResponseQuiz(_))) => return Err(Misstep::TakesText { quiz }),
            Some(_) | None => return Err(Misstep::UnknownQuiz { quiz }),
        };
        let attempt =
            quiz_rules::judge_picks(found, picks).map_err(|fault| Misstep::Quiz { quiz, fault })?;
        let answer = QuizAnswer::Picks {
            picks: found.name_picks(&attempt.picks),
            correct: attempt.correct,
        };
        found.record(attempt);
        Ok(Acted {
            event: StudentEvent::QuizAnswered {
                quiz,
                attempt: found.attempts(),
                answer,
            },
            page,
        })
    }

    /// Records a student's answer to a free-text quiz. It waits there for
    /// the host's verdict.
    pub(crate) fn answer_text(&mut self, quiz: BlockId, text: &str) -> Result<Acted, Misstep> {
        let (page, found) = match self.body_mut(quiz) {
            Some((page, Body::ResponseQuiz(found))) => (page, found),
            Some((_, Body::ChoiceQuiz(_))) => return Err(Misstep::TakesPicks { quiz }),
            Some(_) | None => return Err(Misstep::UnknownQuiz { quiz }),
        };
        let attempt =
            quiz_rules::accept_text(found, text).map_err(|fault| Misstep::Quiz { quiz, fault })?;
        let answer = QuizAnswer::Text {
            kind: found.kind(),
            text: attempt.answer.clone(),
        };
        found.record(attempt);
        Ok(Acted {
            event: StudentEvent::QuizAnswered {
                quiz,
                attempt: found.attempts(),
                answer,
            },
            page,
        })
    }

    /// Records a press of the Continue button at the foot of the newest
    /// page.
    pub(crate) fn press_continue(&mut self, page: PageNumber) -> Result<Acted, Misstep> {
        let newest = self.pages.last().map(Page::number);
        let Some(found) = self.pages.iter_mut().find(|held| held.number() == page) else {
            return Err(Misstep::UnknownPage { page });
        };
        let Some(label) = found.continue_label().map(str::to_owned) else {
            return Err(Misstep::NoContinue { page });
        };
        if newest != Some(page) {
            return Err(Misstep::NotNewest { page });
        }
        if found.is_continue_pressed() {
            return Err(Misstep::AlreadyPressed { page });
        }
        found.press_continue();
        Ok(Acted {
            event: StudentEvent::ContinuePressed { page, label },
            page,
        })
    }

    /// Finds the content of a block by id, with the number of its page.
    fn body_mut(&mut self, id: BlockId) -> Option<(PageNumber, &mut Body)> {
        self.pages.iter_mut().find_map(|page| {
            let number = page.number();
            page.body_mut(id).map(|body| (number, body))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde_json::{Value, json};

    use super::refusal::Fault;
    use crate::event::Pick;

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
                "pop_quiz",
                json!({ "question": "What should the chair do first?", "options": [
                    { "text": "Restate the question", "correct": true,
                      "feedback": "RIGHT-FEEDBACK" },
                    { "text": "Count the room", "correct": false,
                      "feedback": "WRONG-FEEDBACK" },
                ] }),
            ),
            ("short_answer", json!({ "question": "What does it do?" })),
            (
                "short_essay",
                json!({ "question": "Describe a problem.", "words": 100 }),
            ),
            (
                "try_it",
                json!({ "task": "Double it.", "code": "co_return x + 1;", "language": "cpp" }),
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

    /// A verdict on the quiz that `calls_on_a_page` adds as its twelfth
    /// block.
    fn verdict_call() -> (&'static str, Value) {
        (
            "quiz_result",
            json!({ "quiz": "b12", "verdict": "correct", "feedback": "Exactly." }),
        )
    }

    #[test]
    fn the_calls_cover_every_command() {
        let mut covered = vec!["new_page", verdict_call().0];
        covered.extend(calls_on_a_page().iter().map(|(tool, _)| *tool));
        covered.sort_unstable();
        let mut all = TOOLS.to_vec();
        all.sort_unstable();
        assert_eq!(covered, all);
    }

    #[test]
    fn every_command_but_new_page_needs_a_page() {
        let mut calls = calls_on_a_page();
        calls.push(verdict_call());
        for (tool, args) in calls {
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
                { "id": "b11", "kind": "choice_quiz",
                  "question": "What should the chair do first?", "multiple": false,
                  "status": "open", "attempts": 0, "missing": false,
                  "options": [
                    { "text": "Restate the question", "state": "open", "checked": false },
                    { "text": "Count the room", "state": "open", "checked": false },
                  ],
                  "feedback": [] },
                { "id": "b12", "kind": "response_quiz", "variant": "short_answer",
                  "question": "What does it do?", "status": "open", "attempts": 0 },
                { "id": "b13", "kind": "response_quiz", "variant": "short_essay",
                  "question": "Describe a problem.", "words": 100,
                  "status": "open", "attempts": 0 },
                { "id": "b14", "kind": "response_quiz", "variant": "try_it",
                  "question": "Double it.", "code": "co_return x + 1;", "language": "cpp",
                  "status": "open", "attempts": 0 },
                { "id": "b15", "kind": "progress", "done": 8, "total": 12,
                  "label": "Milestone 8 of 12" },
                { "id": "b16", "kind": "callout", "variant": "section_complete",
                  "markdown": "Section 1.3 is done." },
                { "id": "b17", "kind": "success", "text": "Nice work." },
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

    /// Returns the id of the n-th block of a lesson, such as `b3` for 3.
    fn id(n: usize) -> BlockId {
        (1..n).fold(BlockId::FIRST, |id, _| id.next())
    }

    /// Returns the number of the n-th page of a lesson.
    fn page(n: usize) -> PageNumber {
        (1..n).fold(PageNumber::FIRST, |page, _| page.next())
    }

    /// A lesson whose one page holds a pop quiz `b1`, a select-all quiz
    /// `b2`, a short-answer quiz `b3` and a paragraph `b4`.
    fn lesson_with_quizzes() -> Lesson {
        let mut lesson = lesson_on_page_1();
        ok(
            &mut lesson,
            "pop_quiz",
            json!({ "question": "When is the file closed?", "options": [
                { "text": "When main returns", "correct": false,
                  "feedback": "WRONG-FEEDBACK" },
                { "text": "When the object leaves scope", "correct": true,
                  "feedback": "RIGHT-FEEDBACK" },
                { "text": "When you call close", "correct": false },
            ] }),
        );
        ok(
            &mut lesson,
            "pop_quiz",
            json!({ "question": "Which are columns?", "multiple": true, "options": [
                { "text": "Strongly favor", "correct": true },
                { "text": "Abstain", "correct": false },
                { "text": "Neutral", "correct": true },
            ] }),
        );
        ok(
            &mut lesson,
            "short_answer",
            json!({ "question": "What does `std::move` do?" }),
        );
        ok(&mut lesson, "exposition", json!({ "markdown": "Text." }));
        lesson
    }

    /// Returns the wire form of one block of the first page.
    fn block(lesson: &Lesson, index: usize) -> Value {
        wire(lesson)[0]["blocks"][index].clone()
    }

    #[test]
    fn a_pop_quiz_checks_its_options() {
        let mut lesson = lesson_on_page_1();
        let option = |correct: bool| json!({ "text": "An option", "correct": correct });
        let seven: Vec<Value> = (0..7).map(|index| option(index == 0)).collect();
        let cases = [
            (
                json!([option(true)]),
                false,
                Flaw::Count {
                    field: "options",
                    min: 2,
                    max: 6,
                    actual: 1,
                },
            ),
            (
                json!(seven),
                false,
                Flaw::Count {
                    field: "options",
                    min: 2,
                    max: 6,
                    actual: 7,
                },
            ),
            (
                json!([option(false), option(false)]),
                false,
                Flaw::RightOptions {
                    multiple: false,
                    actual: 0,
                },
            ),
            (
                json!([option(true), option(true)]),
                false,
                Flaw::RightOptions {
                    multiple: false,
                    actual: 2,
                },
            ),
            (
                json!([option(false), option(false)]),
                true,
                Flaw::RightOptions {
                    multiple: true,
                    actual: 0,
                },
            ),
        ];
        for (options, multiple, expected) in cases {
            let args = json!({ "question": "Which?", "options": options, "multiple": multiple });
            let refusal = run(&mut lesson, "pop_quiz", args).expect_err("the options break a rule");
            assert!(matches!(refusal, Refusal::Flawed { flaw, .. } if flaw == expected));
        }
    }

    #[test]
    fn a_wrong_pick_is_disabled_and_reported_and_the_right_pick_solves_the_quiz() {
        let mut lesson = lesson_with_quizzes();
        let wrong = lesson
            .answer_choice(id(1), &[0])
            .expect("the pick is allowed");
        assert_eq!(
            wrong,
            Acted {
                page: page(1),
                event: StudentEvent::QuizAnswered {
                    quiz: id(1),
                    attempt: 1,
                    answer: QuizAnswer::Picks {
                        picks: vec![Pick {
                            letter: 'A',
                            text: "When main returns".to_owned(),
                        }],
                        correct: false,
                    },
                },
            },
        );
        assert_eq!(
            block(&lesson, 0)["options"],
            json!([
                { "text": "When main returns", "state": "disabled", "checked": false },
                { "text": "When the object leaves scope", "state": "open", "checked": false },
                { "text": "When you call close", "state": "open", "checked": false },
            ]),
        );
        assert_eq!(block(&lesson, 0)["feedback"], json!(["WRONG-FEEDBACK"]));

        let right = lesson
            .answer_choice(id(1), &[1])
            .expect("the pick is allowed");
        assert!(matches!(
            right.event,
            StudentEvent::QuizAnswered {
                attempt: 2,
                answer: QuizAnswer::Picks { correct: true, .. },
                ..
            }
        ));
        assert_eq!(block(&lesson, 0)["status"], json!("solved"));
        assert_eq!(
            block(&lesson, 0)["options"][1],
            json!({ "text": "When the object leaves scope", "state": "right", "checked": true }),
        );
        assert_eq!(block(&lesson, 0)["feedback"], json!(["RIGHT-FEEDBACK"]));
    }

    #[test]
    fn a_select_all_attempt_reports_every_pick() {
        let mut lesson = lesson_with_quizzes();
        let acted = lesson
            .answer_choice(id(2), &[1, 0])
            .expect("the picks are allowed");
        let StudentEvent::QuizAnswered {
            answer: QuizAnswer::Picks { picks, correct },
            ..
        } = acted.event
        else {
            panic!("expected an answer with picks");
        };
        let letters: Vec<char> = picks.iter().map(|pick| pick.letter).collect();
        assert_eq!(letters, ['A', 'B']);
        assert!(!correct);
        assert_eq!(block(&lesson, 1)["missing"], json!(true));
    }

    #[test]
    fn a_free_text_answer_waits_for_quiz_result() {
        let mut lesson = lesson_with_quizzes();
        let sent = lesson
            .answer_text(id(3), " It moves the object. ")
            .expect("the quiz is open");
        assert_eq!(
            sent,
            Acted {
                page: page(1),
                event: StudentEvent::QuizAnswered {
                    quiz: id(3),
                    attempt: 1,
                    answer: QuizAnswer::Text {
                        kind: ResponseKind::ShortAnswer,
                        text: "It moves the object.".to_owned(),
                    },
                },
            },
        );
        assert_eq!(block(&lesson, 2)["status"], json!("submitted"));
        assert_eq!(block(&lesson, 2)["answer"], json!("It moves the object."));

        let marked = ok(
            &mut lesson,
            "quiz_result",
            json!({ "quiz": "b3", "verdict": "incorrect", "feedback": "Nothing has moved yet." }),
        );
        assert_eq!(
            marked,
            Receipt::MarkedQuiz {
                id: id(3),
                verdict: Verdict::Incorrect,
                page: page(1),
            },
        );
        assert_eq!(block(&lesson, 2)["status"], json!("wrong"));
        assert_eq!(
            block(&lesson, 2)["feedback"],
            json!("Nothing has moved yet.")
        );

        lesson
            .answer_text(id(3), "It casts to an rvalue.")
            .expect("a wrong answer may be retried");
        assert_eq!(block(&lesson, 2)["status"], json!("submitted"));
        assert_eq!(block(&lesson, 2).get("feedback"), None);
        ok(
            &mut lesson,
            "quiz_result",
            json!({ "quiz": "b3", "verdict": "correct", "feedback": "Exactly." }),
        );
        assert_eq!(block(&lesson, 2)["status"], json!("right"));
        assert_eq!(block(&lesson, 2)["attempts"], json!(2));
    }

    #[test]
    fn quiz_result_is_refused_unless_a_free_text_answer_waits() {
        let mut lesson = lesson_with_quizzes();
        let before = wire(&lesson);
        let verdict =
            |quiz: &str| json!({ "quiz": quiz, "verdict": "correct", "feedback": "Exactly." });
        assert!(matches!(
            run(&mut lesson, "quiz_result", verdict("b1")),
            Err(Refusal::MarksItself { quiz }) if quiz == id(1)
        ));
        assert!(matches!(
            run(&mut lesson, "quiz_result", verdict("b3")),
            Err(Refusal::NoAnswerYet { quiz }) if quiz == id(3)
        ));
        assert!(matches!(
            run(&mut lesson, "quiz_result", verdict("b4")),
            Err(Refusal::UnknownQuiz { quiz }) if quiz == id(4)
        ));
        assert!(matches!(
            run(&mut lesson, "quiz_result", verdict("b99")),
            Err(Refusal::UnknownQuiz { .. })
        ));
        let empty = json!({ "quiz": "b3", "verdict": "correct", "feedback": " " });
        assert!(matches!(
            run(&mut lesson, "quiz_result", empty),
            Err(Refusal::Flawed {
                flaw: Flaw::Empty { field: "feedback" },
                ..
            })
        ));
        assert_eq!(wire(&lesson), before);
    }

    #[test]
    fn an_action_that_fits_no_open_quiz_is_refused_and_changes_nothing() {
        let mut lesson = lesson_with_quizzes();
        lesson
            .answer_choice(id(1), &[0])
            .expect("the pick is allowed");
        lesson
            .answer_text(id(3), "An answer.")
            .expect("the quiz is open");
        let before = wire(&lesson);
        let broke = |quiz: BlockId, fault: Fault| Err(Misstep::Quiz { quiz, fault });

        assert_eq!(
            lesson.answer_choice(id(99), &[0]),
            Err(Misstep::UnknownQuiz { quiz: id(99) }),
        );
        assert_eq!(
            lesson.answer_choice(id(4), &[0]),
            Err(Misstep::UnknownQuiz { quiz: id(4) }),
        );
        assert_eq!(
            lesson.answer_choice(id(3), &[0]),
            Err(Misstep::TakesText { quiz: id(3) }),
        );
        assert_eq!(
            lesson.answer_text(id(1), "Text."),
            Err(Misstep::TakesPicks { quiz: id(1) }),
        );
        assert_eq!(
            lesson.answer_text(id(4), "Text."),
            Err(Misstep::UnknownQuiz { quiz: id(4) }),
        );
        assert_eq!(
            lesson.answer_choice(id(1), &[]),
            broke(id(1), Fault::NoPick)
        );
        assert_eq!(
            lesson.answer_choice(id(1), &[1, 2]),
            broke(id(1), Fault::OnePick)
        );
        assert_eq!(
            lesson.answer_choice(id(1), &[5]),
            broke(id(1), Fault::NoSuchOption { pick: 5 }),
        );
        assert_eq!(
            lesson.answer_choice(id(1), &[0]),
            broke(id(1), Fault::OptionDisabled { pick: 0 }),
        );
        assert_eq!(
            lesson.answer_text(id(3), "Again."),
            broke(id(3), Fault::AwaitingVerdict),
        );
        assert_eq!(wire(&lesson), before);

        lesson
            .answer_choice(id(1), &[1])
            .expect("the right pick is allowed");
        assert_eq!(
            lesson.answer_choice(id(1), &[2]),
            broke(id(1), Fault::Solved)
        );
        ok(
            &mut lesson,
            "quiz_result",
            json!({ "quiz": "b3", "verdict": "incorrect", "feedback": "No." }),
        );
        assert_eq!(
            lesson.answer_text(id(3), " "),
            broke(id(3), Fault::EmptyAnswer)
        );
    }

    #[test]
    fn continue_is_pressed_once_on_the_newest_page() {
        let mut lesson = lesson_on_page_1();
        assert_eq!(
            lesson.press_continue(page(2)),
            Err(Misstep::UnknownPage { page: page(2) }),
        );
        assert_eq!(
            lesson.press_continue(page(1)),
            Err(Misstep::NoContinue { page: page(1) }),
        );

        ok(
            &mut lesson,
            "continue_button",
            json!({ "label": "Continue to 3.3" }),
        );
        let pressed = lesson.press_continue(page(1)).expect("the button is ready");
        assert_eq!(
            pressed,
            Acted {
                page: page(1),
                event: StudentEvent::ContinuePressed {
                    page: page(1),
                    label: "Continue to 3.3".to_owned(),
                },
            },
        );
        assert_eq!(wire(&lesson)[0]["continue_pressed"], json!(true));
        assert_eq!(
            lesson.press_continue(page(1)),
            Err(Misstep::AlreadyPressed { page: page(1) }),
        );

        // A new label makes the button ready again.
        ok(
            &mut lesson,
            "continue_button",
            json!({ "label": "Continue to 3.4" }),
        );
        assert_eq!(wire(&lesson)[0]["continue_pressed"], json!(false));

        ok(&mut lesson, "new_page", json!({}));
        assert_eq!(
            lesson.press_continue(page(1)),
            Err(Misstep::NotNewest { page: page(1) }),
        );
    }

    #[test]
    fn the_page_keeps_the_answer_key_and_unreleased_feedback_to_itself() {
        let mut lesson = lesson_with_quizzes();
        let fresh = wire(&lesson).to_string();
        assert!(!fresh.contains("correct"));
        assert!(!fresh.contains("WRONG-FEEDBACK"));
        assert!(!fresh.contains("RIGHT-FEEDBACK"));

        lesson
            .answer_choice(id(1), &[0])
            .expect("the pick is allowed");
        let after_a_wrong_pick = wire(&lesson).to_string();
        assert!(!after_a_wrong_pick.contains("correct"));
        assert!(after_a_wrong_pick.contains("WRONG-FEEDBACK"));
        assert!(!after_a_wrong_pick.contains("RIGHT-FEEDBACK"));
        // Nothing marks the right option before the student picks it.
        assert!(!after_a_wrong_pick.contains("\"right\""));
    }
}
