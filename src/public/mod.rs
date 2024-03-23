
mod routes;
mod templates;

use actix_web::{web, App, HttpServer};
use actix_files as actix_fs;

pub(crate) async fn start_actix() -> anyhow::Result<()> {
    log::info!("Serving an http server at http://0.0.0.0:8080");
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(routes::index)) // index.html
            .service(actix_fs::Files::new("/static", "./static").index_file("index.html")) // static directoryh
    })

    .bind("0.0.0.0:8080")?
    .run()
    .await?;

    Ok(())
}
