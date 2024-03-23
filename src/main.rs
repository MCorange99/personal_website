mod public;
mod logger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    logger::init_logger();

    if let Err(e) = public::start_actix().await {
        log::error!("Actix had an error: {e}");
    }
    Ok(())
}