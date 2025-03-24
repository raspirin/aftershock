use leptos::prelude::*;

static API_BASE: &'static str = "http://127.0.0.1:3030/api/v1";

#[server]
pub async fn get_published_posts_meta() -> Result<Vec<aftershock_bridge::PostMeta>, ServerFnError> {
    let url = format!("{API_BASE}/posts/meta");
    let meta = reqwest::get(url)
        .await
        .unwrap()
        .json::<Vec<aftershock_bridge::PostMeta>>()
        .await
        .unwrap();
    Ok(meta)
}

#[server]
pub async fn get_post_by_uid(uid: String) -> Result<aftershock_bridge::Post, ServerFnError> {
    let url = format!("{API_BASE}/posts/uid/{uid}");
    let post = reqwest::get(url)
        .await
        .unwrap()
        .json::<aftershock_bridge::Post>()
        .await
        .unwrap();
    Ok(post)
}
