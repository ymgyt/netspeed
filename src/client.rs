use crate::{command::Command, Result};
use byteorder::{ReadBytesExt, WriteBytesExt};
use log::{debug, info};
use std::{
    convert::TryFrom,
    fmt,
    net::{TcpStream, ToSocketAddrs},
    time::Duration,
};

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
            )?,
        })
    }

    pub fn run(mut self) -> Result<()> {
        self.ping_pon()
    }

    fn ping_pon(&mut self) -> Result<()> {
        debug!("Send ping");
        self.conn.write_u8(Command::Ping.into())?;
        let cmd = self.conn.read_u8()?;
        let cmd = Command::try_from(cmd)?;
        match cmd {
            Command::Ping => {
                debug!("Receive ping");
                Ok(())
            }
        }
    }
}
