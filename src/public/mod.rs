
mod routes;
mod templates;

use actix_web::{web, App, HttpServer};

use crate::cli::CliArgs;

pub(crate) async fn start_actix(cli: &CliArgs) -> anyhow::Result<()> {
    let bindip = format!("{}:{}", cli.host, cli.port);

    log::info!("Serving an http server at http://{bindip}");
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(routes::index))
    })

    .bind(bindip)?
    .run()
    .await?;

    Ok(())
}
