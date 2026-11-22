//! The chat transcript: what the tutor, the student and the app said, and
//! what the composer shows.

use serde::Serialize;

/// Whether the tutor is at work on a reply.
///
/// While the tutor works, the UI shows the typing dots and locks the
/// composer.
///
/// # Examples
///
/// ```
/// use pedagogue_session::{ChatState, Session};
///
/// let (session, _events) = Session::new();
/// session.set_state(ChatState::Working);
/// session.assistant_message("Read this section, then try the quiz.");
/// session.set_state(ChatState::AwaitingInput);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ChatState {
    /// The tutor is writing a reply.
    Working,
    /// The chat waits for the student. A new session starts here.
    #[default]
    AwaitingInput,
}

/// Who an entry of the chat comes from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum Speaker {
    /// The tutor: a gray bubble on the left.
    Assistant,
    /// The student: a blue bubble on the right.
    User,
    /// The app: a centered line, such as a milestone.
    Notice,
}

/// One entry of the chat.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct Entry {
    kind: Speaker,
    /// The text, in markdown.
    text: String,
    /// When the entry was added, in milliseconds since the Unix epoch.
    at: u64,
}

/// The chat of one session. It travels to the UI in the shape it is stored
/// in.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize)]
pub(crate) struct Chat {
    state: ChatState,
    /// The chips above the composer. A chip sends its text as a message.
    suggestions: Vec<String>,
    entries: Vec<Entry>,
}

impl Chat {
    /// Adds an entry and says whether the chat changed. Text that holds
    /// only whitespace adds nothing.
    pub(crate) fn add(&mut self, kind: Speaker, text: &str, at: u64) -> bool {
        let text = text.trim();
        if text.is_empty() {
            return false;
        }
        self.entries.push(Entry {
            kind,
            text: text.to_owned(),
            at,
        });
        true
    }

    /// Sets whether the tutor is at work, and says whether that changed.
    pub(crate) fn set_state(&mut self, state: ChatState) -> bool {
        let changed = self.state != state;
        self.state = state;
        changed
    }

    /// Replaces the chips, and says whether they changed. Empty chips are
    /// left out.
    pub(crate) fn set_suggestions(
        &mut self,
        suggestions: impl IntoIterator<Item = String>,
    ) -> bool {
        let suggestions: Vec<String> = suggestions
            .into_iter()
            .map(|chip| chip.trim().to_owned())
            .filter(|chip| !chip.is_empty())
            .collect();
        let changed = self.suggestions != suggestions;
        self.suggestions = suggestions;
        changed
    }

    /// Says whether the tutor has replied at least once.
    pub(crate) fn has_reply(&self) -> bool {
        self.entries
            .iter()
            .any(|entry| entry.kind == Speaker::Assistant)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde_json::json;

    #[test]
    fn a_new_chat_awaits_input_and_holds_nothing() {
        let chat = Chat::default();
        assert_eq!(
            serde_json::to_value(&chat).expect("a chat serializes"),
            json!({ "state": "awaiting_input", "suggestions": [], "entries": [] }),
        );
        assert!(!chat.has_reply());
    }

    #[test]
    fn entries_keep_their_order_their_speaker_and_their_time() {
        let mut chat = Chat::default();
        assert!(chat.add(Speaker::Assistant, " Read this section. ", 1000));
        assert!(chat.add(Speaker::User, "Why five columns?", 2000));
        assert!(chat.add(Speaker::Notice, "**Milestone 2 unlocked**", 3000));
        assert!(!chat.add(Speaker::Assistant, "  \n", 4000));
        assert_eq!(
            serde_json::to_value(&chat).expect("a chat serializes")["entries"],
            json!([
                { "kind": "assistant", "text": "Read this section.", "at": 1000 },
                { "kind": "user", "text": "Why five columns?", "at": 2000 },
                { "kind": "notice", "text": "**Milestone 2 unlocked**", "at": 3000 },
            ]),
        );
        assert!(chat.has_reply());
    }

    #[test]
    fn a_setter_says_whether_it_changed_anything() {
        let mut chat = Chat::default();
        assert!(!chat.set_state(ChatState::AwaitingInput));
        assert!(chat.set_state(ChatState::Working));
        assert!(chat.set_suggestions(vec![" Take the quiz ".to_owned(), " ".to_owned()]));
        assert!(!chat.set_suggestions(vec!["Take the quiz".to_owned()]));
        assert_eq!(chat.suggestions, ["Take the quiz"]);
    }
}
