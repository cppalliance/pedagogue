//! The host's side: page commands with their receipts and refusals, the
//! chat functions, and the stream of what the student did.

use std::error::Error;

use pedagogue_session::{ChatState, CommandErrorKind, StudentEvent, Version};
use serde_json::json;

use crate::common::{NOW_MS, apply_all, kinds, page, quiz, read, session};

#[test]
fn a_host_builds_a_page_and_reads_a_receipt_for_each_command() {
    let (session, _events) = session();

    let receipts = apply_all(
        &session,
        &[
            (
                "new_page",
                json!({ "chapter_number": 3, "chapter_title": "Chairing a WG21 Room",
                        "section_number": "3.2", "section_title": "Taking a Straw Poll" }),
            ),
            (
                "exposition",
                json!({ "markdown": "The chair states the question clearly." }),
            ),
            (
                "tip",
                json!({ "title": "Chair's Tip", "markdown": "Keep your eye on the timebox." }),
            ),
            (
                "bar_chart",
                json!({ "caption": "A poll with consensus in favor", "bars": [
                    { "label": "SF", "value": 10 },
                    { "label": "SA", "value": 1, "tone": "negative" },
                ] }),
            ),
            (
                "footnote",
                json!({ "markdown": "Straw polls are non-binding." }),
            ),
            ("continue_button", json!({ "label": "Continue to 3.3" })),
        ],
    );

    assert_eq!(
        receipts,
        [
            "Started page 1.",
            "Added exposition b1 to page 1.",
            "Added tip b2 to page 1.",
            "Added bar chart b3 to page 1.",
            "Added footnote 1 to page 1.",
            "Set the Continue button of page 1 to \"Continue to 3.3\".",
        ],
    );
    let (frames, _) = read(&session, Version::ZERO);
    assert_eq!(kinds(&frames), ["page 1", "chat"]);
    assert_eq!(
        frames[0],
        json!({
            "type": "page",
            "page_count": 1,
            "page": {
                "number": 1,
                "chapter_number": 3,
                "chapter_title": "Chairing a WG21 Room",
                "section_number": "3.2",
                "section_title": "Taking a Straw Poll",
                "chapter_opener": true,
                "blocks": [
                    { "id": "b1", "kind": "exposition",
                      "markdown": "The chair states the question clearly." },
                    { "id": "b2", "kind": "callout", "variant": "tip", "title": "Chair's Tip",
                      "markdown": "Keep your eye on the timebox." },
                    { "id": "b3", "kind": "bar_chart", "figure": "3.1",
                      "caption": "A poll with consensus in favor", "bars": [
                        { "label": "SF", "value": 10.0, "tone": "accent" },
                        { "label": "SA", "value": 1.0, "tone": "negative" },
                      ] },
                ],
                "footnotes": ["Straw polls are non-binding."],
                "continue_label": "Continue to 3.3",
                "continue_pressed": false,
            },
        }),
    );
}

#[test]
fn a_refused_command_says_what_to_fix_and_leaves_the_session_as_it_was() {
    let (session, _events) = session();
    apply_all(&session, &[("new_page", json!({}))]);
    let (_, before) = read(&session, Version::ZERO);

    let unknown = session
        .apply("paragraph", &json!({}))
        .expect_err("no command has that name");
    assert_eq!(unknown.kind(), CommandErrorKind::UnknownTool);
    assert_eq!(unknown.to_string(), "unknown tool `paragraph`");
    assert!(unknown.refusal().starts_with("Unknown tool \"paragraph\"."));

    let misshapen = session
        .apply(
            "milestone",
            &json!({ "number": "two", "markdown": "Finish." }),
        )
        .expect_err("the number is text");
    assert_eq!(misshapen.kind(), CommandErrorKind::InvalidArguments);
    assert_eq!(misshapen.to_string(), "invalid arguments for `milestone`");
    assert_eq!(
        misshapen.refusal(),
        "Invalid arguments for milestone, in \"number\": invalid type: string \"two\", expected \
         u32. Fix that field and call milestone again.",
    );
    let cause = misshapen.source().expect("the parser's error is the cause");
    assert_eq!(
        cause.to_string(),
        "invalid type: string \"two\", expected u32"
    );

    let flawed = session
        .apply("exposition", &json!({ "markdown": " " }))
        .expect_err("the text is empty");
    assert_eq!(flawed.kind(), CommandErrorKind::InvalidArguments);
    assert_eq!(
        flawed.to_string(),
        "the arguments of `exposition` break a rule"
    );
    let cause = flawed.source().expect("the broken rule is the cause");
    assert_eq!(cause.to_string(), "`markdown` is empty");

    let early = session
        .apply(
            "quiz_result",
            &json!({ "quiz": "b1", "verdict": "correct", "feedback": "Exactly." }),
        )
        .expect_err("no quiz has that id");
    assert_eq!(early.kind(), CommandErrorKind::Refused);
    assert_eq!(
        early.refusal(),
        "No quiz has the id \"b1\". Use the id from the receipt that added the quiz.",
    );

    assert_eq!(read(&session, Version::ZERO).1, before);
}

#[test]
fn the_chat_functions_fill_the_chat() {
    let (session, _events) = session();

    session.set_state(ChatState::Working);
    session.assistant_message("Read this section, then try the quiz.");
    session.notice("**Milestone 2 unlocked** - Section 2.2 is ready");
    session.set_suggestions(["Take the quiz", "Explain this page"]);
    session.set_state(ChatState::AwaitingInput);

    let (frames, _) = read(&session, Version::ZERO);
    assert_eq!(
        frames,
        [json!({
            "type": "chat",
            "state": "awaiting_input",
            "suggestions": ["Take the quiz", "Explain this page"],
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
async fn the_host_hears_every_attempt_and_marks_the_free_text_ones() {
    let (session, mut events) = session();
    apply_all(
        &session,
        &[
            ("new_page", json!({})),
            (
                "pop_quiz",
                json!({ "question": "What should the chair do first?", "options": [
                    { "text": "Restate the question", "correct": true,
                      "feedback": "The room has to know what it votes on." },
                    { "text": "Count the room", "correct": false,
                      "feedback": "Counting comes after the room knows the question." },
                ] }),
            ),
            (
                "short_answer",
                json!({ "question": "What does `std::move` do?" }),
            ),
        ],
    );

    session
        .answer_choice(quiz("b1"), &[1])
        .await
        .expect("the wrong pick is taken");
    session
        .answer_choice(quiz("b1"), &[0])
        .await
        .expect("the right pick is taken");
    session
        .answer_text(quiz("b2"), "It moves the object.")
        .await
        .expect("the quiz is open");

    let mut heard = Vec::new();
    for _ in 0..3 {
        let event = events.recv().await.expect("an event for each attempt");
        heard.push(event.to_model_text());
    }
    assert_eq!(
        heard,
        [
            "Quiz b1, attempt 1: chose B \"Count the room\". Incorrect.",
            "Quiz b1, attempt 2: chose A \"Restate the question\". Correct.",
            "Quiz b2 (short answer), attempt 1: the student wrote \"It moves the object.\"",
        ],
    );

    // The host marks the free-text answer, and the student tries again.
    let receipt = session
        .apply(
            "quiz_result",
            &json!({ "quiz": "b2", "verdict": "incorrect", "feedback": "Nothing has moved yet." }),
        )
        .expect("an answer waits for its verdict");
    assert_eq!(
        receipt,
        "Marked quiz b2 incorrect. The student can try again."
    );
    session
        .answer_text(quiz("b2"), "It casts to an rvalue.")
        .await
        .expect("a wrong answer may be retried");
    let again = events.recv().await.expect("an event for the retry");
    assert_eq!(
        again.to_model_text(),
        "Quiz b2 (short answer), attempt 2: the student wrote \"It casts to an rvalue.\"",
    );

    let (frames, _) = read(&session, Version::ZERO);
    let blocks = &frames[0]["page"]["blocks"];
    assert_eq!(blocks[0]["status"], json!("solved"));
    assert_eq!(
        blocks[0]["options"],
        json!([
            { "text": "Restate the question", "state": "right", "checked": true },
            { "text": "Count the room", "state": "disabled", "checked": false },
        ]),
    );
    assert_eq!(blocks[1]["status"], json!("submitted"));
    assert_eq!(blocks[1]["answer"], json!("It casts to an rvalue."));
}

#[tokio::test]
async fn the_host_hears_a_message_a_press_of_continue_and_a_regenerate() {
    let (session, mut events) = session();
    apply_all(
        &session,
        &[
            ("new_page", json!({})),
            ("continue_button", json!({ "label": "Continue to 3.3" })),
        ],
    );
    session.assistant_message("Read this section.");

    session
        .say("Why five columns?")
        .await
        .expect("a message with text is taken");
    session
        .press_continue(page(1))
        .await
        .expect("the button is ready");
    session.regenerate().await.expect("the tutor has replied");

    assert_eq!(
        events.recv().await,
        Some(StudentEvent::UserMessage {
            text: "Why five columns?".to_owned()
        }),
    );
    assert_eq!(
        events.recv().await,
        Some(StudentEvent::ContinuePressed {
            page: page(1),
            label: "Continue to 3.3".to_owned()
        }),
    );
    assert_eq!(events.recv().await, Some(StudentEvent::RegeneratePressed));
}

#[tokio::test]
async fn a_refused_action_names_its_reason_and_reaches_no_host() {
    let (session, mut events) = session();
    apply_all(
        &session,
        &[
            ("new_page", json!({})),
            (
                "pop_quiz",
                json!({ "question": "Which?", "options": [
                    { "text": "This", "correct": true },
                    { "text": "That", "correct": false },
                ] }),
            ),
        ],
    );
    session
        .answer_choice(quiz("b1"), &[1])
        .await
        .expect("the wrong pick is taken");
    events.recv().await.expect("an event for the attempt");
    let (_, before) = read(&session, Version::ZERO);

    let again = session
        .answer_choice(quiz("b1"), &[1])
        .await
        .expect_err("the wrong pick is disabled");
    assert_eq!(again.to_string(), "the answer to quiz `b1` breaks a rule");
    let cause = again.source().expect("the broken rule is the cause");
    assert_eq!(cause.to_string(), "option 1 is disabled");

    let text = session
        .answer_text(quiz("b1"), "This.")
        .await
        .expect_err("a pop quiz takes picks");
    assert_eq!(text.to_string(), "quiz `b1` takes picks");

    assert_eq!(read(&session, Version::ZERO).1, before);
    drop(session);
    assert_eq!(events.recv().await, None);
}
