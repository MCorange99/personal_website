use actix_web::{web::{self, redirect}, HttpResponse, Scope};

use crate::config::definition::Config;

pub fn get_scope(config: &Config) -> Scope {
    let mut s = Scope::new("/r")
        .route("/", web::get().to(HttpResponse::Ok));

    for r in config.redirect.clone() {
        s = s.service(redirect(r.from, r.to));
    }
        // .service(
        //     webhooks::get_scope()
        // )
    s
}