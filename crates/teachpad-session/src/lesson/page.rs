//! One page of a lesson: a head, a column of blocks and a foot.

use serde::Serialize;

use super::block::{Block, Body, optional_text};
use super::command;
use super::id::{BlockId, PageNumber};
use super::refusal::Flaw;

/// One page of a lesson.
///
/// A page travels to the UI in the shape it is stored in.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub(crate) struct Page {
    number: PageNumber,
    #[serde(skip_serializing_if = "Option::is_none")]
    chapter_number: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    chapter_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    section_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    section_title: Option<String>,
    /// Whether this page opens a chapter, so the big chapter title shows.
    chapter_opener: bool,
    blocks: Vec<Block>,
    footnotes: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    continue_label: Option<String>,
    continue_pressed: bool,
}

impl Page {
    /// Starts the page that follows `previous`.
    ///
    /// Chapter fields that the arguments leave out repeat those of the
    /// previous page. The page opens a chapter when it is the first page,
    /// or when its chapter differs from the previous page's.
    pub(crate) fn start(previous: Option<&Self>, args: &command::NewPage) -> Result<Self, Flaw> {
        let chapter_title = optional_text("chapter_title", args.chapter_title.as_deref())?;
        let section_number = optional_text("section_number", args.section_number.as_deref())?;
        let section_title = optional_text("section_title", args.section_title.as_deref())?;
        let chapter_number = args
            .chapter_number
            .or_else(|| previous.and_then(|page| page.chapter_number));
        let chapter_title =
            chapter_title.or_else(|| previous.and_then(|page| page.chapter_title.clone()));
        let chapter_opener = previous.is_none_or(|page| {
            page.chapter_number != chapter_number || page.chapter_title != chapter_title
        });
        Ok(Self {
            number: previous.map_or(PageNumber::FIRST, |page| page.number.next()),
            chapter_number,
            chapter_title,
            section_number,
            section_title,
            chapter_opener,
            blocks: Vec::new(),
            footnotes: Vec::new(),
            continue_label: None,
            continue_pressed: false,
        })
    }

    /// Returns the number of this page.
    pub(crate) fn number(&self) -> PageNumber {
        self.number
    }

    /// Returns the number of the chapter this page belongs to.
    pub(crate) fn chapter_number(&self) -> Option<u32> {
        self.chapter_number
    }

    /// Adds a block to the end of the page.
    pub(crate) fn push(&mut self, block: Block) {
        self.blocks.push(block);
    }

    /// Adds a note to the foot of the page and returns its number there.
    pub(crate) fn add_footnote(&mut self, note: String) -> usize {
        self.footnotes.push(note);
        self.footnotes.len()
    }

    /// Sets the label of the Continue button. A new label makes the button
    /// ready to press again.
    pub(crate) fn set_continue(&mut self, label: String) {
        self.continue_label = Some(label);
        self.continue_pressed = false;
    }

    /// Returns the label of the Continue button, when the page has one.
    pub(crate) fn continue_label(&self) -> Option<&str> {
        self.continue_label.as_deref()
    }

    /// Says whether the Continue button was pressed.
    pub(crate) fn is_continue_pressed(&self) -> bool {
        self.continue_pressed
    }

    /// Marks the Continue button as pressed.
    pub(crate) fn press_continue(&mut self) {
        self.continue_pressed = true;
    }

    /// Returns the content of the block with the given id, to change it.
    pub(crate) fn body_mut(&mut self, id: BlockId) -> Option<&mut Body> {
        self.blocks
            .iter_mut()
            .find(|block| block.id() == id)
            .map(Block::body_mut)
    }
}
