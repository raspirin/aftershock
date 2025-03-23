use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Post {
    pub uid: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub title: String,
    pub tags: Vec<String>,
    pub body: String,
}

#[derive(Serialize, Deserialize)]
pub struct NewPost {
    pub title: String,
    pub body: String,
    pub tags: Vec<String>,
    pub published: bool,
}

#[derive(Serialize, Deserialize)]
pub struct PostMeta {
    pub uid: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub title: String,
    pub tags: Vec<String>,
}
