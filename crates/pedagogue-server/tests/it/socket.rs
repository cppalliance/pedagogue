//! The `/ws` socket: frames down, student frames up.

use pedagogue_session::{QuizAnswer, StudentEvent};
use serde_json::json;
use tokio_tungstenite::tungstenite;

use crate::common::{Served, connect_from};

fn quiz() -> serde_json::Value {
    json!({ "question": "What should the chair do first?", "options": [
        { "text": "Restate the question", "correct": true },
        { "text": "Count the room", "correct": false, "feedback": "Counting comes later." },
    ] })
}

#[tokio::test]
async fn a_new_connection_gets_every_page_and_then_the_chat() {
    let served = Served::start();
    for (tool, args) in [
        (
            "new_page",
            json!({ "section_title": "Taking a Straw Poll" }),
        ),
        (
            "exposition",
            json!({ "markdown": "The chair states the question." }),
        ),
        ("new_page", json!({ "section_title": "Reading the Result" })),
    ] {
        served
            .session
            .apply(tool, &args)
            .expect("the command applies");
    }
    served.session.assistant_message("Read this section.");

    let mut client = served.connect().await;
    let first = client.next().await;
    let second = client.next().await;
    let chat = client.next().await;

    assert_eq!(first["type"], "page");
    assert_eq!(first["page_count"], 2);
    assert_eq!(first["page"]["number"], 1);
    assert_eq!(first["page"]["blocks"][0]["kind"], "exposition");
    assert_eq!(second["page"]["section_title"], "Reading the Result");
    assert_eq!(chat["type"], "chat");
    assert_eq!(chat["entries"][0]["text"], "Read this section.");
    served.stop().await;
}

#[tokio::test]
async fn a_command_arrives_as_a_page_frame() {
    let served = Served::start();
    let mut client = served.connect().await;
    assert_eq!(client.next().await["type"], "chat");

    served
        .session
        .apply("new_page", &json!({}))
        .expect("the command applies");
    let started = client.next().await;
    served
        .session
        .apply(
            "tip",
            &json!({ "markdown": "Keep your eye on the timebox." }),
        )
        .expect("the command applies");
    let grown = client.next().await;

    assert_eq!(started["type"], "page");
    assert_eq!(started["page"]["blocks"], json!([]));
    assert_eq!(grown["page"]["blocks"][0]["variant"], "tip");
    served.stop().await;
}

#[tokio::test]
async fn an_answer_comes_back_as_page_state_and_reaches_the_host() {
    let mut served = Served::start();
    served
        .session
        .apply("new_page", &json!({}))
        .expect("the command applies");
    served
        .session
        .apply("pop_quiz", &quiz())
        .expect("the command applies");
    let mut client = served.connect().await;
    client.next_of("chat").await;

    client
        .send(&json!({ "type": "answer", "quiz": "b1", "picks": [1] }))
        .await;
    let page = client.next().await;

    assert_eq!(page["type"], "page");
    let block = &page["page"]["blocks"][0];
    assert_eq!(block["options"][1]["state"], "disabled");
    assert_eq!(block["feedback"], json!(["Counting comes later."]));
    let event = served.next_event().await;
    assert!(matches!(
        event,
        StudentEvent::QuizAnswered {
            attempt: 1,
            answer: QuizAnswer::Picks { correct: false, .. },
            ..
        }
    ));
    served.stop().await;
}

#[tokio::test]
async fn two_clients_see_the_same_state() {
    let mut served = Served::start();
    let mut one = served.connect().await;
    let mut two = served.connect().await;
    one.next_of("chat").await;
    two.next_of("chat").await;

    one.send(&json!({ "type": "say", "text": "Why five columns?" }))
        .await;
    let seen_by_one = one.next().await;
    let seen_by_two = two.next().await;

    assert_eq!(seen_by_one["entries"][0]["kind"], "user");
    assert_eq!(seen_by_one["entries"][0]["text"], "Why five columns?");
    assert_eq!(seen_by_one, seen_by_two);
    assert_eq!(
        served.next_event().await,
        StudentEvent::UserMessage {
            text: "Why five columns?".to_owned()
        },
    );
    served.stop().await;
}

#[tokio::test]
async fn a_frame_the_server_cannot_use_is_skipped_and_the_connection_stays_up() {
    let mut served = Served::start();
    let mut client = served.connect().await;
    client.next_of("chat").await;

    client.send_text("this is not JSON").await;
    client
        .send(&json!({ "type": "shout", "text": "Hello." }))
        .await;
    client
        .send(&json!({ "type": "answer", "quiz": "b9", "picks": [0] }))
        .await;
    client
        .send(&json!({ "type": "say", "text": "Still here." }))
        .await;

    // The three frames before it changed nothing, so the next frame is the
    // chat with the one message.
    let chat = client.next().await;
    assert_eq!(chat["type"], "chat");
    assert_eq!(
        chat["entries"].as_array().map(Vec::len),
        Some(1),
        "only the last frame was a student action"
    );
    assert_eq!(chat["entries"][0]["text"], "Still here.");
    assert_eq!(
        served.next_event().await,
        StudentEvent::UserMessage {
            text: "Still here.".to_owned()
        },
    );
    served.stop().await;
}

#[tokio::test]
async fn an_upgrade_from_a_foreign_origin_is_refused() {
    let served = Served::start();

    connect_from(&served.socket_url(), &served.origin())
        .await
        .expect("a page of this server upgrades");
    let refused = connect_from(&served.socket_url(), "https://evil.example")
        .await
        .expect_err("a foreign page is refused");

    match refused {
        tungstenite::Error::Http(response) => {
            assert_eq!(response.status(), tungstenite::http::StatusCode::FORBIDDEN);
        }
        other => panic!("expected an HTTP 403, got {other:?}"),
    }
    served.stop().await;
}
