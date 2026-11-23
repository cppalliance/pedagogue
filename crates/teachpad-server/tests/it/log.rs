//! What the server logs about a socket. The release check of each installer
//! reads the first of these lines from the installed app.

use std::io;
use std::sync::{Arc, Mutex, PoisonError};

use tracing_subscriber::fmt::MakeWriter;

use crate::common::Served;

/// Collects what a log subscriber writes.
#[derive(Debug, Clone, Default)]
struct Captured(Arc<Mutex<Vec<u8>>>);

impl Captured {
    /// Returns the level and the message of each line the socket code has
    /// logged, such as `INFO socket opened`.
    fn socket_lines(&self) -> Vec<String> {
        let bytes = self.0.lock().unwrap_or_else(PoisonError::into_inner);
        String::from_utf8_lossy(&bytes)
            .lines()
            .filter_map(|line| {
                let (head, message) = line.split_once(" teachpad_server::socket: ")?;
                let level = head.split_whitespace().last()?;
                Some(format!("{level} {message}"))
            })
            .collect()
    }
}

impl io::Write for Captured {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        self.0
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .extend_from_slice(bytes);
        Ok(bytes.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl MakeWriter<'_> for Captured {
    type Writer = Self;

    fn make_writer(&self) -> Self {
        self.clone()
    }
}

#[tokio::test]
async fn a_socket_is_logged_at_the_default_level_when_it_opens_and_closes() {
    let captured = Captured::default();
    // The subscriber a binary installs, with another writer. Its default
    // level decides which lines an installed app prints.
    let subscriber = tracing_subscriber::fmt()
        .with_writer(captured.clone())
        .finish();
    // The other tests of this binary run beside this one, on threads that
    // have no subscriber. While one subscriber is all there is, `tracing`
    // asks the thread that reaches a log line first whether anyone wants
    // the line, and keeps that answer for every thread. A second
    // subscriber, which wants nothing, makes it ask the subscriber of each
    // thread every time.
    let _second = tracing::Dispatch::new(tracing::subscriber::NoSubscriber::default());
    // The test runs on one thread, so the server's tasks log through this
    // subscriber too.
    let _logging = tracing::subscriber::set_default(subscriber);

    let served = Served::start();
    served.session.assistant_message("Read this section.");
    let mut client = served.connect().await;
    // A frame has arrived, so the connection's task is past its first line.
    client.next().await;
    assert_eq!(captured.socket_lines(), ["INFO socket opened"]);

    // The server ends once every socket has closed.
    served.stop().await;
    assert_eq!(
        captured.socket_lines(),
        ["INFO socket opened", "INFO socket closed ending=Stopping"],
    );
}
