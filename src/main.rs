use log::error;
use netspeed::{cli, logger, Client, Server};
use std::env;

fn run() -> Result<(), failure::Error> {
    let args = cli::ArgParser::parse(env::args_os());

    logger::init(args.occurrences_of("verbose"));

    if let Some(sub) = args.subcommand_matches("server") {
        let server = Server::new(sub.value_of("address").unwrap())?;
        server.run()
    } else {
        let client = Client::new(args.value_of("address").unwrap())?;
        client.run()
    }
}

fn main() {
    if let Err(err) = run() {
        for cause in err.iter_chain() {
            if let Some(ioerr) = cause.downcast_ref::<std::io::Error>() {
                error!("IOError: [{:?}] {}", ioerr.kind(), ioerr);
            } else {
                error!("{}", cause);
            }
            if let Some(bt) = cause.backtrace() {
                if !bt.is_empty() {
                    error!("{}", bt);
                }
            }
        }
        std::process::exit(1)
    }
}
