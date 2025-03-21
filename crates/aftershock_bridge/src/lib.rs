use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Post {
    pub id: i32,
    pub created_at: i64,
    pub updated_at: i64,
    pub title: String,
    pub body: String,
}
