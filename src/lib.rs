pub mod cli;
pub mod client;
pub mod command;
pub mod logger;
pub mod server;

pub use client::Client;
pub use server::Server;

pub const BUFFER_SIZE: usize = 1024 * 1024;
