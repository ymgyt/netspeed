use crate::command::Command;
use anyhow::{Context, Result, anyhow};
use log::{debug, info};
use std::io::Read;
use std::{
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
            )
            .context(format!("Addr:{:?}", addr))?,
        })
    }

    pub fn run(mut self) -> Result<()> {
        self.ping_pon()
            .and(self.downstream(Duration::from_secs(2)))
    }

    fn ping_pon(&mut self) -> Result<()> {
        Command::ping_write_then_read(&mut self.conn).map(|r| {
            debug!("Successfully ping to remote server");
            r
        })
    }

    fn downstream(&mut self, duration: Duration) -> Result<()> {
        debug!("Request downstream");
        Command::request_downstream(self.conn.by_ref(), duration)?;

        let mut buff = [0u8; crate::BUFFER_SIZE];
        let  mut read_bytes = 0u64;
        loop {
            match Command::read(self.conn.by_ref())? {
                Command::SendBuffer => {
                    Command::receive_buffer(self.conn.by_ref(), &mut buff)?;
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
