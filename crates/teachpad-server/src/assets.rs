//! The three UI files and the routes that serve them.
//!
//! The files are named in code. A request picks one of them by its route,
//! so a request path never becomes a file path.

use std::path::Path;
use std::sync::Arc;

use axum::Router;
use axum::extract::State;
use axum::http::header;
use axum::response::{IntoResponse, Response};
use axum::routing::get;

use crate::app::AppState;
use crate::error::Refusal;

/// The UI files under `$OUT_DIR/ui-dist/`, written by this crate's build
/// script. A debug build reads them from that folder at request time, and
/// a release build holds them inside the binary.
#[derive(rust_embed::Embed)]
#[folder = "$OUT_DIR/ui-dist/"]
struct Embedded;

/// Where the UI files come from.
#[derive(Debug, Clone, Default)]
pub(crate) enum UiSource {
    /// The files built with this crate.
    #[default]
    Embedded,
    /// The files of the same names in a folder on disk, read on every
    /// request, so a rebuilt bundle shows on refresh.
    Folder(Arc<Path>),
}

/// One of the three files the UI is made of.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum UiFile {
    Page,
    Script,
    Styles,
}

impl UiFile {
    fn name(self) -> &'static str {
        match self {
            Self::Page => "index.html",
            Self::Script => "app.js",
            Self::Styles => "app.css",
        }
    }

    fn content_type(self) -> &'static str {
        match self {
            Self::Page => "text/html; charset=utf-8",
            Self::Script => "text/javascript; charset=utf-8",
            Self::Styles => "text/css; charset=utf-8",
        }
    }
}

/// The routes of the UI files: the page at `/`, and its script and styles.
pub(crate) fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(page))
        .route("/app.js", get(script))
        .route("/app.css", get(styles))
}

async fn page(State(state): State<AppState>) -> Response {
    serve(&state.ui, UiFile::Page).await
}

async fn script(State(state): State<AppState>) -> Response {
    serve(&state.ui, UiFile::Script).await
}

async fn styles(State(state): State<AppState>) -> Response {
    serve(&state.ui, UiFile::Styles).await
}

/// Answers with one UI file, or with a refusal when its source lacks it.
async fn serve(source: &UiSource, file: UiFile) -> Response {
    let bytes = match source {
        UiSource::Embedded => Embedded::get(file.name()).map(|found| found.data.into_owned()),
        UiSource::Folder(folder) => match tokio::fs::read(folder.join(file.name())).await {
            Ok(bytes) => Some(bytes),
            Err(error) => {
                tracing::warn!(file = file.name(), %error, "a ui file could not be read");
                None
            }
        },
    };
    match bytes {
        Some(bytes) => ([(header::CONTENT_TYPE, file.content_type())], bytes).into_response(),
        None => Refusal::FileMissing(file.name()).into_response(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;

    use crate::app::testing::{body_json, body_text, router, router_with};

    async fn get_from(app: Router, uri: &str) -> Response {
        let request = Request::builder()
            .uri(uri)
            .body(Body::empty())
            .expect("static request parts are valid");
        app.oneshot(request)
            .await
            .expect("the router is infallible")
    }

    fn content_type(response: &Response) -> &str {
        response
            .headers()
            .get(header::CONTENT_TYPE)
            .and_then(|value| value.to_str().ok())
            .unwrap_or_default()
    }

    #[tokio::test]
    async fn the_three_built_files_are_served_by_name() {
        let page = get_from(router(), "/").await;
        assert_eq!(page.status(), StatusCode::OK);
        assert_eq!(content_type(&page), "text/html; charset=utf-8");
        let html = body_text(page).await;
        assert!(html.contains("<main class=\"textbook\">"));
        assert!(html.contains("src=\"/app.js\""));
        assert!(html.contains("href=\"/app.css\""));
        // The build puts the icons of both design files into the page.
        assert!(html.contains("id=\"i-info\""));
        assert!(html.contains("id=\"i-send\""));

        let script = get_from(router(), "/app.js").await;
        assert_eq!(script.status(), StatusCode::OK);
        assert_eq!(content_type(&script), "text/javascript; charset=utf-8");

        let styles = get_from(router(), "/app.css").await;
        assert_eq!(styles.status(), StatusCode::OK);
        assert_eq!(content_type(&styles), "text/css; charset=utf-8");
        assert!(body_text(styles).await.contains("--paper"));
    }

    #[tokio::test]
    async fn no_other_path_reaches_a_file() {
        for uri in [
            "/index.html",
            "/app.js.map",
            "/../Cargo.toml",
            "/src/main.ts",
        ] {
            let response = get_from(router(), uri).await;
            assert_eq!(response.status(), StatusCode::NOT_FOUND, "for {uri}");
        }
    }

    #[tokio::test]
    async fn a_folder_serves_its_own_three_files() {
        let folder = tempfile::TempDir::new().expect("a temp folder");
        std::fs::write(folder.path().join("index.html"), "<p>from the folder</p>")
            .expect("the page is written");
        std::fs::write(folder.path().join("app.css"), ".from-the-folder {}")
            .expect("the styles are written");
        let source = UiSource::Folder(Arc::from(folder.path()));

        let page = get_from(router_with(source.clone()), "/").await;
        assert_eq!(content_type(&page), "text/html; charset=utf-8");
        assert_eq!(body_text(page).await, "<p>from the folder</p>");
        let styles = get_from(router_with(source.clone()), "/app.css").await;
        assert_eq!(body_text(styles).await, ".from-the-folder {}");

        // The folder has no script, and the built one stays out of it.
        let script = get_from(router_with(source), "/app.js").await;
        assert_eq!(script.status(), StatusCode::NOT_FOUND);
        let refusal = body_json(script).await;
        assert_eq!(refusal["error"]["code"], "file_missing");
        assert_eq!(refusal["error"]["message"], "ui file not found: app.js");
    }
}
