//! The frames on the wire, both ways.
//!
//! Frames are JSON objects tagged by `type`, in `snake_case`. The UI mirrors
//! them in `ui/src/protocol.ts`, and shared fixtures pin both sides, so a
//! field renamed here fails a test there.

use serde::{Deserialize, Serialize};

use crate::chat::Chat;
use crate::lesson::{BlockId, Page, PageNumber};

/// One frame from a session to the UI: a whole page, or the whole chat.
///
/// A frame is a value to serialize and send. Sending whole states means a
/// reader that missed a frame is put right by the next one.
///
/// # Examples
///
/// ```
/// use pedagogue_session::{Session, Version};
///
/// let (session, _events) = Session::new();
/// session.assistant_message("Read this section, then try the quiz.");
/// let changes = session.changes_since(Version::ZERO);
/// let wire = serde_json::to_value(&changes.frames()[0])?;
/// assert_eq!(wire["type"], "chat");
/// assert_eq!(wire["entries"][0]["kind"], "assistant");
/// # Ok::<(), serde_json::Error>(())
/// ```
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(transparent)]
pub struct Frame(Kind);

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum Kind {
    /// One whole page, with the number of pages the lesson holds.
    Page { page_count: usize, page: Page },
    /// The whole chat.
    Chat(Chat),
}

impl Frame {
    /// Makes the frame of one page.
    pub(crate) fn page(page_count: usize, page: Page) -> Self {
        Self(Kind::Page { page_count, page })
    }

    /// Makes the frame of the chat.
    pub(crate) fn chat(chat: Chat) -> Self {
        Self(Kind::Chat(chat))
    }
}

/// One frame from the UI to a session: something the student did.
///
/// A frame is parsed from JSON and handed to
/// [`Session::submit`](crate::Session::submit).
///
/// # Examples
///
/// ```
/// use pedagogue_session::StudentFrame;
///
/// let frame: StudentFrame = serde_json::from_str(r#"{ "type": "say", "text": "Why five columns?" }"#)?;
/// let unknown = serde_json::from_str::<StudentFrame>(r#"{ "type": "shout" }"#);
/// assert!(unknown.is_err());
/// # let _ = frame;
/// # Ok::<(), serde_json::Error>(())
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(transparent)]
pub struct StudentFrame(pub(crate) StudentKind);

/// What a student frame asks for.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub(crate) enum StudentKind {
    /// A chat message.
    Say { text: String },
    /// An answer to a quiz: picks for a choice quiz, or text for a
    /// free-text quiz.
    Answer {
        quiz: BlockId,
        picks: Option<Vec<usize>>,
        text: Option<String>,
    },
    /// A press of the Continue button of a page.
    Continue { page: PageNumber },
    /// A press of Regenerate under the last tutor reply.
    Regenerate,
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::BTreeMap;

    use serde_json::json;

    use crate::chat::Speaker;
    use crate::lesson::{Command, Lesson};

    /// The examples the UI tests read too, so both sides agree on the
    /// shape of every student frame.
    const STUDENT_FRAMES: &str = include_str!("../fixtures/student-frames.json");

    fn quiz(id: &str) -> BlockId {
        serde_json::from_value(json!(id)).expect("the text is a block id")
    }

    #[test]
    fn a_page_frame_holds_one_whole_page_and_the_page_count() {
        let mut lesson = Lesson::default();
        let start = json!({ "section_number": "3.2", "section_title": "Taking a Straw Poll" });
        let command = Command::parse("new_page", &start).expect("the arguments parse");
        lesson.apply(command).expect("the command applies");
        let page = lesson.pages().first().expect("the lesson has a page");

        let frame = Frame::page(1, page.clone());

        assert_eq!(
            serde_json::to_value(&frame).expect("a frame serializes"),
            json!({
                "type": "page",
                "page_count": 1,
                "page": {
                    "number": 1,
                    "section_number": "3.2",
                    "section_title": "Taking a Straw Poll",
                    "chapter_opener": true,
                    "blocks": [],
                    "footnotes": [],
                    "continue_pressed": false,
                },
            }),
        );
    }

    #[test]
    fn a_chat_frame_holds_the_whole_chat() {
        let mut chat = Chat::default();
        chat.add(Speaker::Assistant, "Read this section.", 1_795_300_000_000);

        let frame = Frame::chat(chat);

        assert_eq!(
            serde_json::to_value(&frame).expect("a frame serializes"),
            json!({
                "type": "chat",
                "state": "awaiting_input",
                "suggestions": [],
                "entries": [
                    { "kind": "assistant", "text": "Read this section.",
                      "at": 1_795_300_000_000_u64 },
                ],
            }),
        );
    }

    #[test]
    fn the_fixture_holds_one_example_of_each_student_frame() {
        let parsed: BTreeMap<String, StudentFrame> =
            serde_json::from_str(STUDENT_FRAMES).expect("every example parses");
        let expected = [
            (
                "answer_picks",
                StudentKind::Answer {
                    quiz: quiz("b1"),
                    picks: Some(vec![1]),
                    text: None,
                },
            ),
            (
                "answer_select",
                StudentKind::Answer {
                    quiz: quiz("b2"),
                    picks: Some(vec![0, 2]),
                    text: None,
                },
            ),
            (
                "answer_text",
                StudentKind::Answer {
                    quiz: quiz("b3"),
                    picks: None,
                    text: Some("It casts to an rvalue.".to_owned()),
                },
            ),
            (
                "continue",
                StudentKind::Continue {
                    page: serde_json::from_value(json!(1)).expect("1 is a page number"),
                },
            ),
            ("regenerate", StudentKind::Regenerate),
            (
                "say",
                StudentKind::Say {
                    text: "Why five columns?".to_owned(),
                },
            ),
        ];
        let expected: BTreeMap<String, StudentFrame> = expected
            .into_iter()
            .map(|(name, kind)| (name.to_owned(), StudentFrame(kind)))
            .collect();
        assert_eq!(parsed, expected);
    }

    #[test]
    fn a_student_frame_of_another_shape_is_refused() {
        let refused = [
            json!({ "type": "shout", "text": "Hello." }),
            json!({ "text": "A frame with no type." }),
            json!({ "type": "say" }),
            json!({ "type": "say", "text": "Hello.", "loud": true }),
            json!({ "type": "answer", "picks": [0] }),
            json!({ "type": "answer", "quiz": "7", "picks": [0] }),
            json!({ "type": "answer", "quiz": "b1", "picks": [-1] }),
            json!({ "type": "answer", "quiz": "b1", "text": 7 }),
            json!({ "type": "continue", "page": 0 }),
            json!({ "type": "continue", "page": "1" }),
            json!("say"),
        ];
        for frame in refused {
            assert!(
                serde_json::from_value::<StudentFrame>(frame.clone()).is_err(),
                "{frame} is taken as a student frame",
            );
        }
    }
}
