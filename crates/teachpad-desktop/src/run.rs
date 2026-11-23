//! Starting and stopping the app: the session, the server, the demo and
//! the window.

use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::Duration;

use anyhow::Context as _;
use tauri::async_runtime::JoinHandle;
use tauri::webview::NewWindowResponse;
use tauri::{WebviewUrl, WebviewWindowBuilder};
use tauri_plugin_opener::OpenerExt as _;
use teachpad_demo::{Pace, STRAW_POLL, Script};
use teachpad_server::{Options, bind, serve};
use teachpad_session::Session;
use tokio::sync::oneshot;

use crate::navigation::{Navigation, classify_navigation};

/// How long a stop waits for the server to close its sockets. Past it the
/// app exits anyway, so closing the window always finishes.
const STOP_GRACE: Duration = Duration::from_secs(3);

/// Runs the desktop app and returns its exit code.
///
/// The app plays the script named by its first argument, or the built-in
/// straw poll lesson, and prints the address it serves on. Everything that
/// can fail on bad input runs before the window opens, and a failure is
/// printed with its chain of causes.
///
/// # Examples
///
/// ```no_run
/// fn main() -> std::process::ExitCode {
///     teachpad_desktop::run()
/// }
/// ```
#[must_use]
pub fn run() -> ExitCode {
    match start() {
        Ok(code) => code,
        Err(error) => {
            eprintln!("error: {error:#}");
            ExitCode::FAILURE
        }
    }
}

#[expect(
    clippy::disallowed_methods,
    reason = "this binary entry point installs the one log subscriber of the process"
)]
fn start() -> anyhow::Result<ExitCode> {
    let subscriber = tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .finish();
    tracing::subscriber::set_global_default(subscriber).context("install the log subscriber")?;

    let script = read_script(std::env::args_os().nth(1).map(PathBuf::from).as_deref())?;
    let listener = bind(None)?;
    let address = listener
        .local_addr()
        .context("read the address of the listener")?;
    let url = url::Url::parse(&format!("http://{address}/")).context("build the app's address")?;
    println!("{url}");

    // The server and the demo run as tasks on Tauri's async runtime, the
    // one runtime of this process.
    let (session, mut events) = Session::new();
    let (stop, stopped) = oneshot::channel::<()>();
    let shutdown = async move {
        // A dropped sender stops the server too.
        let _ = stopped.await;
    };
    let server =
        tauri::async_runtime::spawn(serve(listener, session.clone(), Options::new(), shutdown));
    let demo = tauri::async_runtime::spawn(async move {
        let played = teachpad_demo::run(&session, &mut events, &script, Pace::Live).await;
        if let Err(error) = played {
            let chain = format!("{:#}", anyhow::Error::new(error));
            tracing::error!(error = %chain, "the script stopped");
        }
    });

    let app = tauri::Builder::default()
        .plugin(
            // The page has no access to Tauri, so the plugin's own click
            // script could only swallow a link. Links are handled in Rust.
            tauri_plugin_opener::Builder::new()
                .open_js_links_on_click(false)
                .build(),
        )
        .setup(move |app| {
            // An error returned from here panics inside Tauri and loses
            // its chain, so it is printed here and the app exits.
            if let Err(error) = open_window(app, &url) {
                let error = anyhow::Error::new(error).context("open the window");
                eprintln!("error: {error:#}");
                app.handle().exit(1);
            }
            Ok(())
        })
        .build(tauri::generate_context!())
        .context("build the desktop application")?;
    let code = app.run_return(|_app, _event| {});

    demo.abort();
    tauri::async_runtime::block_on(stop_server(stop, server));
    Ok(u8::try_from(code).map_or(ExitCode::FAILURE, ExitCode::from))
}

/// Reads the script at `path`, or the built-in straw poll lesson.
fn read_script(path: Option<&Path>) -> anyhow::Result<Script> {
    let Some(path) = path else {
        return Script::parse(STRAW_POLL).context("parse the built-in script");
    };
    let text = std::fs::read_to_string(path)
        .with_context(|| format!("read the script {}", path.display()))?;
    Script::parse(&text).with_context(|| format!("parse the script {}", path.display()))
}

/// Opens the one window of the app on the server's address.
fn open_window(app: &mut tauri::App, url: &url::Url) -> tauri::Result<()> {
    let for_links = app.handle().clone();
    let for_new_windows = app.handle().clone();
    let link_origin = url.origin();
    let new_window_origin = url.origin();
    WebviewWindowBuilder::new(app, "main", WebviewUrl::External(url.clone()))
        .title("TeachPad")
        .inner_size(1280.0, 800.0)
        .min_inner_size(720.0, 480.0)
        .on_navigation(
            move |target| match classify_navigation(&link_origin, target.as_str()) {
                Navigation::Allow => true,
                Navigation::OpenExternally => {
                    open_outside(&for_links, target.as_str());
                    false
                }
            },
        )
        // A link that asks for a new window never gets one inside the app.
        .on_new_window(move |target, _features| {
            if classify_navigation(&new_window_origin, target.as_str())
                == Navigation::OpenExternally
            {
                open_outside(&for_new_windows, target.as_str());
            }
            NewWindowResponse::Deny
        })
        .build()?;
    Ok(())
}

/// Opens an address in the system browser.
fn open_outside(app: &tauri::AppHandle, target: &str) {
    if let Err(error) = app.opener().open_url(target, None::<&str>) {
        tracing::warn!(%error, target, "a link could not be opened in the system browser");
    }
}

/// Signals the server to stop and waits for it, for the grace window at
/// most.
async fn stop_server(stop: oneshot::Sender<()>, server: JoinHandle<std::io::Result<()>>) {
    // The receiver lives in the server task. If that task is gone, the
    // wait below ends at once.
    let _ = stop.send(());
    match tokio::time::timeout(STOP_GRACE, server).await {
        Ok(Ok(Ok(()))) => {}
        Ok(Ok(Err(error))) => tracing::error!(%error, "the server ended with an error"),
        Ok(Err(error)) => tracing::error!(%error, "the server task failed"),
        Err(_) => {
            tracing::warn!("the server was still closing its sockets when the grace window ended");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_built_in_lesson_plays_when_no_script_is_named() {
        read_script(None).expect("the built-in script parses");
    }

    #[test]
    fn a_script_that_cannot_be_read_is_reported_with_its_path_and_cause() {
        let folder = tempfile::TempDir::new().expect("a temp folder");
        let missing = folder.path().join("missing.json");
        let error = read_script(Some(&missing)).expect_err("the file is missing");
        let chain = format!("{error:#}");
        assert!(chain.starts_with(&format!("read the script {}: ", missing.display())));

        let broken = folder.path().join("broken.json");
        std::fs::write(
            &broken,
            r#"[{ "tool": "new_page" }, { "shout": "Hello." }]"#,
        )
        .expect("the script is written");
        let error = read_script(Some(&broken)).expect_err("the second step is no step");
        let chain = format!("{error:#}");
        assert!(chain.starts_with(&format!(
            "parse the script {}: step 2 is not a step: ",
            broken.display()
        )));
    }

    #[test]
    fn a_script_on_disk_is_read() {
        let folder = tempfile::TempDir::new().expect("a temp folder");
        let path = folder.path().join("one-page.json");
        std::fs::write(&path, r#"[{ "tool": "new_page" }]"#).expect("the script is written");
        read_script(Some(&path)).expect("the script parses");
    }
}
