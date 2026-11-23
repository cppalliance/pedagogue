//! The navigation policy: the window loads only the app's own address, and
//! every other http or https address opens in the system browser.
//!
//! A link in a lesson never takes the window away from the app, and no
//! other server on this machine, whatever its port or its spelling of
//! loopback, is shown inside the app's window. Other schemes, such as
//! `about:blank`, and text that is no address are left to the webview.

/// What the app does with an address the window is asked to load.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Navigation {
    /// The window loads the address itself.
    Allow,
    /// The address opens in the system browser, and the window stays put.
    OpenExternally,
}

/// Sorts an address against the app's own origin: the same origin loads in
/// the window, and any other http or https address opens in the system
/// browser.
pub(crate) fn classify_navigation(app: &url::Origin, target: &str) -> Navigation {
    let Ok(url) = url::Url::parse(target) else {
        return Navigation::Allow;
    };
    if !matches!(url.scheme(), "http" | "https") {
        return Navigation::Allow;
    }
    if url.origin() == *app {
        Navigation::Allow
    } else {
        Navigation::OpenExternally
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The origin of the app, as the test window was opened on it.
    fn origin_of(url: &str) -> url::Origin {
        url::Url::parse(url)
            .expect("the test address parses")
            .origin()
    }

    #[test]
    fn the_apps_own_addresses_load_in_the_window() {
        let app = origin_of("http://127.0.0.1:7910/");
        for url in [
            "http://127.0.0.1:7910/",
            "http://127.0.0.1:7910/ws",
            "http://127.0.0.1:7910/?page=2#top",
        ] {
            assert_eq!(classify_navigation(&app, url), Navigation::Allow, "{url}");
        }
    }

    #[test]
    fn a_default_port_is_part_of_the_origin() {
        let app = origin_of("https://127.0.0.1/");
        assert_eq!(
            classify_navigation(&app, "https://127.0.0.1:443/"),
            Navigation::Allow
        );
    }

    #[test]
    fn other_loopback_servers_open_in_the_system_browser() {
        let app = origin_of("http://127.0.0.1:7910/");
        for url in [
            "http://127.0.0.1:4000/",
            "https://127.0.0.1:7910/",
            "http://localhost:7910/",
            "http://[::1]:7910/",
        ] {
            assert_eq!(
                classify_navigation(&app, url),
                Navigation::OpenExternally,
                "{url}"
            );
        }
    }

    #[test]
    fn outside_addresses_open_in_the_system_browser() {
        let app = origin_of("http://127.0.0.1:7910/");
        for url in [
            "https://isocpp.org/std/meetings-and-participation",
            "http://192.168.1.10/",
            "https://localhost.evil.example/",
        ] {
            assert_eq!(
                classify_navigation(&app, url),
                Navigation::OpenExternally,
                "{url}"
            );
        }
    }

    #[test]
    fn other_schemes_and_text_that_is_no_address_are_left_to_the_webview() {
        let app = origin_of("http://127.0.0.1:7910/");
        for url in [
            "about:blank",
            "data:text/html,<p>hi</p>",
            "not an address",
            "/relative/path",
        ] {
            assert_eq!(classify_navigation(&app, url), Navigation::Allow, "{url}");
        }
    }
}
