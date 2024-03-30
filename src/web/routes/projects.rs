
use std::sync::Mutex;

use actix_web_lab::respond::Html;
use actix_web::{web::Data, Responder, Result};
use askama::Template;

use crate::{database::Database, web::templates::ProjectTemplate};



pub async fn projects(_: Data<Mutex<Database>>) -> Result<impl Responder> {

    let html = ProjectTemplate {
        title: String::from("MCorange - My projects :O"),
    }.render().expect("Failed to render projects.html");

    Ok(Html(html))
}