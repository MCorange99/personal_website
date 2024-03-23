use actix_web_lab::respond::Html;
use actix_web::{web, App, HttpServer, Result, Responder};

mod public;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {
        App::new()
            .route("/", public::get().to(index))
    })

    .bind("0.0.0.0:8080")?
    .run()
    .await
}
