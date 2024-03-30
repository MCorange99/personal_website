mod webhooks;

use actix_web::{web, Route, Scope};




pub fn get_scope() -> Scope {
    Scope::new("/api")
        .service(
            webhooks::get_scope()
        )

}