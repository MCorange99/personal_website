use askama::Template;

use crate::database::models::posts::Post;

#[derive(Debug, Clone, Template)]
#[template(path = "index.html")]
pub struct IndexTemplate{
    pub title: String,
    pub posts: Vec<Post>
}


