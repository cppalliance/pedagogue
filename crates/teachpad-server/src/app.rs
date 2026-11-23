//! The router: the routes of each feature, joined, with the two layers
//! that cover them.

use axum::Router;
use axum::middleware;
use teachpad_session::Session;
use tokio::sync::{mpsc, watch};

use crate::assets::UiSource;
use crate::{assets, commands, cross_site, csp, socket};

/// What every route can reach.
#[derive(Debug, Clone)]
pub(crate) struct AppState {
    /// The session the sockets read and the command route writes.
    pub(crate) session: Session,
    /// Where the three UI files come from.
    pub(crate) ui: UiSource,
    /// Turns `true` when the server begins to stop. Every socket task
    /// watches it.
    pub(crate) stopping: watch::Receiver<bool>,
    /// Held by every socket task for as long as it runs. The server waits
    /// until the last holder is gone.
    pub(crate) open: mpsc::Sender<()>,
}

/// Builds the router. The guard covers the socket and the command route,
/// and the content policy covers every response.
pub(crate) fn router(state: AppState) -> Router {
    let guarded = socket::routes()
        .merge(commands::routes())
        .layer(middleware::from_fn(cross_site::guard));
    assets::routes()
        .merge(guarded)
        .layer(middleware::from_fn(csp::stamp))
        .with_state(state)
}

/// Helpers for the router tests of this crate.
#[cfg(test)]
pub(crate) mod testing {
    use axum::Router;
    use axum::response::Response;
    use serde_json::Value;
    use teachpad_session::Session;
    use tokio::sync::{mpsc, watch};

    use super::AppState;
    use crate::assets::UiSource;

    /// Makes a session with no host behind it.
    pub(crate) fn session() -> Session {
        Session::new().0
    }

    /// Builds the router over a new session and the built UI files.
    pub(crate) fn router() -> Router {
        router_over(session())
    }

    /// Builds the router over a given session.
    pub(crate) fn router_over(session: Session) -> Router {
        build(session, UiSource::Embedded)
    }

    /// Builds the router with the UI files from a given source.
    pub(crate) fn router_with(ui: UiSource) -> Router {
        build(session(), ui)
    }

    fn build(session: Session, ui: UiSource) -> Router {
        let (_stop, stopping) = watch::channel(false);
        let (open, _closed) = mpsc::channel(1);
        super::router(AppState {
            session,
            ui,
            stopping,
            open,
        })
    }

    /// Reads a response body as text.
    pub(crate) async fn body_text(response: Response) -> String {
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("the body is read");
        String::from_utf8(bytes.to_vec()).expect("the body is UTF-8")
    }

    /// Reads a response body as JSON. A body that is no JSON reads as
    /// `null`.
    pub(crate) async fn body_json(response: Response) -> Value {
        serde_json::from_str(&body_text(response).await).unwrap_or(Value::Null)
    }
}
