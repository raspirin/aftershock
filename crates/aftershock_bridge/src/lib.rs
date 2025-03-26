use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
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

#[derive(Serialize, Deserialize, Clone)]
pub struct PostMeta {
    pub uid: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub title: String,
    pub tags: Vec<String>,
}

impl From<Post> for PostMeta {
    fn from(value: Post) -> Self {
        Self {
            uid: value.uid,
            created_at: value.created_at,
            updated_at: value.updated_at,
            title: value.title,
            tags: value.tags,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct UpdatePost {
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub body: Option<String>,
    #[serde(default)]
    pub published: Option<bool>,
}
