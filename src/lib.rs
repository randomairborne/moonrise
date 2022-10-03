//!  is a framework for building Discord bots which use only slash commands.
//! It can also be used as part of a bot that has slash commands through another communication channel.

mod router;
mod sigval;

pub use twilight_http::Client as Client;

#[cfg(feature = "server")]
mod server;
#[cfg(feature = "server")]
pub use server::Server;
