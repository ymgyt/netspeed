use std::{
    convert::{From, TryFrom},
    net::TcpStream,
};
use byteorder::{ReadBytesExt, WriteBytesExt};
use anyhow::{Result, anyhow};

#[repr(u8)]
#[derive(Debug, Eq, PartialEq)]
pub enum Command {
    Ping = 1,
}

impl From<Command> for u8 {
    fn from(cmd: Command) -> Self {
        match cmd {
            Command::Ping => 1,
        }
    }
}

impl TryFrom<u8> for Command {
    type Error = anyhow::Error;
    fn try_from(n: u8) -> Result<Self> {
        match n {
            1 => Ok(Command::Ping),
            _ => Err(anyhow!("Invalid number {} for command", n)),
        }
    }
}

impl Command {
    pub fn write_ping(stream: &mut TcpStream) -> Result<()> {
        stream
            .write_u8(Command::Ping.into())
            .map_err(anyhow::Error::from)
    }

    pub fn read_ping(stream: &mut TcpStream) -> Result<()> {
        match Command::try_from(stream.read_u8()?)? {
            Command::Ping => Ok(()),
        }
    }

    pub fn ping_pon(stream: &mut TcpStream) -> Result<()> {
        Command::write_ping(stream).and_then(|_| Command::read_ping(stream))
    }
}
