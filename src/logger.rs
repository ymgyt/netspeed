use env_logger::fmt::Color;
use log::{Level, LevelFilter};
use std::io::Write;

pub fn init(verbose: u64, is_server: bool) {
    let mut builder = env_logger::Builder::new();

    builder.format(move |buf, record| {
        let level_color = match record.level() {
            Level::Trace => Color::White,
            Level::Debug => Color::Blue,
            Level::Info => Color::Green,
            Level::Warn => Color::Yellow,
            Level::Error => Color::Red,
        };
        let mut level_style = buf.style();
        level_style.set_color(level_color);

        if is_server {
            writeln!(
                buf,
                "{level:5} {time} {file:>10}:{line:<4} {args}",
                level = level_style.value(record.level()),
                time = chrono::Utc::now().to_rfc3339(),
                file = &record.file().unwrap_or("____unknown")[4..],
                line = record.line().unwrap_or(0),
                args = record.args(),
            )
        } else {
            writeln!(
                buf,
                "{level:5} {args}",
                level = level_style.value(record.level()),
                args = record.args(),
            )
        }
    });

    builder.filter(
        None,
        match verbose {
            0 => LevelFilter::Info,
            1 => LevelFilter::Debug,
            _ => LevelFilter::Trace,
        },
    );
    builder.write_style(env_logger::WriteStyle::Auto);

    builder.init();
}
