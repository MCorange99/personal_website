use std::{borrow::BorrowMut, sync::Mutex};

use actix_web::{web::Data, HttpResponse, HttpResponseBuilder, Result};

use crate::database::{models::{self, tokens::Token}, Database};

use super::types::ReleaseEvent;

pub async fn release_handler(db: Data<Mutex<Database>>, token: Token, body: ReleaseEvent, raw_body: String,) -> Result<HttpResponseBuilder> {

    let title = format!("{} has been released on {}!", body.release.tag_name, body.repository.full_name);

    dbg!(body);
    body.hook
    // models::posts::Post::create_new(
    //     db.lock().unwrap().borrow_mut(),
    //     title,
    //     descr,
    //     img_url,
    //     origin_url,
    //     orignal_request
    // );




    Ok(HttpResponse::Ok())
}