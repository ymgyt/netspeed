pub mod cli;
pub mod client;
pub mod command;
pub mod logger;
pub mod server;

pub use client::Client;
pub use server::Server;

use failure::Error;

pub type Result<T> = std::result::Result<T, Error>;
