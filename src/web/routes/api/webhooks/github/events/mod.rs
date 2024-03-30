use std::{borrow::BorrowMut, sync::Mutex};

use actix_web::{web::Data, HttpResponse, HttpResponseBuilder, Result};

use crate::database::{models::{self, tokens::Token}, Database};

use super::types::ReleaseEvent;

pub async fn release_handler(db: Data<Mutex<Database>>, token: Token, body: ReleaseEvent, raw_body: String) -> Result<HttpResponseBuilder> {

    if body.action != "released" {
        return Ok(HttpResponse::Ok());
    }


    let title = format!("(New release {}:{}) {}", body.repository.full_name, body.release.tag_name, body.release.name.unwrap_or("No title provided".into()));
    let origin_url = body.repository.html_url.clone();
    let descr = body.release.body.unwrap_or("No body provided".into());
    let img_url = body.repository.owner.avatar_url.clone();

    let res = models::posts::Post::create_new(
        db.lock().unwrap().borrow_mut(),
        title,
        descr,
        img_url,
        origin_url,
        raw_body
    ).await;

    if let Err(e) = res {
        log::error!("{e}");
        return Ok(HttpResponse::InternalServerError());
    }


    Ok(HttpResponse::Ok())
}