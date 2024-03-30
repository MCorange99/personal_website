pub mod api;
pub mod contact;
pub mod projects;
pub mod redirect;

use std::sync::Mutex;

use actix_web_lab::respond::Html;
use actix_web::{web::Data, Responder, Result};
use askama::Template;

use crate::{database::Database, web::templates::IndexTemplate};



// NOTE: Not usefull to have database here but just so u know how
pub async fn index(db: Data<Mutex<Database>>) -> Result<impl Responder> {

    let posts = match crate::database::models::posts::Post::get_last_n(&mut db.lock().unwrap(), 10).await {
        Ok(p) => p,
        _ => vec![]
    };

    let html = IndexTemplate {
        posts,
        title: String::from("MCorange - The website :3"),
    }.render().expect("Failed to render index.html");

    Ok(Html(html))
}