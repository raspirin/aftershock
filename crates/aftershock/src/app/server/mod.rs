pub async fn get_published_posts_meta() -> Vec<aftershock_bridge::PostMeta> {
    let meta = reqwest::get("http://127.0.0.1:3001/api/v1/posts/meta")
        .await
        .unwrap()
        .json::<Vec<aftershock_bridge::PostMeta>>()
        .await
        .unwrap();
    meta
}
