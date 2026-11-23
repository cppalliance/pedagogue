//! The stop: one signal ends the server and every socket.

use serde_json::json;

use crate::common::Served;

#[tokio::test]
async fn the_server_ends_soon_after_the_signal_while_a_socket_is_open() {
    let served = Served::start();
    let mut client = served.connect().await;
    client.next_of("chat").await;
    let mut idle = served.connect().await;
    idle.next_of("chat").await;

    // `stop` waits for the serve future, which ends only once both sockets
    // are closed.
    served.stop().await;

    client.closed().await;
    idle.closed().await;
}

#[tokio::test]
async fn a_server_with_no_socket_open_stops_at_once() {
    let served = Served::start();
    served
        .session
        .apply("new_page", &json!({}))
        .expect("the command applies");

    served.stop().await;
}
