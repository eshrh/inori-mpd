#![warn(missing_docs)]

//! MPD client for Rust
//!
//! This crate tries to provide idiomatic Rust API for [Music Player Daemon][mpd].
//! The main entry point to the API is [`Client`] struct, and inherent methods
//! of the struct follow [MPD protocol][proto] for most part, making use of
//! traits to overload different parameters for convenience.
//!
//! [mpd]: https://www.musicpd.org/
//! [proto]: https://www.musicpd.org/doc/protocol/
//!
//! # Usage
//!
//! ```text
//! [dependencies]
//! mpd = "*"
//! ```
//!
//! ```rust,no_run
//! extern crate inori_mpd as mpd;
//!
//! use mpd::Client;
//! use std::net::TcpStream;
//!
//! # fn main() {
//! let mut conn = Client::connect("127.0.0.1:6600").unwrap();
//! conn.volume(100).unwrap();
//! conn.load("My Lounge Playlist", ..).unwrap();
//! conn.play().unwrap();
//! println!("Status: {:?}", conn.status());
//! # }
//! ```

mod convert;
pub mod error;
pub mod version;
pub mod reply;
pub mod status;
pub mod song;
pub mod output;
pub mod playlist;
pub mod plugin;
pub mod stats;
pub mod search;
pub mod message;
pub mod idle;
pub mod mount;
mod sticker;

mod proto;
pub mod client;
#[cfg(feature = "async")]
mod async_client;
/// Async connection establishment and MPD server discovery.
#[cfg(feature = "async")]
pub mod connection;

pub use client::Client;
#[cfg(feature = "async")]
pub use async_client::{AsyncClient, Response};
pub use idle::{Idle, Subsystem};
pub use message::{Channel, Message};
pub use mount::{Mount, Neighbor};
pub use output::Output;
pub use playlist::Playlist;
pub use plugin::Plugin;
pub use search::{Query, Term};
pub use song::{Id, Song};
pub use stats::Stats;
pub use status::{ReplayGain, State, Status};
pub use version::Version;
