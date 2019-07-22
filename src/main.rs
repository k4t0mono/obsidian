#[macro_use] extern crate log;
extern crate clap;
extern crate simplelog;
mod config;

use config::Config;

fn set_logger(level: u64) {
    use simplelog::*;

    let log_level: LevelFilter = match level {
        0 => LevelFilter::Off,
        1 => LevelFilter::Error,
        2 => LevelFilter::Warn,
        3 => LevelFilter::Info,
        4 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    };

    TermLogger::init(log_level, Config::default(), TerminalMode::Stderr).unwrap();
}


fn main() {
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");
    const ABOUT: &'static str = env!("CARGO_PKG_DESCRIPTION");

    let matches = clap::App::new("Obsidian")
        .version(VERSION)
        .author(AUTHORS)
        .about(ABOUT)
        .arg(clap::Arg::with_name("verbose")
            .short("v")
            .multiple(true)
            .help("Sets the level of verbosity"))
        .get_matches();

    set_logger(matches.occurrences_of("verbose"));

    let c = Config::loads();
    println!("{:?}", c);
}
