use simplelog::*;



pub fn init_logger() {
    // TODO: figure out what these do
    let config = ConfigBuilder::new()
        .build();
    


    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Debug, config, TerminalMode::Mixed, ColorChoice::Auto),
            // TODO: Set up loggin to file
            // WriteLogger::new(LevelFilter::Info, Config::default(), File::create("my_rust_binary.log").unwrap()),
        ]
    ).unwrap();
}