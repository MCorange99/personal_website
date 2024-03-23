
mod routes;
mod templates;

use actix_web::{web, App, HttpServer};
use actix_files as actix_fs;

use crate::cli::CliArgs;

pub(crate) async fn start_actix(cli: &CliArgs) -> anyhow::Result<()> {
    let bindip = format!("{}:{}", cli.host, cli.port);

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
