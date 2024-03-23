
mod routes;
mod templates;

use actix_web::{web, App, HttpServer};

pub(crate) async fn start_actix() -> anyhow::Result<()> {
    log::info!("Serving an http server at https://0.0.0.0:8080");
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(routes::index))
    })

    .bind("0.0.0.0:8080")?
    .run()
    .await?;

    Ok(())
}
