use std::{borrow::BorrowMut, sync::Mutex};

use actix_web::{http::header, web::{self, Data}, HttpRequest, HttpResponse, HttpResponseBuilder, Responder, Result, Scope};
use serde_json::Value;

use crate::database::{models::{self, tokens::Token}, Database};

pub fn get_scope() -> Scope {
    Scope::new("/github")
        .service(
            web::resource("/push")
                .to(handler)
        )
}

pub async fn handler(req: HttpRequest, body: web::Json<Value>, db: Data<Mutex<Database>>) -> Result<impl Responder> {
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

    match event_type {
        "release" => {
            release_handler(db, token, body).await
        }

        _ => Ok(HttpResponse::Ok())
    }



}



pub async fn release_handler(db: Data<Mutex<Database>>, token: Token, body: web::Json<Value>) -> Result<HttpResponseBuilder> {
    let Some(release) = body.get("release") else {
        return Ok(HttpResponse::BadRequest());
    };

    let Some(origin_url) = release.get("repository") else {
        return Ok(HttpResponse::BadRequest());
    };


    models::posts::Post::create_new(
        db.lock().unwrap().borrow_mut(),
        title,
        descr,
        img_url,
        origin_url,
        orignal_request
    );




    Ok(HttpResponse::Ok())
}