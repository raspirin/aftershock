use nid::Nanoid;
use std::collections::HashMap;

pub struct PostIndex {
    pub title: String,
    pub time: String,
    pub tags: Vec<String>,
    pub address: String,
}

pub struct PostsIndex {
    pub map: HashMap<Nanoid, PostIndex>,
}
