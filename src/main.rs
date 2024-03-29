use clap::Parser;

mod web;
mod logger;
mod cli;
mod config;
mod database;

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

    let Ok(database) = database::Database::new(config.get_ref()).await else {return Ok(())};

    if let Err(e) = web::start_actix(config.get_ref(), database).await {
        log::error!("Actix had an error: {e}");
    }
    Ok(())
}