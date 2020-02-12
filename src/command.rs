use anyhow::{anyhow, Result};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Read, Write};
use std::{
    convert::{From, TryFrom},
    net::TcpStream,
    time::Duration,
};

#[repr(u8)]
#[derive(Debug, Eq, PartialEq)]
pub enum Command {
    Ping = 1,
    RequestDownstream = 2,
    SendBuffer = 3,
    Complete = 4,
    Close = 10,
}

impl From<Command> for u8 {
    fn from(cmd: Command) -> Self {
        match cmd {
            Command::Ping => 1,
            Command::RequestDownstream => 2,
            Command::SendBuffer => 3,
            Command::Complete => 4,
            Command::Close => 10,
        }
    }
}

impl TryFrom<u8> for Command {
    type Error = anyhow::Error;
    fn try_from(n: u8) -> Result<Self> {
        match n {
            1 => Ok(Command::Ping),
            2 => Ok(Command::RequestDownstream),
            3 => Ok(Command::SendBuffer),
            4 => Ok(Command::Complete),
            10 => Ok(Command::Close),
            _ => Err(anyhow!("Invalid number {} for command", n)),
        }
    }
}

impl Command {
    pub fn write_ping(stream: &mut TcpStream) -> Result<()> {
        Command::write(stream, Command::Ping)
    }

    pub fn read_ping(stream: &mut TcpStream) -> Result<()> {
        Command::expect(stream, Command::Ping)
    }

    pub fn ping_write_then_read(stream: &mut TcpStream) -> Result<()> {
        Command::write_ping(stream).and_then(|_| Command::read_ping(stream))
    }

    pub fn ping_read_then_write(stream: &mut TcpStream) -> Result<()> {
        Command::read_ping(stream).and_then(|_| Command::write_ping(stream))
    }

    pub fn request_downstream(stream: &mut TcpStream, duration: Duration) -> Result<()> {
        Command::write(stream, Command::RequestDownstream)
            .and(Command::write_duration(stream, duration))
    }

    pub fn expect(stream: &mut TcpStream, expect: Command) -> Result<()> {
        let actual = Command::try_from(stream.read_u8()?)?;
        if actual != expect {
            Err(anyhow!(
                "Unexpected command. expect: {:?}, actual: {:?}",
                expect,
                actual
            ))
        } else {
            Ok(())
        }
    }

    pub fn send_buffer(stream: &mut TcpStream, buff: &[u8]) -> Result<()> {
        Command::write(stream, Command::SendBuffer)?;
        stream
            .write_all(buff)
            .and(stream.flush())
            .map_err(anyhow::Error::from)
    }

    pub fn receive_buffer(stream: &mut TcpStream, buff: &mut [u8]) -> Result<()> {
        stream.read_exact(buff).map_err(anyhow::Error::from)
    }

    pub fn write(stream: &mut TcpStream, cmd: Command) -> Result<()> {
        stream.write_u8(cmd.into()).map_err(anyhow::Error::from)
    }

    pub fn read(stream: &mut TcpStream) -> Result<Command> {
        Command::try_from(stream.read_u8()?)
    }

    pub fn write_duration(stream: &mut TcpStream, duration: Duration) -> Result<()> {
        stream
            .write_u64::<BigEndian>(duration.as_secs())
            .map_err(anyhow::Error::from)
    }

    pub fn read_duration(stream: &mut TcpStream) -> Result<Duration> {
        stream
            .read_u64::<BigEndian>()
            .map_err(anyhow::Error::from)
            .map(|n| Duration::from_secs(n))
    }
}
