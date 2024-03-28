
use log::LevelFilter;

use crate::cli::CliArgs;



pub fn init_logger(cli: &CliArgs) {

    let level = if cli.debug {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };

    env_logger::builder()
        .filter_level(LevelFilter::Off)
        .parse_env("RUST_LOG")
        .filter_module("website", level)
        .init();
    // env_logger::Builder::from_env(Env::default().default_filter_or("warn")).init();
}