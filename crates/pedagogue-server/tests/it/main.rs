//! Server integration tests against a served listener: the socket both
//! ways, what it logs, and the stop.

#[path = "../common/mod.rs"]
mod common;

mod log;
mod socket;
mod stop;
