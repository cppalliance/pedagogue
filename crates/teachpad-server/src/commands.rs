//! The command route: one page command per request, for poking a running
//! app by hand.

use axum::extract::State;
use axum::extract::rejection::JsonRejection;
use axum::routing::post;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::app::AppState;
use crate::error::Refusal;

/// One tool call, in the shape `Session::apply` takes.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Call {
    tool: String,
    /// The arguments of the command. A command that takes none may leave
    /// them out.
    #[serde(default)]
    args: Map<String, Value>,
}

/// The answer to a command that applied.
#[derive(Debug, Serialize)]
struct Applied {
    receipt: String,
}

/// The command route: `POST /commands` with `{ "tool", "args" }`.
pub(crate) fn routes() -> Router<AppState> {
    Router::new().route("/commands", post(apply))
}

/// Applies one page command and answers with its receipt, or with the
/// refusal a model would read.
async fn apply(
    State(state): State<AppState>,
    call: Result<Json<Call>, JsonRejection>,
) -> Result<Json<Applied>, Refusal> {
    let Json(call) = call.map_err(|rejection| Refusal::NotACall(rejection.body_text()))?;
    let receipt = state
        .session
        .apply(&call.tool, &Value::Object(call.args))
        .map_err(Refusal::Command)?;
    Ok(Json(Applied { receipt }))
}

#[cfg(test)]
mod tests {
    use super::*;

    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use serde_json::json;
    use teachpad_session::{Session, Version};
    use tower::ServiceExt;

    use crate::app::testing::{body_json, router_over, session};

    /// Posts a JSON body to the command route of a router over `session`.
    async fn post(session: &Session, body: &Value) -> (StatusCode, Value) {
        let request = Request::builder()
            .method("POST")
            .uri("/commands")
            .header("content-type", "application/json")
            .body(Body::from(body.to_string()))
            .expect("static request parts are valid");
        let response = router_over(session.clone())
            .oneshot(request)
            .await
            .expect("the router is infallible");
        (response.status(), body_json(response).await)
    }

    #[tokio::test]
    async fn a_command_that_applies_answers_with_its_receipt() {
        let session = session();

        let (status, body) = post(&session, &json!({ "tool": "new_page" })).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body, json!({ "receipt": "Started page 1." }));

        let call = json!({ "tool": "exposition", "args": { "markdown": "The chair states the question." } });
        let (status, body) = post(&session, &call).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body, json!({ "receipt": "Added exposition b1 to page 1." }));

        // The command landed in the session that the sockets read.
        let changes = session.changes_since(Version::ZERO);
        let wire = serde_json::to_value(changes.frames()).expect("frames serialize");
        assert_eq!(wire[0]["page"]["blocks"][0]["id"], "b1");
    }

    #[tokio::test]
    async fn a_refused_command_answers_with_the_sentence_a_model_reads() {
        let session = session();
        let cases = [
            (
                json!({ "tool": "exposition", "args": { "markdown": "Text." } }),
                StatusCode::CONFLICT,
                "refused",
                "No page yet. Call new_page first.",
            ),
            (
                json!({ "tool": "new_page", "args": { "colour": "red" } }),
                StatusCode::UNPROCESSABLE_ENTITY,
                "invalid_arguments",
                "Invalid arguments for new_page: unknown field `colour`, expected one of \
                 `chapter_number`, `chapter_title`, `section_number`, `section_title`. Fix them \
                 and call new_page again.",
            ),
        ];
        for (call, status, code, message) in cases {
            let (answered, body) = post(&session, &call).await;
            assert_eq!(answered, status);
            assert_eq!(
                body,
                json!({ "error": { "code": code, "message": message } })
            );
        }

        let (status, body) = post(&session, &json!({ "tool": "paragraph" })).await;
        assert_eq!(status, StatusCode::NOT_FOUND);
        assert_eq!(body["error"]["code"], "unknown_tool");
    }

    #[tokio::test]
    async fn a_body_that_is_no_tool_call_is_refused() {
        let session = session();
        for body in [
            json!({ "args": {} }),
            json!({ "tool": "new_page", "extra": true }),
            json!({ "tool": "new_page", "args": "none" }),
            json!("new_page"),
        ] {
            let (status, answer) = post(&session, &body).await;
            assert_eq!(status, StatusCode::BAD_REQUEST, "for {body}");
            assert_eq!(answer["error"]["code"], "not_a_call", "for {body}");
        }
    }
}
