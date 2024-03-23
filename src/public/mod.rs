
mod routes;
mod templates;

use actix_web::{web, App, HttpServer};
use actix_files as actix_fs;

use crate::config::definition::Config;

pub(crate) async fn start_actix(config: &Config) -> anyhow::Result<()> {
    let bindip = format!("{}:{}", config.webserver.host, config.webserver.port);

    log::info!("Serving an http server at http://{bindip}");
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(routes::index)) // index.html
            .service(actix_fs::Files::new("/static", "./static").index_file("index.html")) // static directoryh
    })

    .bind(bindip)?
    .run()
    .await?;

    Ok(())
}
