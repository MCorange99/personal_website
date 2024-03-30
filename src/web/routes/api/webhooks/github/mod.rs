pub mod types;
pub mod events;

use std::{borrow::BorrowMut, sync::Mutex};

use actix_web::{http::header, web::{self, Bytes, Data}, HttpRequest, HttpResponse, Responder, Result, Scope};

use crate::database::{models, Database};

pub async fn handler(req: HttpRequest, body: Bytes, db: Data<Mutex<Database>>) -> Result<impl Responder> {
    let Some(auth) = req.headers().get(header::AUTHORIZATION) else {
        return Ok(HttpResponse::Unauthorized());
    };

    let Ok(token) = auth.to_str() else {
        return Ok(HttpResponse::Unauthorized());
    };

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

    let Some(event_type) = req.headers().get("X-GitHub-Event") else {
        return Ok(HttpResponse::BadRequest());
    };

    let Ok(event_type) = event_type.to_str() else {
        return Ok(HttpResponse::BadRequest());
    };

    let Ok(json) = String::from_utf8(body.to_vec()) else {
        return Ok(HttpResponse::BadRequest());
    };

    let Ok(event) = types::Event::from_raw_json(event_type, json.clone()) else {
        return Ok(HttpResponse::BadRequest());
    };

    match event {
        types::Event::Release(body) => events::release_handler(db, token, body, json).await,
        _ => {
            dbg!(json);
            Ok(HttpResponse::Ok())
        }
    }

}
