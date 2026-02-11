use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Post {
    pub uid: String,
    pub kind: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub title: String,
    pub tags: Vec<String>,
    pub body: String,
    pub summary: Option<String>,
    pub published: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewPost {
    pub title: String,
    pub kind: String,
    pub body: String,
    pub tags: Vec<String>,
    pub published: bool,
    pub summary: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PostMeta {
    pub uid: String,
    pub kind: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub title: String,
    pub tags: Vec<String>,
    pub summary: Option<String>,
    pub published: bool,
}

impl From<Post> for PostMeta {
    fn from(value: Post) -> Self {
        Self {
            uid: value.uid,
            kind: value.kind,
            created_at: value.created_at,
            updated_at: value.updated_at,
            title: value.title,
            tags: value.tags,
            summary: value.summary,
            published: value.published,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdatePost {
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub body: Option<String>,
    #[serde(default)]
    pub published: Option<bool>,
}
