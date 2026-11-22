//! Cross-site request blocking for the loopback server.
//!
//! Any page the user visits can send requests to `127.0.0.1`, so without
//! these checks the socket and the command route are open to request
//! forgery and DNS rebinding. The [`guard`] middleware wraps both routes.
//! It refuses a request that the browser marks `Sec-Fetch-Site:
//! cross-site`. It refuses a request whose `Host` is not loopback, which is
//! the one thing a rebound page cannot forge. It asks every request with a
//! body to declare `application/json`, which forces a preflight that no
//! foreign page passes. Older browsers leave `Sec-Fetch-Site` off a socket
//! upgrade, so the upgrade also checks its `Origin` with
//! [`origin_allowed`]. The UI files stay outside the guard: they hold
//! nothing private.

use axum::extract::Request;
use axum::http::{HeaderMap, Method, header};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};

use crate::error::Refusal;

/// Refuses cross-site requests, rebound hosts and bodies that are not
/// declared as JSON.
pub(crate) async fn guard(request: Request, next: Next) -> Response {
    let is_cross_site = request
        .headers()
        .get("sec-fetch-site")
        .is_some_and(|site| site.as_bytes().eq_ignore_ascii_case(b"cross-site"));
    if is_cross_site || !host_is_loopback(&request) {
        return Refusal::CrossSite.into_response();
    }
    let has_body = [Method::POST, Method::PUT, Method::PATCH].contains(request.method());
    if has_body && !declares_json(request.headers()) {
        return Refusal::NotJson.into_response();
    }
    next.run(request).await
}

/// Says whether the request names a loopback host.
///
/// The authority in the request line wins over the `Host` header. A
/// request that names no host at all passes: a browser always sends one,
/// so that request comes from a native client.
fn host_is_loopback(request: &Request) -> bool {
    let authority = match request.uri().authority() {
        Some(authority) => authority.as_str(),
        None => match request.headers().get(header::HOST) {
            Some(host) => match host.to_str() {
                Ok(host) => host,
                Err(_) => return false,
            },
            None => return true,
        },
    };
    is_loopback_origin(&format!("http://{authority}"))
}

/// Says whether the request declares a JSON body. A parameter such as
/// `charset` is fine.
fn declares_json(headers: &HeaderMap) -> bool {
    headers
        .get(header::CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .map(|value| value.split(';').next().unwrap_or_default().trim())
        .is_some_and(|essence| essence.eq_ignore_ascii_case("application/json"))
}

/// Says whether the `Origin` of a socket upgrade is acceptable: a loopback
/// origin, which covers the desktop window and a browser tab on this
/// server's address, or no origin at all, which is a native client.
pub(crate) fn origin_allowed(headers: &HeaderMap) -> bool {
    let Some(origin) = headers.get(header::ORIGIN) else {
        return true;
    };
    origin.to_str().is_ok_and(is_loopback_origin)
}

/// Says whether `origin` is an http or https address on a loopback host.
/// The opaque `null` origin of a sandboxed frame is refused.
fn is_loopback_origin(origin: &str) -> bool {
    let Ok(url) = url::Url::parse(origin) else {
        return false;
    };
    if !matches!(url.scheme(), "http" | "https") {
        return false;
    }
    match url.host() {
        Some(url::Host::Ipv4(address)) => address.is_loopback(),
        Some(url::Host::Ipv6(address)) => address.is_loopback(),
        Some(url::Host::Domain(domain)) => domain.eq_ignore_ascii_case("localhost"),
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;

    use crate::app::testing::{body_json, router};

    #[test]
    fn loopback_origins_are_allowed() {
        for origin in [
            "http://127.0.0.1:7910",
            "http://127.5.0.1",
            "http://localhost:7910",
            "http://LOCALHOST",
            "http://[::1]:7910",
            "https://127.0.0.1",
        ] {
            assert!(is_loopback_origin(origin), "{origin} must be allowed");
        }
    }

    #[test]
    fn foreign_and_malformed_origins_are_refused() {
        for origin in [
            "https://evil.example",
            "http://192.168.1.10:7910",
            "https://localhost.evil.example",
            "file:///etc/passwd",
            "null",
            "",
        ] {
            assert!(!is_loopback_origin(origin), "{origin} must be refused");
        }
    }

    #[test]
    fn an_upgrade_with_no_origin_is_a_native_client_and_passes() {
        assert!(origin_allowed(&HeaderMap::new()));
    }

    /// Posts a command with the given extra header and returns the status
    /// and the code of the refusal, when there is one.
    async fn post_command(name: &str, value: &str) -> (StatusCode, Option<String>) {
        let request = Request::builder()
            .method("POST")
            .uri("/commands")
            .header("content-type", "application/json")
            .header(name, value)
            .body(Body::from(r#"{ "tool": "new_page" }"#))
            .expect("static request parts are valid");
        let response = router()
            .oneshot(request)
            .await
            .expect("the router is infallible");
        let status = response.status();
        let code = body_json(response).await["error"]["code"]
            .as_str()
            .map(str::to_owned);
        (status, code)
    }

    #[tokio::test]
    async fn a_request_marked_cross_site_is_refused_on_both_guarded_routes() {
        let refused = (StatusCode::FORBIDDEN, Some("cross_site".to_owned()));
        assert_eq!(post_command("sec-fetch-site", "cross-site").await, refused);
        assert_eq!(post_command("sec-fetch-site", "Cross-Site").await, refused);
        for site in ["same-origin", "none"] {
            let (status, _) = post_command("sec-fetch-site", site).await;
            assert_eq!(status, StatusCode::OK, "for {site}");
        }

        let socket = Request::builder()
            .uri("/ws")
            .header("sec-fetch-site", "cross-site")
            .body(Body::empty())
            .expect("static request parts are valid");
        let response = router()
            .oneshot(socket)
            .await
            .expect("the router is infallible");
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
        assert_eq!(body_json(response).await["error"]["code"], "cross_site");
    }

    #[tokio::test]
    async fn a_rebound_host_is_refused_and_the_loopback_names_pass() {
        let (status, code) = post_command("host", "rebound.example:7910").await;
        assert_eq!(status, StatusCode::FORBIDDEN);
        assert_eq!(code.as_deref(), Some("cross_site"));
        for host in ["127.0.0.1:7910", "localhost:7910", "[::1]:7910"] {
            let (status, _) = post_command("host", host).await;
            assert_eq!(status, StatusCode::OK, "for {host}");
        }
    }

    #[tokio::test]
    async fn a_body_has_to_be_declared_as_json() {
        let post = |content_type: &'static str| async move {
            let request = Request::builder()
                .method("POST")
                .uri("/commands")
                .header("content-type", content_type)
                .body(Body::from(r#"{ "tool": "new_page" }"#))
                .expect("static request parts are valid");
            router()
                .oneshot(request)
                .await
                .expect("the router is infallible")
        };

        let refused = post("text/plain").await;
        assert_eq!(refused.status(), StatusCode::UNSUPPORTED_MEDIA_TYPE);
        assert_eq!(body_json(refused).await["error"]["code"], "not_json");

        let taken = post("application/json; charset=utf-8").await;
        assert_eq!(taken.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn the_ui_files_stay_outside_the_guard() {
        let request = Request::builder()
            .uri("/")
            .header("sec-fetch-site", "cross-site")
            .header("host", "rebound.example")
            .body(Body::empty())
            .expect("static request parts are valid");
        let response = router()
            .oneshot(request)
            .await
            .expect("the router is infallible");
        assert_eq!(response.status(), StatusCode::OK);
    }
}
