//! The Content-Security-Policy stamped on every response.
//!
//! The desktop window loads the UI from this server's address, so the
//! page's policy is this server's to set. Scripts and styles come only from
//! the app's own files, which rules out inline scripts, inline styles and
//! `eval`. Sockets reach loopback alone. WebKit does not count a `ws:`
//! address under `'self'`, so the three loopback spellings are written out,
//! with a port wildcard for the port the operating system assigns.

use axum::extract::Request;
use axum::http::{HeaderValue, header};
use axum::middleware::Next;
use axum::response::Response;

/// The policy, one value for every response.
///
/// `img-src` admits `data:` for the icon the stylesheet draws from an
/// inline image. `form-action 'none'` keeps a quiz form on the page if its
/// script ever fails to take the submit.
pub(crate) const POLICY: &str = "default-src 'self'; script-src 'self'; style-src 'self'; \
                                 connect-src 'self' ws://127.0.0.1:* ws://localhost:* ws://[::1]:*; \
                                 img-src 'self' data:; object-src 'none'; base-uri 'none'; \
                                 form-action 'none'; frame-ancestors 'none'";

/// Stamps the policy on a response, refusals included.
pub(crate) async fn stamp(request: Request, next: Next) -> Response {
    let mut response = next.run(request).await;
    response.headers_mut().insert(
        header::CONTENT_SECURITY_POLICY,
        HeaderValue::from_static(POLICY),
    );
    response
}

#[cfg(test)]
mod tests {
    use super::*;

    use axum::body::Body;
    use axum::http::Request;
    use tower::ServiceExt;

    use crate::app::testing::router;

    #[tokio::test]
    async fn every_response_has_the_exact_policy() {
        // The page, its files, a refusal from each guarded route, and a
        // path that no route serves.
        let requests = [
            ("GET", "/"),
            ("GET", "/app.js"),
            ("GET", "/app.css"),
            ("GET", "/ws"),
            ("POST", "/commands"),
            ("GET", "/no-such-path"),
        ];
        for (method, uri) in requests {
            let request = Request::builder()
                .method(method)
                .uri(uri)
                .body(Body::empty())
                .expect("static request parts are valid");
            let response = router()
                .oneshot(request)
                .await
                .expect("the router is infallible");
            let policy = response
                .headers()
                .get(header::CONTENT_SECURITY_POLICY)
                .unwrap_or_else(|| panic!("{method} {uri} has the policy"));
            assert_eq!(
                policy,
                "default-src 'self'; script-src 'self'; style-src 'self'; connect-src 'self' \
                 ws://127.0.0.1:* ws://localhost:* ws://[::1]:*; img-src 'self' data:; \
                 object-src 'none'; base-uri 'none'; form-action 'none'; frame-ancestors 'none'",
                "for {method} {uri}",
            );
        }
    }
}
