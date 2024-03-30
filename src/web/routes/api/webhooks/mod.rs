use actix_web::{web, Scope};

mod github;


pub fn get_scope() -> Scope {
    Scope::new("/wh")
        .service(
            github::get_scope()
        )
}