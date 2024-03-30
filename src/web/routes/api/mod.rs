pub mod webhooks;

use actix_web::Scope;




pub fn get_scope() -> Scope {
    Scope::new("/api")
        .service(
            webhooks::get_scope()
        )

}