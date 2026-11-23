//! The script player, and the one entry point that plays a script and then
//! answers the live student.

use std::borrow::Cow;
use std::collections::BTreeMap;
use std::time::Duration;

use serde_json::{Value, json};
use teachpad_session::{BlockId, Session, StudentEvent, StudentEvents};

use crate::error::{Problem, ScriptError};
use crate::pace::Pace;
use crate::script::{Answer, QuizRef, Script, Step, Wait, named_quiz};
use crate::stand_in::StandIn;

/// Plays a script into a session, then answers the live student for as long
/// as the caller keeps this future.
///
/// This is the demo's one entry point. It spawns nothing, so the caller
/// picks the runtime, and it holds nothing that needs a clean close, so
/// dropping the future is how it stops. It returns on its own only when the
/// event stream ends.
///
/// # Errors
///
/// Returns a [`ScriptError`] that names the step when the session refuses
/// a step of the script. The steps before it stay applied.
///
/// # Examples
///
/// ```no_run
/// # async fn example() -> Result<(), teachpad_demo::ScriptError> {
/// use teachpad_demo::{Pace, STRAW_POLL, Script, run};
/// use teachpad_session::Session;
///
/// let script = Script::parse(STRAW_POLL)?;
/// let (session, mut events) = Session::new();
/// // A server would get a clone of `session` here.
/// run(&session, &mut events, &script, Pace::Live).await
/// # }
/// ```
pub async fn run(
    session: &Session,
    events: &mut StudentEvents,
    script: &Script,
    pace: Pace,
) -> Result<(), ScriptError> {
    play(session, events, script, pace).await?;
    let stand_in = StandIn::new(session, pace);
    while let Some(event) = events.recv().await {
        stand_in.answer(&event).await?;
    }
    Ok(())
}

/// Plays a script into a session and returns when its last step is done.
///
/// The player takes the events its own student steps cause, so the chat
/// shows exactly what the script wrote. Whatever a live student does in the
/// meantime goes to the stand-in, and a live press of Continue is what ends
/// a `wait` step.
///
/// # Errors
///
/// Returns a [`ScriptError`] that names the step when the session refuses
/// a step of the script. The steps before it stay applied.
///
/// # Examples
///
/// ```
/// # #[tokio::main(flavor = "current_thread")]
/// # async fn main() -> Result<(), teachpad_demo::ScriptError> {
/// use teachpad_demo::{Pace, Script, play};
/// use teachpad_session::{Session, Version};
///
/// let script = Script::parse(
///     r#"[
///         { "tool": "new_page", "args": { "section_title": "Taking a Straw Poll" } },
///         { "assistant": "Read this section, then try the quiz." }
///     ]"#,
/// )?;
/// let (session, mut events) = Session::new();
/// play(&session, &mut events, &script, Pace::Instant).await?;
/// // The page, then the chat.
/// assert_eq!(session.changes_since(Version::ZERO).frames().len(), 2);
/// # Ok(())
/// # }
/// ```
pub async fn play(
    session: &Session,
    events: &mut StudentEvents,
    script: &Script,
    pace: Pace,
) -> Result<(), ScriptError> {
    let stand_in = StandIn::new(session, pace);
    let mut names: BTreeMap<&str, BlockId> = BTreeMap::new();
    for (index, step) in script.steps.iter().enumerate() {
        let number = index + 1;
        if index > 0 {
            rest(events, &stand_in, pace.step_delay()).await?;
        }
        match step {
            Step::Command { tool, args, name } => {
                let receipt = session
                    .apply(tool, &with_ids(args, &names))
                    .map_err(|source| Problem::Refused {
                        step: number,
                        tool: tool.clone(),
                        source,
                    })?;
                if let Some(name) = name {
                    let id = block_in(&receipt).ok_or_else(|| Problem::NoBlock {
                        step: number,
                        name: name.clone(),
                        tool: tool.clone(),
                    })?;
                    names.insert(name, id);
                }
            }
            Step::Assistant(text) => session.assistant_message(text),
            Step::Notice(text) => session.notice(text),
            Step::Suggestions(chips) => {
                session.set_suggestions(chips.iter().map(String::as_str));
            }
            Step::Student(text) => {
                session
                    .say(text)
                    .await
                    .map_err(|source| Problem::ActionRefused {
                        step: number,
                        source,
                    })?;
                let is_own =
                    |event: &StudentEvent| matches!(event, StudentEvent::UserMessage { .. });
                take_own(events, &stand_in, is_own).await?;
            }
            Step::Answer { quiz, answer } => {
                let id = match quiz {
                    QuizRef::Id(id) => *id,
                    QuizRef::Name(name) => {
                        *names
                            .get(name.as_str())
                            .ok_or_else(|| Problem::UnknownName {
                                step: number,
                                name: name.clone(),
                            })?
                    }
                };
                let sent = match answer {
                    Answer::Picks(picks) => session.answer_choice(id, picks).await,
                    Answer::Text(text) => session.answer_text(id, text).await,
                };
                sent.map_err(|source| Problem::ActionRefused {
                    step: number,
                    source,
                })?;
                let is_own = |event: &StudentEvent| matches!(event, StudentEvent::QuizAnswered { quiz, .. } if *quiz == id);
                take_own(events, &stand_in, is_own).await?;
            }
            Step::Wait(Wait::Continue) => {
                if !until_continue(events, &stand_in).await? {
                    return Ok(());
                }
            }
        }
    }
    Ok(())
}

/// Writes the id of a named quiz into the `quiz` argument of a command.
fn with_ids<'a>(args: &'a Value, names: &BTreeMap<&str, BlockId>) -> Cow<'a, Value> {
    let Some(id) = named_quiz(args).and_then(|name| names.get(name)) else {
        return Cow::Borrowed(args);
    };
    let mut filled = args.clone();
    if let Some(quiz) = filled.get_mut("quiz") {
        *quiz = json!(id);
    }
    Cow::Owned(filled)
}

/// Finds the id of the block a receipt names, such as `b7` in
/// `Added pop quiz b7 to page 3.` A host reads ids the same way.
fn block_in(receipt: &str) -> Option<BlockId> {
    receipt
        .split_whitespace()
        .find_map(|word| serde_json::from_value(Value::String(word.to_owned())).ok())
}

/// Rests between two steps. A live event that arrives meanwhile goes to the
/// stand-in.
async fn rest(
    events: &mut StudentEvents,
    stand_in: &StandIn<'_>,
    delay: Duration,
) -> Result<(), Problem> {
    if delay.is_zero() {
        return Ok(());
    }
    let wake = tokio::time::sleep(delay);
    tokio::pin!(wake);
    loop {
        tokio::select! {
            () = &mut wake => return Ok(()),
            event = events.recv() => match event {
                Some(event) => stand_in.answer(&event).await?,
                None => return Ok(()),
            },
        }
    }
}

/// Takes events off the stream up to the one a scripted step caused. A live
/// event that arrived before it goes to the stand-in.
async fn take_own(
    events: &mut StudentEvents,
    stand_in: &StandIn<'_>,
    is_own: impl Fn(&StudentEvent) -> bool,
) -> Result<(), Problem> {
    while let Some(event) = events.recv().await {
        if is_own(&event) {
            break;
        }
        stand_in.answer(&event).await?;
    }
    Ok(())
}

/// Holds the script until a live student presses Continue. Everything else
/// the student does meanwhile goes to the stand-in. Says `false` when the
/// event stream ended first.
async fn until_continue(
    events: &mut StudentEvents,
    stand_in: &StandIn<'_>,
) -> Result<bool, Problem> {
    while let Some(event) = events.recv().await {
        if matches!(event, StudentEvent::ContinuePressed { .. }) {
            return Ok(true);
        }
        stand_in.answer(&event).await?;
    }
    Ok(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::error::Error;
    use std::time::UNIX_EPOCH;

    use teachpad_session::Version;

    fn session() -> (Session, StudentEvents) {
        Session::with_clock(|| UNIX_EPOCH + Duration::from_secs(1_795_300_000))
    }

    fn script(steps: &Value) -> Script {
        Script::parse(&steps.to_string()).expect("the script parses")
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

    #[tokio::test]
    async fn a_script_writes_the_page_and_the_chat_in_step_order() {
        let (session, mut events) = session();
        let steps = script(&json!([
            { "tool": "new_page", "args": { "section_title": "Taking a Straw Poll" } },
            { "tool": "exposition", "args": { "markdown": "The chair states the question." } },
            { "assistant": "Read this section, then try the quiz." },
            { "student": "Why five columns?" },
            { "notice": "**Milestone 2 unlocked**" },
            { "suggestions": ["Take the quiz", "Explain this page"] },
        ]));

        play(&session, &mut events, &steps, Pace::Instant)
            .await
            .expect("every step applies");

        let frames = frames(&session);
        assert_eq!(
            frames[0]["page"]["section_title"],
            json!("Taking a Straw Poll")
        );
        assert_eq!(frames[0]["page"]["blocks"][0]["kind"], json!("exposition"));
        assert_eq!(
            frames[1]["suggestions"],
            json!(["Take the quiz", "Explain this page"])
        );
        // The script's own message shows as written, with no stand-in reply.
        assert_eq!(
            chat_lines(&session),
            [
                "assistant: Read this section, then try the quiz.",
                "user: Why five columns?",
                "notice: **Milestone 2 unlocked**",
            ],
        );
    }

    #[tokio::test]
    async fn a_name_given_with_as_stands_for_the_id_of_its_block() {
        let (session, mut events) = session();
        let steps = script(&json!([
            { "tool": "new_page" },
            { "tool": "exposition", "args": { "markdown": "Text." } },
            { "tool": "pop_quiz", "as": "poll", "args": { "question": "Which?", "options": [
                { "text": "This", "correct": true },
                { "text": "That", "correct": false },
            ] } },
            { "tool": "short_answer", "as": "why", "args": { "question": "Why?" } },
            { "answer": { "quiz": "@poll", "picks": [1] } },
            { "answer": { "quiz": "@why", "text": "Because." } },
            { "tool": "quiz_result",
              "args": { "quiz": "@why", "verdict": "correct", "feedback": "Exactly." } },
            { "answer": { "quiz": "b2", "picks": [0] } },
        ]));

        play(&session, &mut events, &steps, Pace::Instant)
            .await
            .expect("every step applies");

        let blocks = frames(&session)[0]["page"]["blocks"].clone();
        assert_eq!(blocks[1]["id"], json!("b2"));
        assert_eq!(blocks[1]["status"], json!("solved"));
        assert_eq!(blocks[1]["attempts"], json!(2));
        assert_eq!(blocks[2]["status"], json!("right"));
        assert_eq!(blocks[2]["feedback"], json!("Exactly."));
        // The player took the events of its own answers.
        assert!(chat_lines(&session).is_empty());
    }

    #[tokio::test]
    async fn a_refused_step_stops_the_script_and_names_the_step() {
        let (session, mut events) = session();
        let steps = script(&json!([
            { "tool": "new_page" },
            { "tool": "exposition", "args": { "markdown": "One." } },
            { "tool": "exposition", "args": { "markdown": "  " } },
            { "tool": "exposition", "args": { "markdown": "Three." } },
        ]));

        let error = play(&session, &mut events, &steps, Pace::Instant)
            .await
            .expect_err("the third step is refused");

        assert_eq!(error.to_string(), "step 3 (`exposition`) was refused");
        let cause = error.source().expect("the refusal is the cause");
        assert_eq!(
            cause.to_string(),
            "the arguments of `exposition` break a rule"
        );
        let blocks = frames(&session)[0]["page"]["blocks"].clone();
        assert_eq!(blocks.as_array().map(Vec::len), Some(1));
    }

    #[tokio::test]
    async fn a_refused_answer_and_a_name_for_no_block_name_their_steps() {
        let quiz = json!({ "question": "Which?", "options": [
            { "text": "This", "correct": true },
            { "text": "That", "correct": false },
        ] });
        let cases = [
            (
                json!([
                    { "tool": "new_page" },
                    { "tool": "pop_quiz", "as": "poll", "args": quiz },
                    { "answer": { "quiz": "@poll", "picks": [7] } },
                ]),
                "step 3 was refused",
            ),
            (
                json!([{ "tool": "new_page" }, { "student": "  " }]),
                "step 2 was refused",
            ),
            (
                json!([{ "tool": "new_page", "as": "page" }]),
                "step 1 names its block `page`, and `new_page` adds no block",
            ),
        ];
        for (steps, expected) in cases {
            let (session, mut events) = session();
            let error = play(&session, &mut events, &script(&steps), Pace::Instant)
                .await
                .expect_err("a step is refused");
            assert_eq!(error.to_string(), expected);
        }
    }

    #[tokio::test(start_paused = true)]
    async fn steps_play_a_quarter_second_apart_at_the_live_pace() {
        let (session, mut events) = session();
        let steps = script(&json!([
            { "tool": "new_page" },
            { "tool": "exposition", "args": { "markdown": "One." } },
            { "tool": "exposition", "args": { "markdown": "Two." } },
            { "assistant": "Read this section." },
            { "student": "Why five columns?" },
        ]));
        let started = tokio::time::Instant::now();

        play(&session, &mut events, &steps, Pace::Live)
            .await
            .expect("every step applies");

        assert_eq!(started.elapsed(), Duration::from_millis(1000));
    }

    #[tokio::test(start_paused = true)]
    async fn a_wait_holds_the_script_until_a_live_press_of_continue() {
        let (session, mut events) = session();
        let steps = script(&json!([
            { "tool": "new_page" },
            { "tool": "continue_button", "args": { "label": "Continue to 3.3" } },
            { "wait": "continue" },
            { "tool": "new_page" },
            { "tool": "exposition", "args": { "markdown": "Page two." } },
        ]));
        let page_one = serde_json::from_value(json!(1)).expect("1 is a page number");

        // The student writes a message while the script waits, then presses
        // Continue once the stand-in has replied.
        let student = async {
            let mut cursor = Version::ZERO;
            loop {
                let changes = session.wait_for_changes(cursor).await;
                cursor = changes.version();
                let wire = serde_json::to_value(changes.frames()).expect("frames serialize");
                let has_button = wire.as_array().is_some_and(|frames| {
                    frames
                        .iter()
                        .any(|frame| frame["page"]["continue_label"].is_string())
                });
                if has_button {
                    break;
                }
            }
            session
                .say("Is this the end?")
                .await
                .expect("the message is taken");
            while chat_lines(&session).len() < 2 {
                cursor = session.wait_for_changes(cursor).await.version();
            }
            let pages_before = frames(&session).len() - 1;
            session
                .press_continue(page_one)
                .await
                .expect("the button is ready");
            pages_before
        };
        let (played, pages_before) =
            tokio::join!(play(&session, &mut events, &steps, Pace::Live), student);

        played.expect("every step applies");
        assert_eq!(pages_before, 1);
        let frames = frames(&session);
        assert_eq!(frames.len(), 3);
        assert_eq!(frames[0]["page"]["continue_pressed"], json!(true));
        assert_eq!(
            frames[1]["page"]["blocks"][0]["markdown"],
            json!("Page two.")
        );
        // The press woke the script, so the chat holds only the live message
        // and its reply.
        assert_eq!(
            chat_lines(&session),
            [
                "user: Is this the end?",
                "assistant: I am a stand-in. A real tutor answers here once Runvil is connected.",
            ],
        );
    }

    #[tokio::test(start_paused = true)]
    async fn run_answers_the_live_student_after_the_script_ends() {
        let (session, mut events) = session();
        let steps = script(&json!([
            { "tool": "new_page" },
            { "tool": "continue_button", "args": { "label": "Continue to 3.3" } },
            { "assistant": "Read this section." },
        ]));
        let page_one = serde_json::from_value(json!(1)).expect("1 is a page number");

        // With the script done, a press of Continue shows as a notice.
        let student = async {
            let mut cursor = Version::ZERO;
            while chat_lines(&session).is_empty() {
                cursor = session.wait_for_changes(cursor).await.version();
            }
            session
                .press_continue(page_one)
                .await
                .expect("the button is ready");
            while chat_lines(&session).len() < 2 {
                cursor = session.wait_for_changes(cursor).await.version();
            }
        };
        tokio::select! {
            biased;
            ended = run(&session, &mut events, &steps, Pace::Live) => {
                panic!("run ended while its session lives: {ended:?}");
            }
            () = student => {}
        }

        assert_eq!(
            chat_lines(&session),
            [
                "assistant: Read this section.",
                "notice: **Sent to the tutor:** The student pressed \"Continue to 3.3\" on page 1.",
            ],
        );
    }
}
