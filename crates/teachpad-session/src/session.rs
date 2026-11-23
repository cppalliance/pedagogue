//! The session handle: one lesson and its chat behind one lock.

use std::collections::BTreeMap;
use std::fmt;
use std::sync::{Arc, Mutex, PoisonError};
use std::time::{SystemTime, UNIX_EPOCH};

use serde_json::Value;
use tokio::sync::{mpsc, watch};

use crate::changes::{Changes, Version};
use crate::chat::{Chat, ChatState, Speaker};
use crate::error::{ActionError, CommandError};
use crate::event::StudentEvent;
use crate::frames::{Frame, StudentFrame, StudentKind};
use crate::lesson::{BlockId, Command, Lesson, Misstep, PageNumber, Refusal};
use crate::wording;

/// How many student events can wait for the host before the next student
/// action has to wait for room.
const EVENT_ROOM: usize = 64;

/// A handle to one lesson and its chat.
///
/// A host drives the session through this handle: it applies page commands,
/// writes into the chat, and reads what the student did from the event
/// stream. A server uses it too: it asks what changed, and it passes in
/// what the student does. The handle is cheap to clone, and every clone
/// drives the same session.
///
/// # Examples
///
/// ```
/// use teachpad_session::Session;
/// use serde_json::json;
///
/// let (session, _events) = Session::new();
/// let receipt = session.apply("new_page", &json!({ "section_title": "Taking a Straw Poll" }))?;
/// assert_eq!(receipt, "Started page 1.");
/// let receipt = session.apply("exposition", &json!({ "markdown": "The chair states the question." }))?;
/// assert_eq!(receipt, "Added exposition b1 to page 1.");
/// # Ok::<(), teachpad_session::CommandError>(())
/// ```
#[derive(Clone)]
pub struct Session {
    shared: Arc<Shared>,
}

struct Shared {
    state: Mutex<State>,
    /// Wakes readers. It holds the newest version, set after each change.
    wake: watch::Sender<Version>,
    events: mpsc::Sender<StudentEvent>,
    clock: Box<dyn Fn() -> SystemTime + Send + Sync>,
}

/// Everything a session holds, with the version at which each part last
/// changed.
struct State {
    lesson: Lesson,
    chat: Chat,
    /// The version of the session. Every change raises it.
    version: Version,
    page_versions: BTreeMap<PageNumber, Version>,
    chat_version: Version,
}

impl State {
    fn new() -> Self {
        // The chat counts as changed from the start, so a first read always
        // gets a chat frame.
        let first = Version::ZERO.next();
        Self {
            lesson: Lesson::default(),
            chat: Chat::default(),
            version: first,
            page_versions: BTreeMap::new(),
            chat_version: first,
        }
    }

    fn touch_page(&mut self, page: PageNumber) {
        self.version = self.version.next();
        self.page_versions.insert(page, self.version);
    }

    fn touch_chat(&mut self) {
        self.version = self.version.next();
        self.chat_version = self.version;
    }
}

impl Session {
    /// Makes a session with an empty lesson and an empty chat, and returns
    /// it with the stream of what the student does.
    ///
    /// Chat entries are stamped with the system time.
    ///
    /// # Examples
    ///
    /// ```
    /// use teachpad_session::{Session, Version};
    ///
    /// let (session, _events) = Session::new();
    /// // A new session has its chat to offer, and no page yet.
    /// assert_eq!(session.changes_since(Version::ZERO).frames().len(), 1);
    /// ```
    #[must_use]
    pub fn new() -> (Self, StudentEvents) {
        Self::with_clock(SystemTime::now)
    }

    /// Makes a session that stamps chat entries with the time `clock`
    /// returns, so a test or a fixture gets the same stamps on every run.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::time::{Duration, UNIX_EPOCH};
    ///
    /// use teachpad_session::{Session, Version};
    ///
    /// let (session, _events) =
    ///     Session::with_clock(|| UNIX_EPOCH + Duration::from_secs(1_795_300_000));
    /// session.notice("**Milestone 2 unlocked**");
    /// let changes = session.changes_since(Version::ZERO);
    /// let wire = serde_json::to_value(changes.frames())?;
    /// assert_eq!(wire[0]["entries"][0]["at"], 1_795_300_000_000_u64);
    /// # Ok::<(), serde_json::Error>(())
    /// ```
    #[must_use]
    pub fn with_clock<C>(clock: C) -> (Self, StudentEvents)
    where
        C: Fn() -> SystemTime + Send + Sync + 'static,
    {
        let state = State::new();
        let (wake, _) = watch::channel(state.version);
        let (events, receiver) = mpsc::channel(EVENT_ROOM);
        let session = Self {
            shared: Arc::new(Shared {
                state: Mutex::new(state),
                wake,
                events,
                clock: Box::new(clock),
            }),
        };
        (session, StudentEvents { receiver })
    }

    /// Applies one page command and returns its receipt.
    ///
    /// This is the shape of a tool call: the name of a tool and one JSON
    /// object in, one short sentence out. The receipt of a command that
    /// adds a block names the block's id.
    ///
    /// # Errors
    ///
    /// Returns a [`CommandError`] when no command has the name `tool`, when
    /// `args` have the wrong shape or break a rule of the command, or when
    /// the lesson as it stands has no place for the command. The session
    /// stays as it was.
    ///
    /// # Examples
    ///
    /// ```
    /// use teachpad_session::Session;
    /// use serde_json::json;
    ///
    /// let (session, _events) = Session::new();
    /// session.apply("new_page", &json!({ "chapter_number": 3 }))?;
    /// let receipt = session.apply(
    ///     "pop_quiz",
    ///     &json!({ "question": "What should the chair do first?", "options": [
    ///         { "text": "Restate the question", "correct": true },
    ///         { "text": "Count the room", "correct": false },
    ///     ] }),
    /// )?;
    /// assert_eq!(receipt, "Added pop quiz b1 to page 1.");
    /// # Ok::<(), teachpad_session::CommandError>(())
    /// ```
    pub fn apply(&self, tool: &str, args: &Value) -> Result<String, CommandError> {
        let command = Command::parse(tool, args)?;
        let receipt = self.lock(|state| {
            let receipt = state.lesson.apply(command)?;
            state.touch_page(receipt.page());
            Ok::<_, Refusal>(receipt)
        })?;
        Ok(wording::receipt(&receipt))
    }

    /// Adds a tutor reply to the chat: a gray bubble on the left.
    ///
    /// The text is markdown. Text that holds only whitespace adds nothing.
    ///
    /// # Examples
    ///
    /// ```
    /// use teachpad_session::{Session, Version};
    ///
    /// let (session, _events) = Session::new();
    /// session.assistant_message("Read this section, then try the quiz.");
    /// let changes = session.changes_since(Version::ZERO);
    /// let wire = serde_json::to_value(changes.frames())?;
    /// assert_eq!(wire[0]["entries"][0]["kind"], "assistant");
    /// # Ok::<(), serde_json::Error>(())
    /// ```
    pub fn assistant_message(&self, text: &str) {
        self.add_entry(Speaker::Assistant, text);
    }

    /// Adds a centered line to the chat, such as a milestone.
    ///
    /// The text is markdown, and a bold lead stands out. Text that holds
    /// only whitespace adds nothing.
    ///
    /// # Examples
    ///
    /// ```
    /// use teachpad_session::{Session, Version};
    ///
    /// let (session, _events) = Session::new();
    /// session.notice("**Milestone 2 unlocked** - Section 2.2 is ready");
    /// let changes = session.changes_since(Version::ZERO);
    /// let wire = serde_json::to_value(changes.frames())?;
    /// assert_eq!(wire[0]["entries"][0]["kind"], "notice");
    /// # Ok::<(), serde_json::Error>(())
    /// ```
    pub fn notice(&self, text: &str) {
        self.add_entry(Speaker::Notice, text);
    }

    /// Says whether the tutor is at work on a reply.
    ///
    /// While the tutor works, the UI shows the typing dots and locks the
    /// composer. A new session awaits input.
    ///
    /// # Examples
    ///
    /// ```
    /// use teachpad_session::{ChatState, Session, Version};
    ///
    /// let (session, _events) = Session::new();
    /// session.set_state(ChatState::Working);
    /// let changes = session.changes_since(Version::ZERO);
    /// let wire = serde_json::to_value(changes.frames())?;
    /// assert_eq!(wire[0]["state"], "working");
    /// # Ok::<(), serde_json::Error>(())
    /// ```
    pub fn set_state(&self, state: ChatState) {
        self.lock(|held| {
            if held.chat.set_state(state) {
                held.touch_chat();
            }
        });
    }

    /// Replaces the chips above the composer.
    ///
    /// A chip sends its text as a message from the student. Empty chips
    /// are left out.
    ///
    /// # Examples
    ///
    /// ```
    /// use teachpad_session::{Session, Version};
    ///
    /// let (session, _events) = Session::new();
    /// session.set_suggestions(["Take the quiz", "Explain this page"]);
    /// let changes = session.changes_since(Version::ZERO);
    /// let wire = serde_json::to_value(changes.frames())?;
    /// assert_eq!(wire[0]["suggestions"][1], "Explain this page");
    /// # Ok::<(), serde_json::Error>(())
    /// ```
    pub fn set_suggestions<I>(&self, suggestions: I)
    where
        I: IntoIterator,
        I::Item: Into<String>,
    {
        let suggestions: Vec<String> = suggestions.into_iter().map(Into::into).collect();
        self.lock(|state| {
            if state.chat.set_suggestions(suggestions) {
                state.touch_chat();
            }
        });
    }

    /// Records a chat message from the student: a blue bubble on the
    /// right.
    ///
    /// The host hears a [`StudentEvent::UserMessage`]. The call waits while
    /// the event stream is full.
    ///
    /// # Errors
    ///
    /// Returns an [`ActionError`] when `text` holds only whitespace.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() -> Result<(), teachpad_session::ActionError> {
    /// use teachpad_session::{Session, StudentEvent};
    ///
    /// let (session, mut events) = Session::new();
    /// session.say("Why five columns?").await?;
    /// let heard = events.recv().await;
    /// assert_eq!(
    ///     heard,
    ///     Some(StudentEvent::UserMessage { text: "Why five columns?".to_owned() }),
    /// );
    /// # Ok(())
    /// # }
    /// ```
    pub async fn say(&self, text: &str) -> Result<(), ActionError> {
        let text = text.trim();
        self.act(|state, now| {
            if !state.chat.add(Speaker::User, text, now) {
                return Err(Misstep::EmptyMessage);
            }
            state.touch_chat();
            Ok(StudentEvent::UserMessage {
                text: text.to_owned(),
            })
        })
        .await
    }

    /// Records the options a student picked on a multiple-choice quiz.
    ///
    /// The picks are option indexes from 0, in any order. The page marks
    /// them at once, and the host hears a [`StudentEvent::QuizAnswered`]
    /// that says whether they solved the quiz. The call waits while the
    /// event stream is full.
    ///
    /// # Errors
    ///
    /// Returns an [`ActionError`] when `quiz` names no multiple-choice
    /// quiz, when the quiz is solved, or when the picks fit no open option
    /// of it.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use teachpad_session::{QuizAnswer, Session, StudentEvent};
    /// use serde_json::json;
    ///
    /// let (session, mut events) = Session::new();
    /// session.apply("new_page", &json!({}))?;
    /// session.apply(
    ///     "pop_quiz",
    ///     &json!({ "question": "What should the chair do first?", "options": [
    ///         { "text": "Restate the question", "correct": true },
    ///         { "text": "Count the room", "correct": false },
    ///     ] }),
    /// )?;
    /// let quiz = serde_json::from_str("\"b1\"")?;
    /// session.answer_choice(quiz, &[1]).await?;
    /// let Some(StudentEvent::QuizAnswered { answer, .. }) = events.recv().await else {
    ///     panic!("the host hears every attempt");
    /// };
    /// assert!(matches!(answer, QuizAnswer::Picks { correct: false, .. }));
    /// # Ok(())
    /// # }
    /// ```
    pub async fn answer_choice(&self, quiz: BlockId, picks: &[usize]) -> Result<(), ActionError> {
        self.act(|state, _now| {
            let acted = state.lesson.answer_choice(quiz, picks)?;
            state.touch_page(acted.page);
            Ok(acted.event)
        })
        .await
    }

    /// Records the text a student sent to a free-text quiz.
    ///
    /// The answer waits there for the host's verdict, which is the
    /// `quiz_result` command. The host hears a
    /// [`StudentEvent::QuizAnswered`]. The call waits while the event
    /// stream is full.
    ///
    /// # Errors
    ///
    /// Returns an [`ActionError`] when `quiz` names no free-text quiz, when
    /// `text` holds only whitespace, when an earlier answer still waits for
    /// its verdict, or when the quiz was marked correct.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use teachpad_session::Session;
    /// use serde_json::json;
    ///
    /// let (session, _events) = Session::new();
    /// session.apply("new_page", &json!({}))?;
    /// session.apply("short_answer", &json!({ "question": "What does `std::move` do?" }))?;
    /// let quiz = serde_json::from_str("\"b1\"")?;
    /// session.answer_text(quiz, "It casts to an rvalue.").await?;
    /// let receipt = session.apply(
    ///     "quiz_result",
    ///     &json!({ "quiz": "b1", "verdict": "correct", "feedback": "Exactly." }),
    /// )?;
    /// assert_eq!(receipt, "Marked quiz b1 correct.");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn answer_text(&self, quiz: BlockId, text: &str) -> Result<(), ActionError> {
        self.act(|state, _now| {
            let acted = state.lesson.answer_text(quiz, text)?;
            state.touch_page(acted.page);
            Ok(acted.event)
        })
        .await
    }

    /// Records a press of the Continue button at the foot of a page.
    ///
    /// The host hears a [`StudentEvent::ContinuePressed`], and the usual
    /// answer to it is the `new_page` command. The call waits while the
    /// event stream is full.
    ///
    /// # Errors
    ///
    /// Returns an [`ActionError`] when `page` is not the newest page, when
    /// it has no Continue button, or when the button was pressed already.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use teachpad_session::{Session, StudentEvent};
    /// use serde_json::json;
    ///
    /// let (session, mut events) = Session::new();
    /// session.apply("new_page", &json!({}))?;
    /// session.apply("continue_button", &json!({ "label": "Continue to 3.3" }))?;
    /// let page = serde_json::from_str("1")?;
    /// session.press_continue(page).await?;
    /// let heard = events.recv().await;
    /// assert_eq!(
    ///     heard,
    ///     Some(StudentEvent::ContinuePressed { page, label: "Continue to 3.3".to_owned() }),
    /// );
    /// # Ok(())
    /// # }
    /// ```
    pub async fn press_continue(&self, page: PageNumber) -> Result<(), ActionError> {
        self.act(|state, _now| {
            let acted = state.lesson.press_continue(page)?;
            state.touch_page(acted.page);
            Ok(acted.event)
        })
        .await
    }

    /// Records that the student asked for the last tutor reply again.
    ///
    /// The host hears a [`StudentEvent::RegeneratePressed`]. The call waits
    /// while the event stream is full.
    ///
    /// # Errors
    ///
    /// Returns an [`ActionError`] when the tutor has yet to reply.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() -> Result<(), teachpad_session::ActionError> {
    /// use teachpad_session::{Session, StudentEvent};
    ///
    /// let (session, mut events) = Session::new();
    /// session.assistant_message("Read this section, then try the quiz.");
    /// session.regenerate().await?;
    /// assert_eq!(events.recv().await, Some(StudentEvent::RegeneratePressed));
    /// # Ok(())
    /// # }
    /// ```
    pub async fn regenerate(&self) -> Result<(), ActionError> {
        self.act(|state, _now| {
            if !state.chat.has_reply() {
                return Err(Misstep::NoReply);
            }
            Ok(StudentEvent::RegeneratePressed)
        })
        .await
    }

    /// Records what one frame from the UI says the student did.
    ///
    /// # Errors
    ///
    /// Returns an [`ActionError`] when the action the frame asks for is
    /// refused: see [`Session::say`], [`Session::answer_choice`],
    /// [`Session::answer_text`], [`Session::press_continue`] and
    /// [`Session::regenerate`]. An answer frame that holds both picks and
    /// text, or neither, is refused too.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use teachpad_session::{Session, StudentEvent};
    ///
    /// let (session, mut events) = Session::new();
    /// let frame = serde_json::from_str(r#"{ "type": "say", "text": "Why five columns?" }"#)?;
    /// session.submit(frame).await?;
    /// assert!(matches!(events.recv().await, Some(StudentEvent::UserMessage { .. })));
    /// # Ok(())
    /// # }
    /// ```
    pub async fn submit(&self, frame: StudentFrame) -> Result<(), ActionError> {
        match frame.0 {
            StudentKind::Say { text } => self.say(&text).await,
            StudentKind::Answer {
                quiz,
                picks: Some(picks),
                text: None,
            } => self.answer_choice(quiz, &picks).await,
            StudentKind::Answer {
                quiz,
                picks: None,
                text: Some(text),
            } => self.answer_text(quiz, &text).await,
            StudentKind::Answer { quiz, .. } => Err(Misstep::AnswerShape { quiz }.into()),
            StudentKind::Continue { page } => self.press_continue(page).await,
            StudentKind::Regenerate => self.regenerate().await,
        }
    }

    /// Returns what changed since a cursor: each page that changed, in
    /// order, then the chat if it changed.
    ///
    /// A reader starts from [`Version::ZERO`], which returns everything,
    /// and takes its next cursor from [`Changes::version`].
    ///
    /// # Examples
    ///
    /// ```
    /// use teachpad_session::{Session, Version};
    /// use serde_json::json;
    ///
    /// let (session, _events) = Session::new();
    /// let cursor = session.changes_since(Version::ZERO).version();
    /// session.apply("new_page", &json!({}))?;
    /// let changes = session.changes_since(cursor);
    /// let wire = serde_json::to_value(changes.frames()).expect("frames serialize");
    /// assert_eq!(wire[0]["type"], "page");
    /// assert_eq!(wire[0]["page"]["number"], 1);
    /// # Ok::<(), teachpad_session::CommandError>(())
    /// ```
    #[must_use]
    pub fn changes_since(&self, cursor: Version) -> Changes {
        self.lock(|state| {
            let page_count = state.lesson.pages().len();
            let changed = |version: Option<&Version>| version.is_some_and(|at| *at > cursor);
            let mut frames: Vec<Frame> = state
                .lesson
                .pages()
                .iter()
                .filter(|page| changed(state.page_versions.get(&page.number())))
                .map(|page| Frame::page(page_count, page.clone()))
                .collect();
            if state.chat_version > cursor {
                frames.push(Frame::chat(state.chat.clone()));
            }
            Changes::new(state.version, frames)
        })
    }

    /// Waits until the session is past a cursor, then returns what changed
    /// since.
    ///
    /// It returns at once when the session is past the cursor already.
    /// Dropping the future before it finishes loses nothing, since the
    /// caller keeps the cursor.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() {
    /// use teachpad_session::{Session, Version};
    ///
    /// let (session, _events) = Session::new();
    /// let seen = session.wait_for_changes(Version::ZERO).await.version();
    /// session.assistant_message("Read this section, then try the quiz.");
    /// let changes = session.wait_for_changes(seen).await;
    /// assert_eq!(changes.frames().len(), 1);
    /// # }
    /// ```
    pub async fn wait_for_changes(&self, cursor: Version) -> Changes {
        let mut wake = self.shared.wake.subscribe();
        // The sender lives in `self`, so this wait ends only when a change
        // lands, never because the channel closed.
        let _ = wake.wait_for(|version| *version > cursor).await;
        self.changes_since(cursor)
    }

    fn add_entry(&self, kind: Speaker, text: &str) {
        let now = self.now();
        self.lock(|state| {
            if state.chat.add(kind, text, now) {
                state.touch_chat();
            }
        });
    }

    /// Applies a student action and sends its event to the host. The action
    /// gets the state and the time on the session's clock.
    ///
    /// Room on the event channel comes first, with the lock free. The event
    /// is then sent under the lock, so events keep the order of the changes
    /// they report. A closed channel means the host is gone, and the action
    /// still applies.
    async fn act(
        &self,
        action: impl FnOnce(&mut State, u64) -> Result<StudentEvent, Misstep>,
    ) -> Result<(), ActionError> {
        let room = self.shared.events.reserve().await.ok();
        let now = self.now();
        self.lock(|state| {
            let event = action(state, now)?;
            if let Some(room) = room {
                room.send(event);
            }
            Ok::<(), Misstep>(())
        })
        .map_err(ActionError::from)
    }

    /// Runs `work` on the state, then wakes readers if the version rose.
    ///
    /// A poisoned lock gives its state back: every change checks its input
    /// before it writes, so a panic leaves the state whole.
    fn lock<T>(&self, work: impl FnOnce(&mut State) -> T) -> T {
        let (result, version) = {
            let mut state = self
                .shared
                .state
                .lock()
                .unwrap_or_else(PoisonError::into_inner);
            let result = work(&mut state);
            (result, state.version)
        };
        // Two writers can reach this point out of order, so the wake-up
        // only ever moves forward.
        self.shared.wake.send_if_modified(|seen| {
            let rose = *seen < version;
            if rose {
                *seen = version;
            }
            rose
        });
        result
    }

    /// Returns the time on the session's clock, in milliseconds since the
    /// Unix epoch.
    fn now(&self) -> u64 {
        let since_epoch = (self.shared.clock)()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default();
        u64::try_from(since_epoch.as_millis()).unwrap_or(u64::MAX)
    }
}

impl fmt::Debug for Session {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Session")
            .field("version", &self.lock(|state| state.version))
            .finish_non_exhaustive()
    }
}

/// The stream of what the student does, for the one host of a session.
///
/// # Examples
///
/// ```
/// # #[tokio::main(flavor = "current_thread")]
/// # async fn main() -> Result<(), teachpad_session::ActionError> {
/// use teachpad_session::Session;
///
/// let (session, mut events) = Session::new();
/// session.say("Why five columns?").await?;
/// let event = events.recv().await.expect("the session is alive");
/// assert_eq!(event.to_model_text(), "Why five columns?");
/// # Ok(())
/// # }
/// ```
#[derive(Debug)]
pub struct StudentEvents {
    receiver: mpsc::Receiver<StudentEvent>,
}

impl StudentEvents {
    /// Waits for the next thing the student does.
    ///
    /// Returns `None` once every handle of the session is gone. Dropping
    /// the future before it finishes loses nothing.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() {
    /// use teachpad_session::Session;
    ///
    /// let (session, mut events) = Session::new();
    /// drop(session);
    /// assert_eq!(events.recv().await, None);
    /// # }
    /// ```
    pub async fn recv(&mut self) -> Option<StudentEvent> {
        self.receiver.recv().await
    }
}

// A session is shared between tasks on any thread.
const _: () = {
    const fn shared_between_threads<T: Send + Sync>() {}
    shared_between_threads::<Session>();
    shared_between_threads::<StudentEvents>();
};

#[cfg(test)]
mod tests {
    use super::*;

    use std::time::Duration;

    use serde_json::json;

    use crate::event::QuizAnswer;

    const NOW_MS: u64 = 1_795_300_000_000;

    fn session() -> (Session, StudentEvents) {
        Session::with_clock(|| UNIX_EPOCH + Duration::from_millis(NOW_MS))
    }

    /// A session with a pop quiz `b1`, a short-answer quiz `b2`, a Continue
    /// button and one tutor reply.
    fn session_with_quizzes() -> (Session, StudentEvents) {
        let (session, events) = session();
        let calls = [
            ("new_page", json!({})),
            (
                "pop_quiz",
                json!({ "question": "Which?", "options": [
                    { "text": "This", "correct": true },
                    { "text": "That", "correct": false },
                ] }),
            ),
            ("short_answer", json!({ "question": "Why?" })),
            ("continue_button", json!({ "label": "Continue to 3.3" })),
        ];
        for (tool, args) in calls {
            session.apply(tool, &args).expect("the command applies");
        }
        session.assistant_message("Read this section.");
        (session, events)
    }

    fn id(text: &str) -> BlockId {
        serde_json::from_value(json!(text)).expect("the text is a block id")
    }

    fn page(number: u32) -> PageNumber {
        serde_json::from_value(json!(number)).expect("the number is a page number")
    }

    /// Returns the frames that changed since a cursor, as JSON, with the
    /// reader's next cursor.
    fn read(session: &Session, cursor: Version) -> (Vec<serde_json::Value>, Version) {
        let changes = session.changes_since(cursor);
        let frames = changes
            .frames()
            .iter()
            .map(|frame| serde_json::to_value(frame).expect("a frame serializes"))
            .collect();
        (frames, changes.version())
    }

    /// Says what kind of frame each one is, such as `page 2` or `chat`.
    fn kinds(frames: &[serde_json::Value]) -> Vec<String> {
        frames
            .iter()
            .map(|frame| match frame["type"].as_str() {
                Some("page") => format!("page {}", frame["page"]["number"]),
                Some(other) => other.to_owned(),
                None => "untyped".to_owned(),
            })
            .collect()
    }

    #[test]
    fn a_first_reader_gets_the_chat_of_a_new_session() {
        let (session, _events) = session();
        let (frames, cursor) = read(&session, Version::ZERO);
        assert_eq!(
            frames,
            [
                json!({ "type": "chat", "state": "awaiting_input", "suggestions": [], "entries": [] })
            ],
        );
        assert!(cursor > Version::ZERO);
        assert!(read(&session, cursor).0.is_empty());
    }

    #[test]
    fn a_cursor_sees_only_what_changed_since() {
        let (session, _events) = session();
        let (_, start) = read(&session, Version::ZERO);

        session
            .apply("new_page", &json!({}))
            .expect("the command applies");
        let (frames, after_page) = read(&session, start);
        assert_eq!(kinds(&frames), ["page 1"]);
        assert!(after_page > start);

        session.assistant_message("Read this section.");
        let (frames, after_chat) = read(&session, after_page);
        assert_eq!(kinds(&frames), ["chat"]);

        session
            .apply("exposition", &json!({ "markdown": "Text." }))
            .expect("the command applies");
        session
            .apply("new_page", &json!({}))
            .expect("the command applies");
        let (frames, _) = read(&session, after_chat);
        assert_eq!(kinds(&frames), ["page 1", "page 2"]);
        assert_eq!(frames[0]["page_count"], json!(2));

        // A reader that starts over gets every page, then the chat.
        let (frames, _) = read(&session, Version::ZERO);
        assert_eq!(kinds(&frames), ["page 1", "page 2", "chat"]);
    }

    #[test]
    fn a_call_that_changes_nothing_leaves_the_version_alone() {
        let (session, _events) = session();
        let (_, before) = read(&session, Version::ZERO);

        assert!(
            session
                .apply("exposition", &json!({ "markdown": "Text." }))
                .is_err()
        );
        session.set_state(ChatState::AwaitingInput);
        session.set_suggestions(Vec::<String>::new());
        session.assistant_message("  ");
        session.notice("");

        assert_eq!(read(&session, Version::ZERO).1, before);
    }

    #[test]
    fn the_chat_functions_fill_the_chat_frame_and_stamp_each_entry() {
        let (session, _events) = session();
        session.set_state(ChatState::Working);
        session.assistant_message("Read this section, then try the quiz.");
        session.notice("**Milestone 2 unlocked** - Section 2.2 is ready");
        session.set_suggestions(["Take the quiz"]);
        session.set_state(ChatState::AwaitingInput);

        let (frames, _) = read(&session, Version::ZERO);
        assert_eq!(
            frames,
            [json!({
                "type": "chat",
                "state": "awaiting_input",
                "suggestions": ["Take the quiz"],
                "entries": [
                    { "kind": "assistant", "text": "Read this section, then try the quiz.",
                      "at": NOW_MS },
                    { "kind": "notice", "text": "**Milestone 2 unlocked** - Section 2.2 is ready",
                      "at": NOW_MS },
                ],
            })],
        );
    }

    #[tokio::test]
    async fn say_adds_a_user_entry_and_tells_the_host() {
        let (session, mut events) = session();
        let (_, cursor) = read(&session, Version::ZERO);

        session
            .say(" Why five columns? ")
            .await
            .expect("a message with text is taken");

        assert_eq!(
            events.recv().await,
            Some(StudentEvent::UserMessage {
                text: "Why five columns?".to_owned()
            }),
        );
        let (frames, _) = read(&session, cursor);
        assert_eq!(
            frames[0]["entries"],
            json!([{ "kind": "user", "text": "Why five columns?", "at": NOW_MS }]),
        );
    }

    #[tokio::test]
    async fn each_quiz_attempt_changes_the_page_and_tells_the_host() {
        let (session, mut events) = session_with_quizzes();
        let (_, cursor) = read(&session, Version::ZERO);

        session
            .answer_choice(id("b1"), &[1])
            .await
            .expect("the pick is allowed");
        let Some(StudentEvent::QuizAnswered {
            quiz,
            attempt,
            answer,
        }) = events.recv().await
        else {
            panic!("the host hears the attempt");
        };
        assert_eq!((quiz, attempt), (id("b1"), 1));
        assert!(matches!(answer, QuizAnswer::Picks { correct: false, .. }));

        session
            .answer_text(id("b2"), "Because.")
            .await
            .expect("the quiz is open");
        assert!(matches!(
            events.recv().await,
            Some(StudentEvent::QuizAnswered {
                answer: QuizAnswer::Text { .. },
                ..
            })
        ));

        let (frames, _) = read(&session, cursor);
        assert_eq!(kinds(&frames), ["page 1"]);
        assert_eq!(
            frames[0]["page"]["blocks"][0]["options"][1]["state"],
            json!("disabled")
        );
        assert_eq!(frames[0]["page"]["blocks"][1]["status"], json!("submitted"));
    }

    #[tokio::test]
    async fn continue_and_regenerate_tell_the_host() {
        let (session, mut events) = session_with_quizzes();
        let (_, cursor) = read(&session, Version::ZERO);

        session
            .press_continue(page(1))
            .await
            .expect("the button is ready");
        assert_eq!(
            events.recv().await,
            Some(StudentEvent::ContinuePressed {
                page: page(1),
                label: "Continue to 3.3".to_owned()
            }),
        );
        let (frames, after_press) = read(&session, cursor);
        assert_eq!(frames[0]["page"]["continue_pressed"], json!(true));

        session.regenerate().await.expect("the tutor has replied");
        assert_eq!(events.recv().await, Some(StudentEvent::RegeneratePressed));
        // Asking again changes nothing on the page or in the chat.
        assert_eq!(read(&session, Version::ZERO).1, after_press);
    }

    #[tokio::test]
    async fn a_frame_from_the_ui_takes_the_same_path() {
        let (session, mut events) = session_with_quizzes();
        let frames = [
            json!({ "type": "say", "text": "Why five columns?" }),
            json!({ "type": "answer", "quiz": "b1", "picks": [0] }),
            json!({ "type": "answer", "quiz": "b2", "text": "Because." }),
            json!({ "type": "continue", "page": 1 }),
            json!({ "type": "regenerate" }),
        ];
        for frame in frames {
            let frame = serde_json::from_value(frame).expect("the frame parses");
            session.submit(frame).await.expect("the action is taken");
        }
        let mut heard = Vec::new();
        for _ in 0..5 {
            heard.push(events.recv().await.expect("an event for each frame"));
        }
        let lines: Vec<String> = heard.iter().map(StudentEvent::to_model_text).collect();
        assert_eq!(
            lines,
            [
                "Why five columns?",
                "Quiz b1, attempt 1: chose A \"This\". Correct.",
                "Quiz b2 (short answer), attempt 1: the student wrote \"Because.\"",
                "The student pressed \"Continue to 3.3\" on page 1.",
                "The student asked for the last reply again.",
            ],
        );
    }

    #[tokio::test]
    async fn a_refused_action_changes_nothing_and_tells_no_one() {
        let (session, mut events) = session();
        let (_, before) = read(&session, Version::ZERO);
        let both = serde_json::from_value(
            json!({ "type": "answer", "quiz": "b1", "picks": [0], "text": "Both." }),
        )
        .expect("the frame parses");
        let neither = serde_json::from_value(json!({ "type": "answer", "quiz": "b1" }))
            .expect("the frame parses");

        let refused = [
            session.say("  ").await,
            session.regenerate().await,
            session.answer_choice(id("b1"), &[0]).await,
            session.answer_text(id("b1"), "Text.").await,
            session.press_continue(page(1)).await,
            session.submit(both).await,
            session.submit(neither).await,
        ];
        let reasons: Vec<String> = refused
            .into_iter()
            .map(|outcome| outcome.expect_err("the action is refused").to_string())
            .collect();
        assert_eq!(
            reasons,
            [
                "the message is empty",
                "the tutor has given no reply to repeat",
                "no quiz has the id `b1`",
                "no quiz has the id `b1`",
                "no page has the number 1",
                "the answer to quiz `b1` holds neither picks alone nor text alone",
                "the answer to quiz `b1` holds neither picks alone nor text alone",
            ],
        );

        assert_eq!(read(&session, Version::ZERO).1, before);
        drop(session);
        assert_eq!(events.recv().await, None);
    }

    #[tokio::test]
    async fn an_action_waits_for_room_on_a_full_event_stream() {
        let (session, mut events) = session();
        for _ in 0..EVENT_ROOM {
            session.say("Again.").await.expect("there is room");
        }
        let (_, full) = read(&session, Version::ZERO);

        // With the stream full, one more message waits. Dropping the wait
        // leaves the chat as it was.
        tokio::select! {
            biased;
            _ = session.say("One more.") => panic!("the stream is full"),
            () = tokio::task::yield_now() => {}
        }
        assert_eq!(read(&session, Version::ZERO).1, full);

        events.recv().await.expect("the stream holds events");
        session.say("One more.").await.expect("there is room again");
    }

    #[tokio::test]
    async fn actions_still_apply_after_the_host_is_gone() {
        let (session, events) = session();
        drop(events);
        session
            .say("Anyone there?")
            .await
            .expect("the message is taken");
        let (frames, _) = read(&session, Version::ZERO);
        assert_eq!(frames[0]["entries"][0]["text"], json!("Anyone there?"));
    }

    #[tokio::test]
    async fn a_reader_waits_until_something_changes() {
        let (session, _events) = session();
        let cursor = session.wait_for_changes(Version::ZERO).await.version();

        tokio::select! {
            biased;
            _ = session.wait_for_changes(cursor) => panic!("nothing changed yet"),
            () = tokio::task::yield_now() => {}
        }

        let writer = session.clone();
        let (changes, ()) = tokio::join!(session.wait_for_changes(cursor), async move {
            writer.notice("**Milestone 2 unlocked**");
        });
        assert_eq!(changes.frames().len(), 1);
        assert!(changes.version() > cursor);
    }

    #[test]
    fn a_session_prints_without_its_contents() {
        let (session, _events) = session();
        let printed = format!("{session:?}");
        assert!(printed.starts_with("Session { version: Version("));
    }
}
