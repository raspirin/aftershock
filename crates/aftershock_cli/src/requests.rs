use ::reqwest::header::CONTENT_TYPE;
use reqwest::blocking as reqwest;

static API_BASE: &str = "http://127.0.0.1:3030/api/v1";

pub fn add(path: String) -> String {
    let url = format!("{API_BASE}/posts");
    let input = std::fs::read_to_string(&path).unwrap();
    let output = crate::parser::parse(&input);
    let new_post: aftershock_bridge::NewPost = output.into();
    let new_post = serde_json::to_string(&new_post).unwrap();
    let client = reqwest::Client::new();
    let body = client
        .post(url)
        .header(CONTENT_TYPE, "application/json")
        .body(new_post)
        .send()
        .unwrap()
        .json::<aftershock_bridge::Post>()
        // .text()
        .unwrap();
    serde_json::to_string_pretty(&body).unwrap()
    // body
}

pub fn list() -> String {
    let url = format!("{API_BASE}/posts/meta");
    let body = reqwest::get(url)
        .unwrap()
        .json::<Vec<aftershock_bridge::PostMeta>>()
        .unwrap();
    serde_json::to_string_pretty(&body).unwrap()
}

pub fn view(id: String) -> String {
    let url = format!("{API_BASE}/posts/uid/{id}");
    let body = reqwest::get(url);
    serde_json::to_string_pretty(
        &body
            .ok()
            .map(|body| body.json::<aftershock_bridge::Post>().unwrap()),
    )
    .unwrap()
}

pub fn delete(id: String) -> String {
    let url = format!("{API_BASE}/posts/uid/{id}");
    let client = reqwest::Client::new();
    let body = client
        .delete(url)
        .send()
        .unwrap()
        .json::<aftershock_bridge::Post>()
        .unwrap();
    serde_json::to_string_pretty(&body).unwrap()
}
