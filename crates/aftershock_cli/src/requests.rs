use std::sync::LazyLock;

use ::reqwest::{IntoUrl, blocking::Response, header::CONTENT_TYPE};
use reqwest::blocking as reqwest;

static API_BASE: &str = "http://127.0.0.1:3030/api/v1";
static CLIENT: LazyLock<reqwest::Client> = LazyLock::new(|| reqwest::Client::new());

fn get<U: IntoUrl>(url: U) -> Result<Response, ::reqwest::Error> {
    CLIENT.get(url).send()
}

pub fn add(path: String) -> String {
    let url = format!("{API_BASE}/posts");
    let input = std::fs::read_to_string(&path).unwrap();
    let output = crate::parser::parse(&input);
    let new_post: aftershock_bridge::NewPost = output.into();
    let new_post = serde_json::to_string(&new_post).unwrap();
    let client = &CLIENT;
    let body = client
        .post(url)
        .header(CONTENT_TYPE, "application/json")
        .body(new_post)
        .send()
        .unwrap()
        .json::<aftershock_bridge::Post>()
        .unwrap();
    serde_json::to_string_pretty(&body).unwrap()
}

pub fn list() -> String {
    let url = format!("{API_BASE}/posts/all-meta");
    let body = get(url)
        .unwrap()
        .json::<Vec<aftershock_bridge::PostMeta>>()
        .unwrap();
    serde_json::to_string_pretty(&body).unwrap()
}

pub fn view(id: String) -> String {
    let url = format!("{API_BASE}/posts/uid/{id}");
    let body = get(url);
    serde_json::to_string_pretty(
        &body
            .ok()
            .map(|body| body.json::<aftershock_bridge::Post>().unwrap()),
    )
    .unwrap()
}

pub fn delete(id: String) -> String {
    let url = format!("{API_BASE}/posts/uid/{id}");
    let client = &CLIENT;
    let body = client
        .delete(url)
        .send()
        .unwrap()
        .json::<aftershock_bridge::Post>()
        .unwrap();
    serde_json::to_string_pretty(&body).unwrap()
}

pub fn publish(id: String) -> String {
    let url = format!("{API_BASE}/posts/uid/{id}");
    let body = aftershock_bridge::UpdatePost {
        title: None,
        body: None,
        published: Some(true),
    };
    let body = serde_json::to_string(&body).unwrap();
    let post = CLIENT
        .put(url)
        .header(CONTENT_TYPE, "application/json")
        .body(body)
        .send()
        .unwrap()
        .json::<aftershock_bridge::Post>()
        .unwrap();
    serde_json::to_string_pretty(&post).unwrap()
}
