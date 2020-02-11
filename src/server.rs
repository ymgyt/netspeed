use crate::{command::Command, Result};
use byteorder::{ReadBytesExt, WriteBytesExt};
#[allow(unused_imports)]
use log::{debug, error, info};
use std::{
    convert::TryFrom,
    fmt,
    net::{SocketAddr, TcpListener, TcpStream, ToSocketAddrs},
    thread,
};

pub struct Server {
    listener: TcpListener,
}

impl Server {
    pub fn new(addr: impl ToSocketAddrs + fmt::Debug) -> Result<Self> {
        info!("Listening on {:?}", addr);
        Ok(Server {
            listener: TcpListener::bind(addr)?,
        })
    }

    pub fn run(self) -> Result<()> {
        for stream in self.listener.incoming() {
            let stream = stream?;
            thread::spawn(move || handle(stream));
        }
        Ok(())
    }
}

fn handle(stream: TcpStream) {
    match stream.peer_addr() {
        Ok(addr) => {
            info!("Incoming connection from {}", addr);

            let mut worker = Worker::new(addr, stream);
            if let Err(err) = worker.run() {
                for cause in err.iter_chain() {
                    error!("{}", cause);
                    if let Some(bt) = cause.backtrace() {
                        if !bt.is_empty() {
                            error!("{}", bt);
                        }
                    }
                }
            }
        }
        Err(err) => error!("Could not get peer address: {}", err),
    }
}

struct Worker {
    peer: String,
    stream: TcpStream,
}

impl Worker {
    fn new(addr: SocketAddr, stream: TcpStream) -> Self {
        Self {
            peer: format!("{}", addr),
            stream,
        }
    }
    fn run(&mut self) -> Result<()> {
        self.ping_pon()
    }

    fn ping_pon(&mut self) -> Result<()> {
        let cmd = self.stream.read_u8()?;
        let cmd = Command::try_from(cmd)?;
        match cmd {
            Command::Ping => {
                self.stream.write_u8(Command::Ping.into())?;
                debug!("{} Successfully ping pon", self);
                Ok(())
            }
        }
    }
}

impl fmt::Display for Worker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(Worker:{}) =>", self.peer.as_str())
    }
}
