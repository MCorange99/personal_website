use simplelog::*;

use crate::cli::CliArgs;



pub fn init_logger(cli: &CliArgs) {
    // TODO: figure out what these do
    let config = ConfigBuilder::new()
        .build();

    let level = if cli.debug {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };


    CombinedLogger::init(
        vec![
            TermLogger::new(level, config, TerminalMode::Mixed, ColorChoice::Auto),
            // TODO: Set up loggin to file
            // WriteLogger::new(LevelFilter::Info, Config::default(), File::create("my_rust_binary.log").unwrap()),
        ]
    ).unwrap();
}