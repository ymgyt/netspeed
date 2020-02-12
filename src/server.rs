use crate::command::Command;
use anyhow::{anyhow, Context, Result};
#[allow(unused_imports)]
use byteorder::{ReadBytesExt, WriteBytesExt};
#[allow(unused_imports)]
use log::{debug, error, info};
use std::{
    fmt,
    io::{self, Read},
    net::{SocketAddr, TcpListener, TcpStream, ToSocketAddrs},
    thread, time,
};

pub struct Server {
    listener: TcpListener,
}

impl Server {
    pub fn new(addr: impl ToSocketAddrs + fmt::Debug) -> Result<Self> {
        info!("Listening on {:?}", addr);
        Ok(Server {
            listener: TcpListener::bind(addr).context("Listener binding")?,
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
            info!("Handle incoming connection. dispatch worker {}", addr);
            if let Err(err) = Worker::dispatch(addr, stream) {
                eprintln!("{:#?}", err);
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
    fn dispatch(addr: SocketAddr, stream: TcpStream) -> Result<()> {
        Worker::new(addr, stream).run()
    }
    fn new(addr: SocketAddr, stream: TcpStream) -> Self {
        Self {
            peer: format!("{}", addr),
            stream,
        }
    }
    fn run(&mut self) -> Result<()> {
        self.ping_pon()?;
        info!("{} Successfully ping to client", self);
        loop {
            let cmd = Command::read(self.stream.by_ref())
                .or_else(|err| {
                    if let Some(io_err) = err.downcast_ref::<io::Error>() {
                        if io_err.kind() == io::ErrorKind::UnexpectedEof {
                            info!("{} Closed by remote", self);
                            return Ok(Command::Close);
                        }
                    }
                    Err(err)
                })
                .context("Read command")?;

            match cmd {
                Command::RequestDownstream => {
                    info!("{} Handle downstream", self);
                    self.handle_downstream()?;
                    info!("{} Successfully handle downstream", self);
                }
                Command::Close => return Ok(()),
                _ => return Err(anyhow!("Unexpected command {:?}", cmd)),
            }
        }
    }

    fn ping_pon(&mut self) -> Result<()> {
        Command::ping_read_then_write(self.stream.by_ref())
    }

    fn handle_downstream(&mut self) -> Result<()> {
        let timeout = Command::read_duration(self.stream.by_ref())?;
        debug!("{} Timeout: {:?}", self, timeout);

        let start = time::Instant::now();
        let buff = [0u8; crate::BUFFER_SIZE];
        loop {
            Command::send_buffer(self.stream.by_ref(), &buff)?;
            if start.elapsed() >= timeout {
                break;
            }
        }
        Command::write(self.stream.by_ref(), Command::Complete)
    }
}

impl fmt::Display for Worker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(Worker:{}) =>", self.peer.as_str())
    }
}
