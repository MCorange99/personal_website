use askama::Template;

#[derive(Debug, Clone, Template)]
#[template(path = "index.html")]
pub struct IndexTemplate{
    pub title: String,
    pub posts: Vec<IndexTemplatePost>
}

#[derive(Debug, Clone)]
pub struct IndexTemplatePost {
    pub image: String,
    pub title: String,
    pub description: String,
    pub url: String
}
