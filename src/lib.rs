pub mod cli;
pub mod client;
pub mod command;
pub mod logger;
pub mod server;

pub use client::Client;
pub use server::Server;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
