use clap::Parser;

mod public;
mod logger;
mod cli;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cli = cli::CliArgs::parse();
    logger::init_logger(&cli);


    if let Err(e) = public::start_actix(&cli).await {
        log::error!("Actix had an error: {e}");
    }
    Ok(())
}