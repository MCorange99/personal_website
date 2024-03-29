use std::sync::Mutex;

use actix_web_lab::respond::Html;
use actix_web::{web::Data, Responder, Result};
use askama::Template;

use crate::{database::Database, web::templates::IndexTemplate};

use super::templates::IndexTemplatePost;


// NOTE: Not usefull to have database here but just so u know how
pub async fn index(_: Data<Mutex<Database>>) -> Result<impl Responder> {
    let html = IndexTemplate {
        posts: vec![
            IndexTemplatePost {
                image: String::from("/static/assets/uwu.jpg"),
                title: String::from("Cutie"),
                description: String::from("Yes you are ;3"),
                url: String::from("https://djc.github.io/askama/template_expansion.html")
            }
        ],
        title: String::from("Very cool mcoranges website :3"),
    }.render().expect("Failed to render index.html");

    Ok(Html(html))
}
/*
<div class="post">
    <img src="/static/assets/uwu.jpg" alt="post img">
    <span>
        <h2>Title text</h3>
        <p>Description text</p>
    </span>
</div>
*/