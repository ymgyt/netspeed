use crate::{
    command::{Command, Operator},
    Result,
};
use anyhow::{anyhow, Context};
use log::{debug, info};
use std::{
    fmt,
    net::{TcpStream, ToSocketAddrs},
    time::Duration,
};

pub struct Client {
    operator: Operator,
}

impl Client {
    pub fn new(addr: impl ToSocketAddrs + fmt::Debug) -> Result<Self> {
        info!("Connecting to {:?}", addr);

        Ok(Self {
            operator: Operator::new(
                TcpStream::connect_timeout(
                    &addr.to_socket_addrs().unwrap().next().unwrap(),
                    Duration::from_secs(3),
                )
                .context(format!("Addr:{:?}", addr))?,
            ),
        })
    }

    pub fn run(mut self) -> Result<()> {
        self.ping_pon().and(self.downstream(Duration::from_secs(2)))
    }

    fn ping_pon(&mut self) -> Result<()> {
        self.operator.ping_write_then_read().map(|r| {
            debug!("Successfully ping to remote server");
            r
        })
    }

    fn downstream(&mut self, duration: Duration) -> Result<()> {
        debug!("Request downstream");
        self.operator.request_downstream(duration)?;

        let mut buff = [0u8; crate::BUFFER_SIZE];
        let mut read_bytes = 0u64;
        loop {
            match self.operator.read()? {
                Command::SendBuffer => {
                    self.operator.receive_buffer(&mut buff)?;
                    read_bytes = read_bytes.saturating_add(crate::BUFFER_SIZE as u64);
                }
                Command::Complete => {
                    info!("Success! {}MiB", read_bytes / 1024 / 1024);
                    return Ok(());
                }
                _ => return Err(anyhow!("Unexpected command")),
            }
        }
    }
}
