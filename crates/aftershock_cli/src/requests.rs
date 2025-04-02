use std::sync::LazyLock;

use ::reqwest::{IntoUrl, blocking::Response, header::CONTENT_TYPE};
use reqwest::blocking as reqwest;

use crate::parser::ParserOutput;

static API_BASE: &str = "http://127.0.0.1:3030/api/v1";
static CLIENT: LazyLock<reqwest::Client> = LazyLock::new(|| reqwest::Client::new());

fn get<U: IntoUrl>(url: U) -> Result<Response, ::reqwest::Error> {
    CLIENT.get(url).send()
}

fn parse_from_file(path: &str) -> ParserOutput {
    let input = std::fs::read_to_string(path).unwrap();
    crate::parser::parse(&input)
}

pub fn add(kind: String, path: String) -> String {
    let url = format!("{API_BASE}/{kind}s");
    // let input = std::fs::read_to_string(&path).unwrap();
    // let output = crate::parser::parse(&input);
    let output = parse_from_file(&path);
    let new_post: aftershock_bridge::NewPost = output.into();
    if new_post.kind != kind {
        panic!("Kind doesn't match!")
    }
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

pub fn list(kind: String) -> String {
    let url = format!("{API_BASE}/{kind}s/all-meta");
    let body = get(url)
        .unwrap()
        .json::<Vec<aftershock_bridge::PostMeta>>()
        .unwrap();
    serde_json::to_string_pretty(&body).unwrap()
}

pub fn view(kind: String, id: String) -> String {
    let url = format!("{API_BASE}/{kind}s/uid/{id}");
    let body = get(url);
    serde_json::to_string_pretty(
        &body
            .ok()
            .map(|body| body.json::<aftershock_bridge::Post>().unwrap()),
    )
    .unwrap()
}

pub fn delete(kind: String, id: String) -> String {
    let url = format!("{API_BASE}/{kind}s/uid/{id}");
    let client = &CLIENT;
    let body = client
        .delete(url)
        .send()
        .unwrap()
        .json::<aftershock_bridge::Post>()
        .unwrap();
    serde_json::to_string_pretty(&body).unwrap()
}

fn send_update_request(url: &str, body: aftershock_bridge::UpdatePost) -> aftershock_bridge::Post {
    let body = serde_json::to_string(&body).unwrap();
    CLIENT
        .put(url)
        .header(CONTENT_TYPE, "application/json")
        .body(body)
        .send()
        .unwrap()
        .json::<aftershock_bridge::Post>()
        .unwrap()
}

pub fn publish(kind: String, id: String) -> String {
    let url = format!("{API_BASE}/{kind}s/uid/{id}");
    let body = aftershock_bridge::UpdatePost {
        title: None,
        body: None,
        published: Some(true),
    };
    // let body = serde_json::to_string(&body).unwrap();
    // let post = CLIENT
    //     .put(url)
    //     .header(CONTENT_TYPE, "application/json")
    //     .body(body)
    //     .send()
    //     .unwrap()
    //     .json::<aftershock_bridge::Post>()
    //     .unwrap();
    let post = send_update_request(&url, body);
    serde_json::to_string_pretty(&post).unwrap()
}

pub fn update(kind: String, path: String, id: String) -> String {
    let url = format!("{API_BASE}/{kind}s/uid/{id}");
    let output = parse_from_file(&path);
    let body = aftershock_bridge::UpdatePost {
        title: Some(output.metadata.title),
        body: Some(output.html),
        published: None,
    };
    let post = send_update_request(&url, body);
    serde_json::to_string_pretty(&post).unwrap()
}
