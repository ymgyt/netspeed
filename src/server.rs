use crate::command::DeclineReason;
use crate::{
    command::{Command, Operator},
    util, Result,
};
use anyhow::{anyhow, Context};
#[allow(unused_imports)]
use byteorder::{ReadBytesExt, WriteBytesExt};
#[allow(unused_imports)]
use log::{debug, error, info, warn};
use std::{
    fmt, io,
    net::{SocketAddr, TcpListener, TcpStream, ToSocketAddrs},
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    thread,
};

pub const DEFAULT_MAX_THREADS: u32 = 100;

pub struct Server {
    listener: TcpListener,
    dispatcher: Arc<Dispatcher>,
}

impl Server {
    pub fn new(addr: impl ToSocketAddrs + fmt::Debug, max_threads: u32) -> Result<Self> {
        info!("Listening on {:?} max threads: {}", addr, max_threads);
        Ok(Server {
            listener: TcpListener::bind(addr).context("Listener binding")?,
            dispatcher: Arc::new(Dispatcher::new(max_threads)),
        })
    }

    pub fn run(self) -> Result<()> {
        for stream in self.listener.incoming() {
            self.dispatcher.dispatch(stream?)
        }
        Ok(())
    }
}

struct Dispatcher {
    max_workers: u32,
    active_workers: AtomicUsize,
}

impl Dispatcher {
    fn new(max_threads: u32) -> Self {
        Self {
            active_workers: AtomicUsize::new(0),
            max_workers: max_threads,
        }
    }

    fn dispatch(self: &Arc<Self>, stream: TcpStream) {
        let current_workers = self.active_workers.load(Ordering::Relaxed) as u32;
        if current_workers >= self.max_workers {
            warn!(
                "Max Threads/Workers counts exceeded. ({}/{})",
                current_workers, self.max_workers
            );
            self.decline(stream);
        } else {
            info!(
                "Pass concurrent threads check. ({}/{})",
                current_workers, self.max_workers
            );
            self.active_workers.fetch_add(1, Ordering::Relaxed);
            self.dispatch_worker(stream)
        }
    }

    fn decline(self: &Arc<Self>, stream: TcpStream) {
        let mut operator = Operator::new(stream);
        if let Err(err) =
            operator.write_decline(DeclineReason::MaxThreadsExceed(self.max_workers), false)
        {
            error!("{:#?}", err);
        }
    }

    fn dispatch_worker(self: &Arc<Self>, stream: TcpStream) {
        let dispatcher: Arc<Dispatcher> = Arc::clone(self);
        thread::spawn(move || match stream.peer_addr() {
            Ok(addr) => {
                info!(
                    "Handle incoming connection. dispatch worker {} actives: {}",
                    addr,
                    dispatcher.active_workers.load(Ordering::SeqCst)
                );
                if let Err(err) = Worker::dispatch(addr, stream) {
                    eprintln!("{:#?}", err);
                }
                dispatcher.active_workers.fetch_sub(1, Ordering::Relaxed);
            }
            Err(err) => error!("Could not get peer address: {}", err),
        });
    }
}

struct Worker {
    peer: String,
    operator: Operator,
}

impl Worker {
    fn dispatch(addr: SocketAddr, stream: TcpStream) -> Result<()> {
        Worker::new(addr, stream).run()
    }
    fn new(addr: SocketAddr, stream: TcpStream) -> Self {
        Self {
            peer: format!("{}", addr),
            operator: Operator::new(stream),
        }
    }
    fn run(&mut self) -> Result<()> {
        self.ready().and_then(|_| self.ping_pon())?;
        debug!("{} Successfully ping to client", self);
        loop {
            let cmd = self
                .operator
                .read()
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
                Command::RequestUpstream => {
                    info!("{} Handle upstream", self);
                    self.handle_upstream()?;
                    info!("{} Successfully handle upstream", self);
                }
                Command::Close => return Ok(()),
                _ => return Err(anyhow!("Unexpected command {:?}", cmd)),
            }
        }
    }

    fn ready(&mut self) -> Result<()> {
        self.operator.write(Command::Ready)
    }

    fn ping_pon(&mut self) -> Result<()> {
        self.operator.ping_read_then_write()
    }

    fn handle_downstream(&mut self) -> Result<()> {
        let timeout = self.operator.read_duration()?;
        debug!("{} Timeout: {:?}", self, timeout);
        let write_bytes = self.operator.write_loop(timeout)?;
        debug!("{} Write {}", self, util::format_bytes(write_bytes));
        Ok(())
    }

    fn handle_upstream(&mut self) -> Result<()> {
        // consume
        let timeout = self.operator.read_duration()?;
        debug!("{} Timeout: {:?}", self, timeout);
        let read_bytes = self.operator.read_loop()?;
        debug!("{} Read {}", self, util::format_bytes(read_bytes));
        Ok(())
    }
}

impl fmt::Display for Worker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(Worker:{}) =>", self.peer.as_str())
    }
}
