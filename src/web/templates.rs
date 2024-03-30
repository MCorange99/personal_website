use askama::Template;

use crate::database::models::posts::Post;

#[derive(Debug, Clone, Template)]
#[template(path = "index.html")]
pub struct IndexTemplate{
    pub title: String,
    pub posts: Vec<Post>
}


#[derive(Debug, Clone, Template)]
#[template(path = "projects.html")]
pub struct ProjectTemplate{
    pub title: String,
}

#[derive(Debug, Clone, Template)]
#[template(path = "contact.html")]
pub struct ContactTemplate{
    pub title: String,
}


