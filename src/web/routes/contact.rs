
use std::sync::Mutex;

use actix_web_lab::respond::Html;
use actix_web::{web::Data, Responder, Result};
use askama::Template;

use crate::{database::Database, web::templates::ContactTemplate};



pub async fn contact(_: Data<Mutex<Database>>) -> Result<impl Responder> {

    let html = ContactTemplate {
        title: String::from("MCorange - Contact me!"),
    }.render().expect("Failed to render contacts.html");

    Ok(Html(html))
}