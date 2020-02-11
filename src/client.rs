use std::{
    fmt,
    net::{TcpStream, ToSocketAddrs},
    time::Duration,
};
use log::{debug, info};
use anyhow::{Result, Context};
use crate::command::Command;

pub struct Client {
    conn: TcpStream,
}

impl Client {
    pub fn new(addr: impl ToSocketAddrs + fmt::Debug) -> Result<Self> {
        info!("Connecting to {:?}", addr);

        Ok(Self {
            conn: TcpStream::connect_timeout(
                &addr.to_socket_addrs().unwrap().next().unwrap(),
                Duration::from_secs(3),
            )
            .context(format!("Addr:{:?}", addr))?,
        })
    }

    pub fn run(mut self) -> Result<()> {
        self.ping_pon()
    }

    fn ping_pon(&mut self) -> Result<()> {
        Command::ping_pon(&mut self.conn)
            .map(|r| { debug!("Successfully ping to remote server"); r })
    }
}
