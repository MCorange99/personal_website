use actix_web::{web, App, HttpServer};

mod public;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(public::index))
    })

    .bind("0.0.0.0:8080")?
    .run()
    .await
}