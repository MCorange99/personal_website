mod webhooks;

use actix_web::{web, HttpResponse, Scope};




pub fn get_scope() -> Scope {
    Scope::new("/api")
        .route("/", web::get().to(HttpResponse::Ok))
        .service(
            webhooks::get_scope()
        )

}