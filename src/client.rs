use crate::{command::Operator, Result};
use anyhow::Context;
use log::{debug, info};
use std::{
    fmt,
    io::{self, Write},
    net::{TcpStream, ToSocketAddrs},
    str::FromStr,
    time::Duration,
};

#[derive(Default, Debug)]
struct Throughput {
    bytes: u64,
    duration: Duration,
}

#[derive(Default, Debug)]
struct NetworkSpec {
    downstream: Throughput,
    upstream: Throughput,
}

pub struct Client {
    operator: Operator,
    spec: NetworkSpec,
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
            spec: NetworkSpec::default(),
        })
    }

    pub fn duration(mut self, duration: Option<&str>) -> Self {
        let duration =
            Duration::from_secs(u64::from_str(duration.unwrap_or("3").as_ref()).unwrap());
        self.spec.downstream.duration = duration;
        self.spec.upstream.duration = duration;
        self
    }

    pub fn run(mut self) -> Result<()> {
        self.ping_pon()
            .and(self.downstream())
            .and(self.upstream())
            .and(self.print_result(io::stdout()))
    }

    fn ping_pon(&mut self) -> Result<()> {
        self.operator.ping_write_then_read().map(|r| {
            debug!("Successfully ping to remote server");
            r
        })
    }

    fn downstream(&mut self) -> Result<()> {
        debug!(
            "Request downstream duration: {} seconds",
            self.spec.downstream.duration.as_secs()
        );
        self.operator
            .request_downstream(self.spec.downstream.duration)?;
        self.spec.downstream.bytes = self.operator.read_loop()?;
        Ok(())
    }

    fn upstream(&mut self) -> Result<()> {
        debug!(
            "Request upstream duration: {} seconds",
            self.spec.upstream.duration.as_secs()
        );
        self.operator
            .request_upstream(self.spec.upstream.duration)?;
        self.spec.upstream.bytes = self.operator.write_loop(self.spec.upstream.duration)?;
        Ok(())
    }

    fn print_result<W: Write>(&mut self, mut writer: W) -> Result<()> {
        writeln!(
            writer,
            "Downstream: {}",
            self.format_throughput(&self.spec.downstream)
        )
        .and(writeln!(
            writer,
            "  Upstream: {}",
            self.format_throughput(&self.spec.upstream)
        ))
        .map_err(anyhow::Error::from)
    }

    fn format_throughput(&self, throughput: &Throughput) -> String {
        use crate::util::*;
        format_bps(to_bps(throughput.bytes, throughput.duration))
    }
}
