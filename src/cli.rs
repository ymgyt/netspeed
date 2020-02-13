use clap::{App, AppSettings, Arg, ArgMatches};
use std::env;

pub struct ArgParser {}

const VERSION: &str = env!("CARGO_PKG_VERSION");

impl ArgParser {
    pub fn parse(args: env::ArgsOs) -> ArgMatches<'static> {
        App::new("netspeed")
            .version(VERSION)
            .about("measure tcp throughput")
            .setting(AppSettings::ColorAuto)
            .setting(AppSettings::ColoredHelp)
            .setting(AppSettings::VersionlessSubcommands)
            .arg(
                Arg::with_name("verbose")
                    .long("verbose")
                    .short("v")
                    .multiple(true)
                    .help("logging verbose"),
            )
            .arg(
                Arg::with_name("address")
                    .long("addr")
                    .alias("address")
                    .short("a")
                    .help("tcp listening/connection address")
                    .takes_value(true)
                    .default_value("localhost:5555")
                    .global(true),
            )
            .arg(
                Arg::with_name("duration")
                    .long("duration")
                    .alias("duration-seconds")
                    .help("speed test duration seconds")
                    .takes_value(true)
                    .default_value("3")
                    .value_name("SECONDS"),
            )
            .subcommand(
                App::new("server")
                    .setting(AppSettings::ColorAuto)
                    .setting(AppSettings::ColoredHelp)
                    .arg(
                        Arg::with_name("run")
                            .index(1)
                            .required(true)
                            .help("running server"),
                    ),
            )
            .get_matches_from(args)
    }
}
