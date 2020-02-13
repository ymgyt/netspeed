pub mod cli;
pub mod client;
pub mod command;
pub mod logger;
pub mod server;
pub mod util;

pub use client::Client;
pub use server::Server;

pub type Result<T> = std::result::Result<T, anyhow::Error>;

pub const BUFFER_SIZE: usize = 1024 * 1024;
