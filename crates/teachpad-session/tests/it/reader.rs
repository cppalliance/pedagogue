//! The reader's side: a cursor over whole states, and the wait for the next
//! change.

use serde_json::{Value, json};
use teachpad_session::Version;

use crate::common::{apply_all, kinds, quiz, read, session};

#[test]
fn two_readers_each_get_what_they_have_yet_to_see() {
    let (session, _events) = session();
    apply_all(&session, &[("new_page", json!({}))]);
    let (first_read, early) = read(&session, Version::ZERO);
    assert_eq!(kinds(&first_read), ["page 1", "chat"]);

    apply_all(
        &session,
        &[
            ("exposition", json!({ "markdown": "One." })),
            ("new_page", json!({})),
            ("exposition", json!({ "markdown": "Two." })),
        ],
    );
    let (_, middle) = read(&session, early);
    session.assistant_message("Read both pages.");

    // The reader that fell behind gets both pages and the chat. The one
    // that kept up gets the chat alone.
    let (behind, newest) = read(&session, early);
    assert_eq!(kinds(&behind), ["page 1", "page 2", "chat"]);
    assert_eq!(behind[0]["page_count"], json!(2));
    let (kept_up, same) = read(&session, middle);
    assert_eq!(kinds(&kept_up), ["chat"]);
    assert_eq!(same, newest);

    assert!(read(&session, newest).0.is_empty());
}

#[tokio::test]
async fn an_answer_on_an_older_page_reaches_the_reader_as_that_page_alone() {
    let (session, _events) = session();
    apply_all(
        &session,
        &[
            ("new_page", json!({})),
            ("short_answer", json!({ "question": "Why?" })),
            ("new_page", json!({})),
        ],
    );
    let (_, cursor) = read(&session, Version::ZERO);

    session
        .answer_text(quiz("b1"), "Because.")
        .await
        .expect("the quiz is open");

    let (frames, _) = read(&session, cursor);
    assert_eq!(kinds(&frames), ["page 1"]);
    assert_eq!(frames[0]["page_count"], json!(2));
    assert_eq!(frames[0]["page"]["blocks"][0]["status"], json!("submitted"));
}

#[tokio::test]
async fn a_waiting_reader_wakes_when_another_handle_changes_the_session() {
    let (session, _events) = session();
    let cursor = session.wait_for_changes(Version::ZERO).await.version();

    let writer = session.clone();
    let (changes, ()) = tokio::join!(session.wait_for_changes(cursor), async move {
        apply_all(&writer, &[("new_page", json!({}))]);
    });

    assert!(changes.version() > cursor);
    let frames: Vec<Value> = changes
        .frames()
        .iter()
        .map(|frame| serde_json::to_value(frame).expect("a frame serializes"))
        .collect();
    assert_eq!(kinds(&frames), ["page 1"]);
}
