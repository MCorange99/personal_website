use actix_web::{web, HttpResponse, Scope};

pub mod github;


pub fn get_scope() -> Scope {
    Scope::new("/wh")
        .route("/", web::get().to(HttpResponse::Ok))
        .service(
            github::get_scope()
        )
}