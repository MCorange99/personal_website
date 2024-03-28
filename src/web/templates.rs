use askama::Template;

#[derive(Debug, Clone, Copy, Template)]
#[template(path = "index.html")]
pub struct IndexTemplate<'a> {
    pub placeholder: &'a str, 
}