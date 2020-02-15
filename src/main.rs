use log::error;
use netspeed::{cli, logger, Client, Server, DEFAULT_MAX_THREADS};
use std::env;

fn run() -> Result<(), anyhow::Error> {
    let args = cli::ArgParser::parse(env::args_os());

    logger::init(args.occurrences_of("verbose"), args.is_present("server"));

    if let Some(sub) = args.subcommand_matches("server") {
        let server = Server::new(
            sub.value_of("address").unwrap(),
            sub.value_of("max-threads")
                .unwrap()
                .parse()
                .unwrap_or(DEFAULT_MAX_THREADS),
        )?;
        server.run()
    } else {
        let client =
            Client::new(args.value_of("address").unwrap())?.duration(args.value_of("duration"));
        client.run()
    }
}

fn main() {
    if let Err(err) = run() {
        error!("{:?}", err);
        std::process::exit(1)
    }
}
