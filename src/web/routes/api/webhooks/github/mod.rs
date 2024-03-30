pub mod types;
pub mod events;

use std::{borrow::BorrowMut, sync::Mutex};

use actix_web::{web::{self, Bytes, Data}, HttpRequest, HttpResponse, Responder, Result};

use crate::database::{models::{self, Permissions}, Database};

pub async fn handler(req: HttpRequest, token: web::Path<String>, body: Bytes, db: Data<Mutex<Database>>) -> Result<impl Responder> {


    let token = models::tokens::Token::get_by_token(
            db.lock().unwrap().borrow_mut(),
            token.to_string()
        ).await;

    let Ok(token) = token else {
        return Ok(HttpResponse::Unauthorized());
    };

    let Some(token) = token else {
        return Ok(HttpResponse::Unauthorized());
    };

    if !token.permissions.contains(Permissions::MAKE_POST) {
        return Ok(HttpResponse::Unauthorized());
    }

    let Some(event_type) = req.headers().get("X-GitHub-Event") else {
        log::debug!("No X-GitHub-Event header");
        return Ok(HttpResponse::BadRequest());
    };

    let Ok(event_type) = event_type.to_str() else {
        log::debug!("Bad X-GitHub-Event header");
        return Ok(HttpResponse::BadRequest());
    };

    let Ok(json) = String::from_utf8(body.to_vec()) else {
        log::debug!("Bad request body");
        return Ok(HttpResponse::BadRequest());
    };

    let event = match types::Event::from_raw_json(event_type, json.clone()) {
        Ok(e) => e,
        Err(e) => {
            log::debug!("Bad request body json: {e}");
            // log::debug!("{json}");
            return Ok(HttpResponse::BadRequest());
        }
    };

    match event {
        types::Event::Release(body) => events::release_handler(db, body, json).await,
        _ => {
            // dbg!(json);
            Ok(HttpResponse::Ok())
        }
    }

}
