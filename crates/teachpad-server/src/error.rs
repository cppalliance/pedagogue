//! The refusals this server sends over HTTP, and the error of a listener
//! that cannot open.

use std::net::SocketAddr;

use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;
use teachpad_session::{CommandError, CommandErrorKind};
use thiserror::Error;

/// A listener that cannot open.
///
/// `Display` names the address, and the operating system's reason is
/// attached as a source.
///
/// # Examples
///
/// ```
/// use teachpad_server::bind;
///
/// let taken = bind(None)?;
/// let port = taken.local_addr().expect("the listener has an address").port();
/// let error = bind(Some(port)).expect_err("the port is taken");
/// assert_eq!(error.to_string(), format!("cannot listen on 127.0.0.1:{port}"));
/// # Ok::<(), teachpad_server::BindError>(())
/// ```
#[derive(Debug, Error)]
#[error("cannot listen on {address}")]
pub struct BindError {
    pub(crate) address: SocketAddr,
    #[source]
    pub(crate) source: std::io::Error,
}

/// A request this server refuses. Each case has one status and one code,
/// and every refusal leaves as the same small JSON object.
#[derive(Debug, Error)]
pub(crate) enum Refusal {
    /// The request came from another site: it is marked cross-site, it
    /// names a foreign host, or its socket upgrade has a foreign origin.
    #[error("cross-site request refused")]
    CrossSite,
    /// A request with a body did not declare it as JSON.
    #[error("the request body is not application/json")]
    NotJson,
    /// The body is not a tool call.
    #[error("the body is not a tool call: {0}")]
    NotACall(String),
    /// One of the three UI files is missing from the build or the folder.
    #[error("ui file not found: {0}")]
    FileMissing(&'static str),
    /// The session refused a page command.
    #[error("{}", .0.refusal())]
    Command(#[source] CommandError),
}

impl Refusal {
    fn status(&self) -> StatusCode {
        match self {
            Self::CrossSite => StatusCode::FORBIDDEN,
            Self::NotJson => StatusCode::UNSUPPORTED_MEDIA_TYPE,
            Self::NotACall(_) => StatusCode::BAD_REQUEST,
            Self::FileMissing(_) => StatusCode::NOT_FOUND,
            Self::Command(error) => match error.kind() {
                CommandErrorKind::UnknownTool => StatusCode::NOT_FOUND,
                CommandErrorKind::InvalidArguments => StatusCode::UNPROCESSABLE_ENTITY,
                CommandErrorKind::Refused => StatusCode::CONFLICT,
            },
        }
    }

    fn code(&self) -> &'static str {
        match self {
            Self::CrossSite => "cross_site",
            Self::NotJson => "not_json",
            Self::NotACall(_) => "not_a_call",
            Self::FileMissing(_) => "file_missing",
            Self::Command(error) => match error.kind() {
                CommandErrorKind::UnknownTool => "unknown_tool",
                CommandErrorKind::InvalidArguments => "invalid_arguments",
                CommandErrorKind::Refused => "refused",
            },
        }
    }
}

impl IntoResponse for Refusal {
    fn into_response(self) -> Response {
        let body = json!({ "error": { "code": self.code(), "message": self.to_string() } });
        (self.status(), Json(body)).into_response()
    }
}
