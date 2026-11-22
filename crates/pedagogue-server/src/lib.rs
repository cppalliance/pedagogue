//! The loopback HTTP server of Pedagogue.
//!
//! It delivers the UI bundle, and it connects each open page to a session
//! over one WebSocket: page and chat state go down, student actions come up.
//!
//! # Examples
//!
//! ```no_run
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! use pedagogue_server::{Options, bind, serve};
//! use pedagogue_session::Session;
//!
//! let (session, _events) = Session::new();
//! let listener = bind(None)?;
//! println!("http://{}", listener.local_addr()?);
//! // A host would write pages into a clone of `session` from here on.
//! serve(listener, session, Options::new(), std::future::pending()).await?;
//! # Ok(())
//! # }
//! ```

mod app;
mod assets;
mod commands;
mod cross_site;
mod csp;
mod error;
mod serve;
mod socket;

pub use crate::error::BindError;
pub use crate::serve::{Options, bind, serve};
