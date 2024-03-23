use actix_web_lab::respond::Html;
use actix_web::{Result, Responder};
use askama::Template;

use crate::public::templates::IndexTemplate;

pub async fn index() -> Result<impl Responder> {
    let html = IndexTemplate {
        placeholder: "hewwo world"
    }.render().expect("Failed to render index.html");

    Ok(Html(html))
}