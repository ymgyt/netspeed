use crate::Result;
use anyhow::anyhow;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::{
    convert::{From, TryFrom},
    io::{Read, Write},
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

pub struct Operator {
    conn: TcpStream,
}

impl Operator {
    pub fn new(conn: TcpStream) -> Self {
        Self { conn }
    }
    pub fn ping_write_then_read(&mut self) -> Result<()> {
        self.write_ping().and(self.read_ping())
    }

    pub fn ping_read_then_write(&mut self) -> Result<()> {
        self.read_ping().and(self.write_ping())
    }

    fn write_ping(&mut self) -> Result<()> {
        self.write(Command::Ping)
    }

    fn read_ping(&mut self) -> Result<()> {
        self.expect(Command::Ping)
    }

    pub fn request_downstream(&mut self, duration: Duration) -> Result<()> {
        self.write(Command::RequestDownstream)
            .and(self.write_duration(duration))
    }

    pub fn send_buffer(&mut self, buff: &[u8]) -> Result<()> {
        self.write(Command::SendBuffer)?;
        Write::by_ref(&mut self.conn)
            .write_all(buff)
            .and(Write::by_ref(&mut self.conn).flush())
            .map_err(anyhow::Error::from)
    }

    pub fn receive_buffer(&mut self, buff: &mut [u8]) -> Result<()> {
        Read::by_ref(&mut self.conn)
            .read_exact(buff)
            .map_err(anyhow::Error::from)
    }

    pub fn write(&mut self, cmd: Command) -> Result<()> {
        Write::by_ref(&mut self.conn)
            .write_u8(cmd.into())
            .map_err(anyhow::Error::from)
    }

    pub fn read(&mut self) -> Result<Command> {
        Command::try_from(Read::by_ref(&mut self.conn).read_u8()?)
    }

    pub fn write_duration(&mut self, duration: Duration) -> Result<()> {
        Write::by_ref(&mut self.conn)
            .write_u64::<BigEndian>(duration.as_secs())
            .map_err(anyhow::Error::from)
    }

    pub fn read_duration(&mut self) -> Result<Duration> {
        Read::by_ref(&mut self.conn)
            .read_u64::<BigEndian>()
            .map_err(anyhow::Error::from)
            .map(Duration::from_secs)
    }

    pub fn expect(&mut self, expect: Command) -> Result<()> {
        let actual = Command::try_from(Read::by_ref(&mut self.conn).read_u8()?)?;
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
}
