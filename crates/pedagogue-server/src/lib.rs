//! The loopback HTTP server of Pedagogue.
//!
//! It delivers the UI bundle, and it connects each open page to a session
//! over one WebSocket: page and chat state go down, student actions come up.
