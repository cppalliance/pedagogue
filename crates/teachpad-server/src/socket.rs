//! The `/ws` socket: page and chat frames go down, student frames come up.
//!
//! One task owns each socket for reading and writing. It keeps a cursor
//! into the session and sends whatever changed since, which makes the
//! first send, a change and a catch-up after a slow stretch the same code
//! path. A missed wake-up costs nothing, since the next one sends
//! everything since the cursor, and a slow client slows only its own
//! task.

use axum::Router;
use axum::extract::State;
use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::http::HeaderMap;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use teachpad_session::{Session, StudentFrame, Version};
use tokio::sync::{mpsc, watch};

use crate::app::AppState;
use crate::cross_site::origin_allowed;
use crate::error::Refusal;

/// The most bytes one inbound message may hold. A student frame is a line
/// of JSON, and an essay runs to a few thousand characters.
const MAX_INBOUND: usize = 64 * 1024;

/// The socket route.
pub(crate) fn routes() -> Router<AppState> {
    Router::new().route("/ws", get(upgrade))
}

/// Upgrades `GET /ws` to a socket. An upgrade from a foreign origin is
/// refused, since older browsers leave the cross-site mark off an upgrade.
async fn upgrade(
    State(state): State<AppState>,
    headers: HeaderMap,
    upgrade: WebSocketUpgrade,
) -> Response {
    if !origin_allowed(&headers) {
        return Refusal::CrossSite.into_response();
    }
    upgrade
        .max_message_size(MAX_INBOUND)
        .max_frame_size(MAX_INBOUND)
        .on_upgrade(move |socket| connection(socket, state.session, state.stopping, state.open))
}

/// Why a connection ended.
#[derive(Debug, Clone, Copy)]
enum Ending {
    /// The client closed the socket, or the socket failed.
    Closed,
    /// The server is stopping.
    Stopping,
}

/// Runs one connection until the socket closes or the server stops.
///
/// `_open` is held for as long as the task lives: the server waits for
/// every holder to finish before it reports that it has stopped.
async fn connection(
    mut socket: WebSocket,
    session: Session,
    mut stopping: watch::Receiver<bool>,
    _open: mpsc::Sender<()>,
) {
    // At the level a binary shows by default, because the release check of
    // each installer waits for this line: it appears once the page in the
    // app's window has connected, which shows that the window works.
    tracing::info!("socket opened");
    let mut cursor = Version::ZERO;
    let ending = loop {
        tokio::select! {
            biased;
            () = stopped(&mut stopping) => break Ending::Stopping,
            changes = session.wait_for_changes(cursor) => {
                cursor = changes.version();
                if !send_all(&mut socket, changes.frames()).await {
                    break Ending::Closed;
                }
            }
            inbound = socket.recv() => match inbound {
                Some(Ok(Message::Text(text))) => {
                    if !take(&session, &mut stopping, text.as_str()).await {
                        break Ending::Stopping;
                    }
                }
                Some(Ok(Message::Close(_)) | Err(_)) | None => break Ending::Closed,
                // Pings are answered by the socket itself, and the UI
                // sends no binary frames.
                Some(Ok(_)) => {}
            },
        }
    };
    if matches!(ending, Ending::Stopping) {
        // The client may be gone already, and then there is no one to tell.
        let _ = socket.send(Message::Close(None)).await;
    }
    tracing::info!(?ending, "socket closed");
}

/// Sends frames in order. Says `false` when the socket no longer takes
/// them.
async fn send_all(socket: &mut WebSocket, frames: &[teachpad_session::Frame]) -> bool {
    for frame in frames {
        let text = match serde_json::to_string(frame) {
            Ok(text) => text,
            Err(error) => {
                tracing::error!(%error, "a frame could not be written as JSON");
                continue;
            }
        };
        if socket.send(Message::Text(text.into())).await.is_err() {
            return false;
        }
    }
    true
}

/// Passes one inbound frame to the session. A frame that cannot be read,
/// or an action the session refuses, is logged and skipped, and the
/// connection stays up. Says `false` when the server began to stop while
/// the action waited for room on the event stream.
async fn take(session: &Session, stopping: &mut watch::Receiver<bool>, text: &str) -> bool {
    let frame: StudentFrame = match serde_json::from_str(text) {
        Ok(frame) => frame,
        Err(error) => {
            tracing::warn!(%error, "an inbound frame could not be read and was skipped");
            return true;
        }
    };
    tokio::select! {
        biased;
        () = stopped(stopping) => false,
        outcome = session.submit(frame) => {
            if let Err(error) = outcome {
                tracing::warn!(%error, "a student action was refused");
            }
            true
        }
    }
}

/// Resolves when the server begins to stop. A closed signal channel means
/// the server is gone, which is the same reason to stop.
async fn stopped(stopping: &mut watch::Receiver<bool>) {
    // Both outcomes mean stop, so the value is not read.
    let _ = stopping.wait_for(|stop| *stop).await;
}
