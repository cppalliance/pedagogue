//! Shared helpers for the server integration tests: a server on a real
//! listener, and a small typed socket client.
//!
//! These tests run on real time. Every read has a deadline in place of a
//! sleep, since a paused clock fires timers early while a socket is in
//! flight.

// clippy.toml's allow-expect-in-tests covers #[test] functions and
// #[cfg(test)] modules only, so these helpers ask for it themselves.
#![expect(
    clippy::expect_used,
    reason = "test helpers fail by panicking with the invariant named"
)]

use std::net::SocketAddr;
use std::time::{Duration, UNIX_EPOCH};

use futures_util::{SinkExt, StreamExt};
use pedagogue_server::{Options, bind, serve};
use pedagogue_session::{Session, StudentEvent, StudentEvents};
use serde_json::Value;
use tokio::net::TcpStream;
use tokio::sync::oneshot;
use tokio::task::JoinHandle;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::tungstenite::{self, Message};
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

/// How long a test waits for something the server should do at once.
pub(crate) const DEADLINE: Duration = Duration::from_secs(10);

/// A server on a real listener, with the session behind it.
pub(crate) struct Served {
    pub(crate) session: Session,
    pub(crate) events: StudentEvents,
    address: SocketAddr,
    stop: oneshot::Sender<()>,
    task: JoinHandle<std::io::Result<()>>,
}

impl Served {
    /// Starts a server over a new session on a port the system picks.
    pub(crate) fn start() -> Self {
        let (session, events) =
            Session::with_clock(|| UNIX_EPOCH + Duration::from_secs(1_795_300_000));
        let listener = bind(None).expect("a loopback port opens");
        let address = listener.local_addr().expect("the listener has an address");
        let (stop, stopped) = oneshot::channel::<()>();
        let shutdown = async move {
            // A dropped sender stops the server too.
            let _ = stopped.await;
        };
        let task = tokio::spawn(serve(listener, session.clone(), Options::new(), shutdown));
        Self {
            session,
            events,
            address,
            stop,
            task,
        }
    }

    /// Returns the address of the socket route.
    pub(crate) fn socket_url(&self) -> String {
        format!("ws://{}/ws", self.address)
    }

    /// Returns the origin a page served by this server has.
    pub(crate) fn origin(&self) -> String {
        format!("http://{}", self.address)
    }

    /// Opens a socket the way a native client does, with no origin.
    pub(crate) async fn connect(&self) -> Client {
        let (socket, _) = tokio_tungstenite::connect_async(self.socket_url())
            .await
            .expect("the socket connects");
        Client { socket }
    }

    /// Takes the next student event, within the deadline.
    pub(crate) async fn next_event(&mut self) -> StudentEvent {
        tokio::time::timeout(DEADLINE, self.events.recv())
            .await
            .expect("an event arrives within the deadline")
            .expect("the session is alive")
    }

    /// Signals the server to stop and waits for its future to end, within
    /// the deadline.
    pub(crate) async fn stop(self) {
        // The receiver lives inside the serve future, which is still running.
        let _ = self.stop.send(());
        tokio::time::timeout(DEADLINE, self.task)
            .await
            .expect("the server stops within the deadline")
            .expect("the serve task does not panic")
            .expect("the server ends cleanly");
    }
}

/// Tries to open a socket with an `Origin` header, as a browser page would.
pub(crate) async fn connect_from(url: &str, origin: &str) -> Result<(), tungstenite::Error> {
    let mut request = url.into_client_request().expect("the request builds");
    request.headers_mut().insert(
        "origin",
        origin.parse().expect("the origin is a header value"),
    );
    tokio_tungstenite::connect_async(request).await.map(|_| ())
}

/// A socket client that speaks JSON frames.
pub(crate) struct Client {
    socket: WebSocketStream<MaybeTlsStream<TcpStream>>,
}

impl Client {
    /// Takes the next frame as JSON, within the deadline.
    pub(crate) async fn next(&mut self) -> Value {
        let message = tokio::time::timeout(DEADLINE, self.socket.next())
            .await
            .expect("a frame arrives within the deadline")
            .expect("the socket is open")
            .expect("the socket has no error");
        let text = message.into_text().expect("the frame is text");
        serde_json::from_str(text.as_str()).expect("the frame is JSON")
    }

    /// Takes frames until one is of the given type, and returns that one.
    pub(crate) async fn next_of(&mut self, kind: &str) -> Value {
        loop {
            let frame = self.next().await;
            if frame["type"] == kind {
                return frame;
            }
        }
    }

    /// Sends one JSON frame.
    pub(crate) async fn send(&mut self, frame: &Value) {
        self.send_text(&frame.to_string()).await;
    }

    /// Sends one text frame as it is.
    pub(crate) async fn send_text(&mut self, text: &str) {
        self.socket
            .send(Message::text(text))
            .await
            .expect("the frame is sent");
    }

    /// Waits until the server closes the socket, within the deadline.
    pub(crate) async fn closed(&mut self) {
        let wait = async {
            loop {
                match self.socket.next().await {
                    Some(Ok(Message::Close(_)) | Err(_)) | None => return,
                    Some(Ok(_)) => {}
                }
            }
        };
        tokio::time::timeout(DEADLINE, wait)
            .await
            .expect("the server closes the socket within the deadline");
    }
}
