//! The frames the UI sends for the student, read from the fixture that the
//! UI tests read too.

use std::collections::BTreeMap;

use pedagogue_session::{StudentFrame, Version};
use serde_json::json;

use crate::common::{apply_all, read, session};

const STUDENT_FRAMES: &str = include_str!("../../fixtures/student-frames.json");

#[tokio::test]
async fn a_session_takes_every_frame_in_the_fixture() {
    let mut examples: BTreeMap<String, StudentFrame> =
        serde_json::from_str(STUDENT_FRAMES).expect("every example parses");

    // A lesson that fits the fixture: a pop quiz `b1`, a select-all quiz
    // `b2`, a short-answer quiz `b3`, a Continue button and one reply.
    let (session, mut events) = session();
    apply_all(
        &session,
        &[
            ("new_page", json!({})),
            (
                "pop_quiz",
                json!({ "question": "What should the chair do first?", "options": [
                    { "text": "Restate the question", "correct": true },
                    { "text": "Count the room", "correct": false },
                ] }),
            ),
            (
                "pop_quiz",
                json!({ "question": "Which columns count in favor?", "multiple": true,
                        "options": [
                    { "text": "Strongly favor", "correct": true },
                    { "text": "Neutral", "correct": false },
                    { "text": "Weakly favor", "correct": true },
                ] }),
            ),
            (
                "short_answer",
                json!({ "question": "What does `std::move` do?" }),
            ),
            ("continue_button", json!({ "label": "Continue to 3.3" })),
        ],
    );
    session.assistant_message("Read this section, then try the quiz.");

    let order = [
        "say",
        "answer_picks",
        "answer_select",
        "answer_text",
        "continue",
        "regenerate",
    ];
    let mut heard = Vec::new();
    for name in order {
        let frame = examples
            .remove(name)
            .unwrap_or_else(|| panic!("the fixture holds `{name}`"));
        session
            .submit(frame)
            .await
            .unwrap_or_else(|error| panic!("`{name}` is taken: {error}"));
        let event = events.recv().await.expect("an event for each frame");
        heard.push(event.to_model_text());
    }
    assert!(
        examples.is_empty(),
        "the fixture holds examples this test leaves out: {:?}",
        examples.keys()
    );

    assert_eq!(
        heard,
        [
            "Why five columns?",
            "Quiz b1, attempt 1: chose B \"Count the room\". Incorrect.",
            "Quiz b2, attempt 1: chose A \"Strongly favor\" and C \"Weakly favor\". Correct.",
            "Quiz b3 (short answer), attempt 1: the student wrote \"It casts to an rvalue.\"",
            "The student pressed \"Continue to 3.3\" on page 1.",
            "The student asked for the last reply again.",
        ],
    );
    let (frames, _) = read(&session, Version::ZERO);
    let page = &frames[0]["page"];
    assert_eq!(page["blocks"][0]["options"][1]["state"], json!("disabled"));
    assert_eq!(page["blocks"][1]["status"], json!("solved"));
    assert_eq!(page["blocks"][2]["status"], json!("submitted"));
    assert_eq!(page["continue_pressed"], json!(true));
    assert_eq!(
        frames[1]["entries"][1],
        json!({ "kind": "user", "text": "Why five columns?", "at": crate::common::NOW_MS }),
    );
}
