use actix_web::Scope;

pub mod github;


pub fn get_scope() -> Scope {
    Scope::new("/wh")
        .service(
            github::get_scope()
        )
}