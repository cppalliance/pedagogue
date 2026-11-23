//! Opening the listener and serving on it.
//!
//! Both are plain functions. [`bind`] returns the listener before any task
//! runs, so the caller knows the address first. [`serve`] is one future for
//! the caller to run on its own runtime, and it stops on the caller's
//! signal.

use std::future::Future;
use std::net::{Ipv4Addr, SocketAddr, TcpListener};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use teachpad_session::Session;
use tokio::sync::{mpsc, watch};

use crate::app::{AppState, router};
use crate::assets::UiSource;
use crate::error::BindError;

/// How a server is set up.
///
/// # Examples
///
/// ```
/// use teachpad_server::Options;
///
/// // The UI files built with the server.
/// let built = Options::new();
/// // The files `npm run watch` keeps current, for the styling loop.
/// let from_disk = Options::new().with_ui_folder("ui/dist");
/// # let _ = (built, from_disk);
/// ```
#[derive(Debug, Clone, Default)]
pub struct Options {
    ui_folder: Option<PathBuf>,
}

impl Options {
    /// Returns the default setup: the UI files built with the server.
    ///
    /// # Examples
    ///
    /// ```
    /// use teachpad_server::Options;
    ///
    /// let options = Options::new();
    /// assert!(options.ui_folder().is_none());
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Serves `index.html`, `app.js` and `app.css` from a folder on disk in
    /// place of the built ones. The files are read on every request, so a
    /// rebuilt bundle shows on refresh.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::Path;
    ///
    /// use teachpad_server::Options;
    ///
    /// let options = Options::new().with_ui_folder("ui/dist");
    /// assert_eq!(options.ui_folder(), Some(Path::new("ui/dist")));
    /// ```
    #[must_use]
    pub fn with_ui_folder(mut self, folder: impl Into<PathBuf>) -> Self {
        self.ui_folder = Some(folder.into());
        self
    }

    /// Returns the folder the UI files are read from, when one is set.
    ///
    /// # Examples
    ///
    /// ```
    /// use teachpad_server::Options;
    ///
    /// assert!(Options::new().ui_folder().is_none());
    /// ```
    #[must_use]
    pub fn ui_folder(&self) -> Option<&Path> {
        self.ui_folder.as_deref()
    }
}

/// Opens a listener on `127.0.0.1`, on the given port or on one the
/// operating system picks.
///
/// The listener is open when this returns, so its address is the app's
/// address before any task runs.
///
/// # Errors
///
/// Returns a [`BindError`] when the address cannot be opened, for example
/// because another program holds the port.
///
/// # Examples
///
/// ```
/// use teachpad_server::bind;
///
/// let listener = bind(None)?;
/// let address = listener.local_addr().expect("the listener has an address");
/// assert!(address.ip().is_loopback());
/// println!("http://{address}");
/// # Ok::<(), teachpad_server::BindError>(())
/// ```
pub fn bind(port: Option<u16>) -> Result<TcpListener, BindError> {
    let address = SocketAddr::from((Ipv4Addr::LOCALHOST, port.unwrap_or(0)));
    TcpListener::bind(address).map_err(|source| BindError { address, source })
}

/// Serves the UI and its socket on a listener until `shutdown` resolves.
///
/// This spawns one task for each connection on the caller's runtime, and
/// each socket task watches the same signal and closes its socket. The
/// future ends once `shutdown` has resolved and every connection is
/// closed. A caller that wants a bound on that wait puts a timeout around
/// the future and drops it.
///
/// # Errors
///
/// Returns the `std::io::Error` of a listener that the runtime cannot take
/// over.
///
/// # Examples
///
/// ```no_run
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// use teachpad_server::{Options, bind, serve};
/// use teachpad_session::Session;
///
/// let (session, _events) = Session::new();
/// let listener = bind(None)?;
/// println!("http://{}", listener.local_addr()?);
/// let (stop, stopped) = tokio::sync::oneshot::channel::<()>();
/// let shutdown = async move {
///     // A dropped sender stops the server too.
///     let _ = stopped.await;
/// };
/// serve(listener, session, Options::new(), shutdown).await?;
/// # drop(stop);
/// # Ok(())
/// # }
/// ```
pub async fn serve<F>(
    listener: TcpListener,
    session: Session,
    options: Options,
    shutdown: F,
) -> std::io::Result<()>
where
    F: Future<Output = ()> + Send + 'static,
{
    listener.set_nonblocking(true)?;
    let listener = tokio::net::TcpListener::from_std(listener)?;
    let (stop, stopping) = watch::channel(false);
    let stop = Arc::new(stop);
    let (open, mut closed) = mpsc::channel::<()>(1);
    let ui = options.ui_folder.map_or(UiSource::Embedded, |folder| {
        UiSource::Folder(Arc::from(folder))
    });
    let app = router(AppState {
        session,
        ui,
        stopping,
        open,
    });

    let signal = {
        let stop = Arc::clone(&stop);
        async move {
            shutdown.await;
            stop.send_replace(true);
        }
    };
    let served = axum::serve(listener, app)
        .with_graceful_shutdown(signal)
        .await;
    // On an early end the sockets are told here.
    stop.send_replace(true);
    // The router is gone by now, so the socket tasks hold the only senders
    // left, and the channel closes when the last of them ends.
    let _ = closed.recv().await;
    served
}
