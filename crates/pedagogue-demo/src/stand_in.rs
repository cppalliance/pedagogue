//! The stand-in host: what the demo does with each thing a live student
//! does.

use pedagogue_session::{BlockId, ChatState, QuizAnswer, Session, StudentEvent};
use serde_json::json;

use crate::error::Problem;
use crate::pace::{Pace, pause};
use crate::wording;

/// Stands where the tutor will, and uses only what a real host could.
pub(crate) struct StandIn<'a> {
    session: &'a Session,
    pace: Pace,
}

impl<'a> StandIn<'a> {
    pub(crate) fn new(session: &'a Session, pace: Pace) -> Self {
        Self { session, pace }
    }

    /// Answers one thing a live student did.
    ///
    /// A message gets the typing dots and then the fixed reply. Everything
    /// else is shown in the chat as the text a tutor would be sent, and a
    /// free-text answer also gets a verdict, so its box moves on.
    pub(crate) async fn answer(&self, event: &StudentEvent) -> Result<(), Problem> {
        match event {
            StudentEvent::UserMessage { .. } => {
                self.session.set_state(ChatState::Working);
                pause(self.pace.reply_delay()).await;
                self.session.assistant_message(wording::REPLY);
                self.session.set_state(ChatState::AwaitingInput);
            }
            StudentEvent::QuizAnswered {
                quiz,
                attempt,
                answer,
            } => {
                self.show(event);
                if matches!(answer, QuizAnswer::Text { .. }) {
                    self.mark(*quiz, *attempt)?;
                }
            }
            StudentEvent::ContinuePressed { .. } | StudentEvent::RegeneratePressed => {
                self.show(event);
            }
        }
        Ok(())
    }

    /// Shows the text a tutor would be sent for an event.
    fn show(&self, event: &StudentEvent) {
        self.session
            .notice(&wording::sent_to_the_tutor(&event.to_model_text()));
    }

    /// Marks a free-text answer: the first one incorrect, so the student
    /// sees the box reopen, and every later one correct.
    fn mark(&self, quiz: BlockId, attempt: usize) -> Result<(), Problem> {
        let (verdict, feedback) = if attempt <= 1 {
            ("incorrect", wording::FIRST_TRY)
        } else {
            ("correct", wording::LATER_TRY)
        };
        let args = json!({ "quiz": quiz, "verdict": verdict, "feedback": feedback });
        self.session
            .apply("quiz_result", &args)
            .map_err(|source| Problem::Verdict {
                quiz: quiz.to_string(),
                source,
            })?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::time::{Duration, UNIX_EPOCH};

    use pedagogue_session::{StudentEvents, Version};
    use serde_json::Value;

    fn session() -> (Session, StudentEvents) {
        Session::with_clock(|| UNIX_EPOCH + Duration::from_secs(1_795_300_000))
    }

    /// Returns every frame of a session as JSON: its pages, then the chat.
    fn frames(session: &Session) -> Vec<Value> {
        session
            .changes_since(Version::ZERO)
            .frames()
            .iter()
            .map(|frame| serde_json::to_value(frame).expect("a frame serializes"))
            .collect()
    }

    /// Returns the texts of the chat entries, each with its kind.
    fn chat_lines(session: &Session) -> Vec<String> {
        let frames = frames(session);
        let chat = frames.last().expect("the chat is the last frame");
        chat["entries"]
            .as_array()
            .expect("entries are a list")
            .iter()
            .map(|entry| {
                format!(
                    "{}: {}",
                    entry["kind"].as_str().unwrap_or("?"),
                    entry["text"].as_str().unwrap_or("?")
                )
            })
            .collect()
    }

    /// Takes the next event a student action caused.
    async fn next(events: &mut StudentEvents) -> StudentEvent {
        events.recv().await.expect("the action sent an event")
    }

    #[tokio::test(start_paused = true)]
    async fn a_message_gets_the_typing_dots_and_then_the_fixed_reply() {
        let (session, mut events) = session();
        session
            .say("Why five columns?")
            .await
            .expect("the message is taken");
        let message = next(&mut events).await;
        let stand_in = StandIn::new(&session, Pace::Live);
        let cursor = session.changes_since(Version::ZERO).version();

        // A reader beside the stand-in notes each state of the chat.
        let reader = async {
            let mut seen = Vec::new();
            let mut cursor = cursor;
            while seen.last() != Some(&("awaiting_input".to_owned(), 2)) {
                let changes = session.wait_for_changes(cursor).await;
                cursor = changes.version();
                let chat = serde_json::to_value(changes.frames()).expect("frames serialize");
                let state = chat[0]["state"].as_str().unwrap_or("?").to_owned();
                let entries = chat[0]["entries"].as_array().map_or(0, Vec::len);
                seen.push((state, entries));
            }
            seen
        };
        let started = tokio::time::Instant::now();
        let (answered, seen) = tokio::join!(stand_in.answer(&message), reader);

        answered.expect("the stand-in replies");
        assert_eq!(
            seen,
            [("working".to_owned(), 1), ("awaiting_input".to_owned(), 2)]
        );
        assert_eq!(started.elapsed(), Pace::Live.reply_delay());
        assert_eq!(
            chat_lines(&session),
            [
                "user: Why five columns?",
                "assistant: I am a stand-in. A real tutor answers here once Runvil is connected.",
            ],
        );
    }

    #[tokio::test]
    async fn a_quiz_attempt_a_continue_and_a_regenerate_show_what_a_tutor_is_sent() {
        let (session, mut events) = session();
        let calls = [
            ("new_page", json!({})),
            (
                "pop_quiz",
                json!({ "question": "What should the chair do first?", "options": [
                    { "text": "Restate the question", "correct": true },
                    { "text": "Count the room", "correct": false },
                ] }),
            ),
            ("continue_button", json!({ "label": "Continue to 3.3" })),
        ];
        for (tool, args) in calls {
            session.apply(tool, &args).expect("the command applies");
        }
        session.assistant_message("Read this section.");
        let stand_in = StandIn::new(&session, Pace::Instant);
        let quiz = serde_json::from_value(json!("b1")).expect("b1 is a block id");
        let page = serde_json::from_value(json!(1)).expect("1 is a page number");

        session
            .answer_choice(quiz, &[1])
            .await
            .expect("the pick is taken");
        session
            .press_continue(page)
            .await
            .expect("the button is ready");
        session.regenerate().await.expect("the tutor has replied");
        for _ in 0..3 {
            let event = next(&mut events).await;
            stand_in.answer(&event).await.expect("the stand-in answers");
        }

        assert_eq!(
            chat_lines(&session)[1..],
            [
                "notice: **Sent to the tutor:** Quiz b1, attempt 1: chose B \"Count the room\". \
                 Incorrect.",
                "notice: **Sent to the tutor:** The student pressed \"Continue to 3.3\" on page 1.",
                "notice: **Sent to the tutor:** The student asked for the last reply again.",
            ],
        );
    }

    #[tokio::test]
    async fn a_free_text_answer_is_marked_wrong_once_and_right_after_that() {
        let (session, mut events) = session();
        session
            .apply("new_page", &json!({}))
            .expect("the command applies");
        session
            .apply("short_answer", &json!({ "question": "Why?" }))
            .expect("the command applies");
        let stand_in = StandIn::new(&session, Pace::Instant);
        let quiz = serde_json::from_value(json!("b1")).expect("b1 is a block id");
        let block = |session: &Session| frames(session)[0]["page"]["blocks"][0].clone();

        session
            .answer_text(quiz, "Because.")
            .await
            .expect("the quiz is open");
        let first = next(&mut events).await;
        stand_in.answer(&first).await.expect("the stand-in marks");
        assert_eq!(block(&session)["status"], json!("wrong"));
        assert_eq!(block(&session)["feedback"], json!(wording::FIRST_TRY));

        session
            .answer_text(quiz, "Because of the columns.")
            .await
            .expect("a wrong answer may be retried");
        let second = next(&mut events).await;
        stand_in.answer(&second).await.expect("the stand-in marks");
        assert_eq!(block(&session)["status"], json!("right"));
        assert_eq!(block(&session)["feedback"], json!(wording::LATER_TRY));
        assert_eq!(
            chat_lines(&session),
            [
                "notice: **Sent to the tutor:** Quiz b1 (short answer), attempt 1: the student \
                 wrote \"Because.\"",
                "notice: **Sent to the tutor:** Quiz b1 (short answer), attempt 2: the student \
                 wrote \"Because of the columns.\"",
            ],
        );
    }

    #[tokio::test]
    async fn a_verdict_the_session_refuses_is_an_error() {
        let (session, _events) = session();
        let stand_in = StandIn::new(&session, Pace::Instant);
        let stale = StudentEvent::QuizAnswered {
            quiz: serde_json::from_value(json!("b9")).expect("b9 is a block id"),
            attempt: 1,
            answer: QuizAnswer::Text {
                kind: pedagogue_session::ResponseKind::ShortAnswer,
                text: "Because.".to_owned(),
            },
        };

        let problem = stand_in
            .answer(&stale)
            .await
            .expect_err("the session holds no such quiz");

        assert_eq!(
            problem.to_string(),
            "the stand-in's verdict on quiz `b9` was refused"
        );
    }
}
