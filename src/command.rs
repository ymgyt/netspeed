use std::convert::{From, TryFrom};

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
    type Error = String;
    fn try_from(n: u8) -> Result<Self, Self::Error> {
        match n {
            1 => Ok(Command::Ping),
            _ => Err(format!("Invalid number for command: {}", n)),
        }
    }
}
