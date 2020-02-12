use log::error;
use netspeed::{cli, logger, Client, Server};
use std::env;

fn run() -> Result<(), anyhow::Error> {
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
        error!("{:?}", err);
        std::process::exit(1)
    }
}
