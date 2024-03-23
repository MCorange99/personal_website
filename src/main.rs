use actix_web_lab::respond::Html;
use askama::Template;
use actix_web::{web, App, HttpServer, Result, Responder};

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    placeholder: &'a str, 
}

async fn index() -> Result<impl Responder> {
    let html = IndexTemplate {
        placeholder: "hewwo world"
    }.render().expect("Failed to render index.html");

    Ok(Html(html))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
    })

    .bind("0.0.0.0:8080")?
    .run()
    .await
}
