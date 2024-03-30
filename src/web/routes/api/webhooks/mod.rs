use actix_web::{web, HttpResponse, Scope};

pub mod github;


pub fn get_scope() -> Scope {
    Scope::new("/wh")
        .route("/", web::get().to(HttpResponse::Ok))
        .route("/github", web::get().to(HttpResponse::Ok))
        .route("/github", web::post().to(github::handler))
}