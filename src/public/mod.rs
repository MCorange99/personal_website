use actix_web_lab::respond::Html;
use actix_web::{Result, Responder};
use askama::Template;


mod templates;

pub async fn index() -> Result<impl Responder> {
    let html = templates::IndexTemplate {
        placeholder: "hewwo world"
    }.render().expect("Failed to render index.html");

    Ok(Html(html))
}

