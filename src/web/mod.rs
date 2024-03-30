pub mod routes;
pub mod templates;

use std::sync::Mutex;

use actix_web::{web, App, HttpServer};
use actix_files as actix_fs;

use crate::{config::definition::Config, database::Database};

pub(crate) async fn start_actix(config: &Config, database: Database) -> anyhow::Result<()> {
    let bindip = format!("{}:{}", config.webserver.host, config.webserver.port);

    log::info!("Serving an http server at http://{bindip}");
    HttpServer::new(move || {
        App::new()
            .app_data(actix_web::web::Data::new(Mutex::new(database.clone())))
            .route("/", web::get().to(routes::index)) // index.html
            .service(routes::api::get_scope())
            .service(actix_fs::Files::new("/static", "./static").index_file("index.html")) // static directory
            .service(web::redirect("/favicon.ico", "/static/favicon.ico")) //? special redirect for favicon
    })
    .bind(bindip)?
    .run()
    .await?;

    Ok(())
}
