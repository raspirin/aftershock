use leptos::prelude::*;

#[cfg(feature = "ssr")]
static API_BASE: &'static str = "http://127.0.0.1:3030/api/v1";

#[server]
pub async fn get_published_posts_meta() -> Result<Vec<aftershock_bridge::PostMeta>, ServerFnError> {
    let url = format!("{API_BASE}/posts/meta");
    let meta = reqwest::get(url)
        .await?
        .json::<Vec<aftershock_bridge::PostMeta>>()
        .await?;
    Ok(meta)
}

#[server]
pub async fn get_post_by_uid(uid: String) -> Result<aftershock_bridge::Post, ServerFnError> {
    let url = format!("{API_BASE}/posts/uid/{uid}");
    let post = reqwest::get(url)
        .await?
        .json::<aftershock_bridge::Post>()
        .await?;
    Ok(post)
}

#[server]
pub async fn get_page(name: String) -> Result<aftershock_bridge::Post, ServerFnError> {
    let url = format!("{API_BASE}/pages/uid/{name}");
    let page = reqwest::get(url)
        .await?
        .json::<aftershock_bridge::Post>()
        .await?;
    Ok(page)
}

#[server]
pub async fn get_posts_meta_by_tag(
    tag: String,
) -> Result<Vec<aftershock_bridge::PostMeta>, ServerFnError> {
    let url = format!("{API_BASE}/posts/tag/{tag}");
    let meta = reqwest::get(url)
        .await?
        .json::<Vec<aftershock_bridge::PostMeta>>()
        .await?;
    Ok(meta)
}
