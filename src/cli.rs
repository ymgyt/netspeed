use clap::{App, AppSettings, Arg, ArgMatches};
use std::env;

pub struct ArgParser {}

const VERSION: &str = env!("CARGO_PKG_VERSION");

impl ArgParser {
    pub fn parse(args: env::ArgsOs) -> ArgMatches<'static> {
        App::new("netspeed")
            .version(VERSION)
            .about("Measure tcp throughput")
            .global_setting(AppSettings::ColorAuto)
            .global_setting(AppSettings::ColoredHelp)
            .global_setting(AppSettings::VersionlessSubcommands)
            .arg(
                Arg::with_name("verbose")
                    .long("verbose")
                    .short("v")
                    .multiple(true)
                    .global(true)
                    .help("Logging verbose"),
            )
            .arg(
                Arg::with_name("address")
                    .long("addr")
                    .alias("address")
                    .short("a")
                    .help("Remote server address")
                    .takes_value(true)
                    .default_value("netspeed.ymgyt.io:5555"),
            )
            .arg(
                Arg::with_name("duration")
                    .long("duration")
                    .alias("duration-seconds")
                    .help("Speed test duration seconds(max: 10)")
                    .takes_value(true)
                    .default_value("3")
                    .validator(|s| {
                        let n = s.parse::<u8>().map_err(|err| format!("{}", err))?;
                        if n > 10 {
                            Err("Max duration exceeded (max: 10)".to_owned())
                        } else {
                            Ok(())
                        }
                    })
                    .value_name("SECONDS"),
            )
            .subcommand(
                App::new("server")
                    .about("Server mode")
                    .arg(
                        Arg::with_name("run")
                            .index(1)
                            .required(true)
                            .help("Running server"),
                    )
                    .arg(
                        Arg::with_name("address")
                            .long("addr")
                            .alias("address")
                            .short("a")
                            .help("Listening address")
                            .takes_value(true)
                            .default_value("localhost:5555"),
                    )
                    .arg(
                        Arg::with_name("max-threads")
                            .long("max-threads")
                            .alias("max-workers")
                            .help("Max concurrent threads/workers")
                            .takes_value(true)
                            .default_value("100")
                            .value_name("NUMBER"),
                    ),
            )
            .get_matches_from(args)
    }
}
