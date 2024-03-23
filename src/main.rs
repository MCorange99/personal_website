use clap::Parser;

mod public;
mod logger;
mod cli;
mod config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cli = cli::CliArgs::parse();
    logger::init_logger(&cli);

    let config = match config::ConfigManager::parse_and_join(&cli) {
        Ok(r) => r,
        Err(e) => {
            log::error!("Failed to parse configs: {e}");
            return Ok(());
        }
    };


    if let Err(e) = public::start_actix(config.get_ref()).await {
        log::error!("Actix had an error: {e}");
    }
    Ok(())
}