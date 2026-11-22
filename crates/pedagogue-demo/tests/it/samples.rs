//! The sample scripts under `samples/`, played into a session.

use pedagogue_demo::{Pace, play};
use pedagogue_session::Version;
use serde_json::{Value, json};

use crate::common::{frames, sample, sample_path, session};

/// Set to `1` to rewrite the frames file from the sample.
const UPDATE_SWITCH: &str = "PEDAGOGUE_UPDATE_FIXTURES";

#[tokio::test]
async fn the_frames_file_holds_what_the_contact_sheet_sample_sends() {
    let (session, mut events) = session();
    play(
        &session,
        &mut events,
        &sample("contact-sheet.json"),
        Pace::Instant,
    )
    .await
    .expect("every step of the sample applies");
    let changes = session.changes_since(Version::ZERO);
    let path = sample_path("contact-sheet.frames.json");

    if std::env::var_os(UPDATE_SWITCH).is_some_and(|value| value == "1") {
        let mut text = serde_json::to_string_pretty(changes.frames()).expect("frames serialize");
        text.push('\n');
        std::fs::write(&path, text).expect("the frames file is written");
    }

    let on_disk = std::fs::read_to_string(&path).expect("the frames file is on disk");
    let on_disk: Value = serde_json::from_str(&on_disk).expect("the frames file is JSON");
    let sent = serde_json::to_value(changes.frames()).expect("frames serialize");
    assert!(
        on_disk == sent,
        "samples/contact-sheet.frames.json is stale. Run this test again with \
         {UPDATE_SWITCH}=1 to rewrite it, and review the diff.",
    );
}

#[tokio::test]
async fn the_contact_sheet_sample_reaches_every_quiz_state_on_the_sheet() {
    let (session, mut events) = session();
    play(
        &session,
        &mut events,
        &sample("contact-sheet.json"),
        Pace::Instant,
    )
    .await
    .expect("every step of the sample applies");

    let frames = frames(&session);
    assert_eq!(frames.len(), 2, "one page, then the chat");
    let blocks = frames[0]["page"]["blocks"]
        .as_array()
        .expect("blocks are a list");
    let of_kind = |kind: &str| -> Vec<&Value> {
        blocks
            .iter()
            .filter(|block| block["kind"] == json!(kind))
            .collect()
    };

    // Each choice quiz: its status, the state of each option, and whether a
    // right option is still unpicked.
    let choice: Vec<String> = of_kind("choice_quiz")
        .iter()
        .map(|quiz| {
            let options: Vec<&str> = quiz["options"]
                .as_array()
                .expect("options are a list")
                .iter()
                .map(|option| option["state"].as_str().unwrap_or("?"))
                .collect();
            let missing = if quiz["missing"] == json!(true) {
                " missing"
            } else {
                ""
            };
            format!(
                "{} [{}]{missing}",
                quiz["status"].as_str().unwrap_or("?"),
                options.join(" ")
            )
        })
        .collect();
    assert_eq!(
        choice,
        [
            "open [open open open open]",
            "solved [right open open]",
            "open [disabled open open]",
            "open [open open open open]",
            "open [open open open open]",
            "open [open open disabled open] missing",
            "solved [right right disabled right]",
        ],
    );

    let response: Vec<String> = of_kind("response_quiz")
        .iter()
        .map(|quiz| {
            format!(
                "{} {}",
                quiz["variant"].as_str().unwrap_or("?"),
                quiz["status"].as_str().unwrap_or("?")
            )
        })
        .collect();
    assert_eq!(
        response,
        [
            "short_essay open",
            "short_answer open",
            "short_answer submitted",
            "short_answer right",
            "short_answer wrong",
            "try_it open",
        ],
    );

    // The chat is the live panel of the chat design, with no stand-in lines.
    let entries: Vec<&str> = frames[1]["entries"]
        .as_array()
        .expect("entries are a list")
        .iter()
        .map(|entry| entry["kind"].as_str().unwrap_or("?"))
        .collect();
    assert_eq!(entries, ["assistant", "user", "assistant"]);
    assert_eq!(
        frames[1]["suggestions"],
        json!(["Take the quiz", "Explain this page", "Show an example"])
    );
}

#[tokio::test]
async fn the_straw_poll_sample_adds_its_second_page_after_a_press_of_continue() {
    let (session, mut events) = session();
    let script = sample("straw-poll.json");
    let page_one = serde_json::from_value(json!(1)).expect("1 is a page number");

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
        let pages_before = frames(&session).len() - 1;
        session
            .press_continue(page_one)
            .await
            .expect("the button is ready");
        pages_before
    };
    let (played, pages_before) =
        tokio::join!(play(&session, &mut events, &script, Pace::Instant), student);

    played.expect("every step of the sample applies");
    assert_eq!(pages_before, 1, "the script waits on page 1");
    let frames = frames(&session);
    assert_eq!(frames.len(), 3, "two pages, then the chat");
    assert_eq!(frames[0]["page"]["continue_pressed"], json!(true));
    assert_eq!(
        frames[1]["page"]["section_title"],
        json!("Reading the Result")
    );
    assert_eq!(frames[1]["page"]["chapter_opener"], json!(false));
}
