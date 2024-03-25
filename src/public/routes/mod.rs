use std::sync::Mutex;

use actix_web_lab::respond::Html;
use actix_web::{web::Data, Responder, Result};
use askama::Template;

use crate::{database::Database, public::templates::IndexTemplate};


// NOTE: Not usefull to have database here but just so u know how
pub async fn index(_: Data<Mutex<Database>>) -> Result<impl Responder> {
    let html = IndexTemplate {
        placeholder: "hewwo world"
    }.render().expect("Failed to render index.html");

    Ok(Html(html))
}