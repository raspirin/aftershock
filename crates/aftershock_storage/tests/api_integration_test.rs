use axum::Router;
use http_body_util::BodyExt;
use serde_json::{json, Value};
use std::env;
use tower::{Service, ServiceExt};

const API_V1: &str = "/api/v1";

// ===================================================================
// Test Helpers & Setup
// ===================================================================

fn setup_test_env() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());
    let env_path = std::path::Path::new(&manifest_dir).join("../../.env");
    
    if env_path.exists() {
        let _ = dotenvy::from_path(&env_path);
    }
    
    if let Ok(db_url) = env::var("DATABASE_URL") {
        if db_url.starts_with("./") {
            let project_root = std::path::Path::new(&manifest_dir).parent().unwrap().parent().unwrap();
            let abs_path = project_root.join(&db_url[2..]);
            unsafe { env::set_var("DATABASE_URL", abs_path.to_str().unwrap()); }
        }
    }
}

fn test_router() -> Router {
    setup_test_env();
    aftershock_storage::migration::run_migrations().expect("Failed to run migrations");
    aftershock_storage::create_router()
}

async fn make_request(router: &mut Router, method: &str, uri: &str, body: Option<Value>) -> (u16, Value) {
    let request = match body {
        Some(json_body) => axum::http::Request::builder()
            .method(method)
            .uri(uri)
            .header("Content-Type", "application/json")
            .body(axum::body::Body::from(json_body.to_string()))
            .expect("Failed to build request"),
        None => axum::http::Request::builder()
            .method(method)
            .uri(uri)
            .body(axum::body::Body::empty())
            .expect("Failed to build request"),
    };

    let response = ServiceExt::<axum::http::Request<axum::body::Body>>::ready(router)
        .await
        .expect("Service not ready")
        .call(request)
        .await
        .expect("Request failed");

    let status = response.status().as_u16();
    let body_bytes = response.into_body().collect().await.expect("Failed to collect body").to_bytes();

    let json_body = if body_bytes.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&body_bytes).unwrap_or_else(|_| {
            let text = String::from_utf8_lossy(&body_bytes);
            json!({ "_text": text.to_string() })
        })
    };

    (status, json_body)
}

async fn create_test_item(router: &mut Router, kind: &str, published: bool) -> (String, Value) {
    let endpoint = if kind == "post" { "posts" } else { "pages" };
    let payload = json!({
        "title": format!("Test {} {}", kind, uuid::Uuid::new_v4()),
        "kind": kind,
        "body": format!("Test {} body content.", kind),
        "tags": ["test"],
        "published": published
    });

    let (status, body) = make_request(router, "POST", &format!("{}/{}", API_V1, endpoint), Some(payload)).await;
    assert_eq!(status, 200, "Create {} failed", kind);
    let uid = body["uid"].as_str().unwrap().to_string();
    (uid, body)
}

// ===================================================================
// Core CRUD Tests (Post & Page)
// ===================================================================

#[tokio::test]
async fn test_post_lifecycle() {
    let mut router = test_router();
    
    // 1. Create & Verify (Must Read Created Success)
    let (uid, created) = create_test_item(&mut router, "post", true).await;
    let (status, read) = make_request(&mut router, "GET", &format!("{}/posts/uid/{}", API_V1, uid), None).await;
    assert_eq!(status, 200);
    assert_eq!(read["title"], created["title"]);

    // 2. Update (Full update must succeed)
    let update_payload = json!({
        "title": "Updated Title",
        "body": "Updated content",
        "published": false
    });
    let (status, updated) = make_request(&mut router, "PUT", &format!("{}/posts/uid/{}", API_V1, uid), Some(update_payload)).await;
    assert_eq!(status, 200);
    assert_eq!(updated["title"], "Updated Title");

    // 3. Delete & Verify Gone
    let (status, _) = make_request(&mut router, "DELETE", &format!("{}/posts/uid/{}", API_V1, uid), None).await;
    assert_eq!(status, 200);
    let (status, _) = make_request(&mut router, "GET", &format!("{}/posts/uid/{}", API_V1, uid), None).await;
    assert_eq!(status, 404);
}

#[tokio::test]
async fn test_page_lifecycle() {
    let mut router = test_router();
    let (uid, _) = create_test_item(&mut router, "page", true).await;

    // Verify Read
    let (status, read) = make_request(&mut router, "GET", &format!("{}/pages/uid/{}", API_V1, uid), None).await;
    assert_eq!(status, 200);
    assert_eq!(read["kind"], "page");

    // Cleanup
    let (status, _) = make_request(&mut router, "DELETE", &format!("{}/pages/uid/{}", API_V1, uid), None).await;
    assert_eq!(status, 200);
}

// ===================================================================
// List and Query Tests
// ===================================================================

#[tokio::test]
async fn test_list_filtering_and_meta() {
    let mut router = test_router();
    let (pub_uid, _) = create_test_item(&mut router, "post", true).await;
    let (unpub_uid, _) = create_test_item(&mut router, "post", false).await;

    // 1. Published only list
    let (status, list) = make_request(&mut router, "GET", &format!("{}/posts", API_V1), None).await;
    assert_eq!(status, 200);
    let posts = list.as_array().unwrap();
    assert!(posts.iter().any(|p| p["uid"] == pub_uid));
    assert!(!posts.iter().any(|p| p["uid"] == unpub_uid));

    // 2. All posts list (Admin)
    let (status, list_all) = make_request(&mut router, "GET", &format!("{}/posts/all", API_V1), None).await;
    assert_eq!(status, 200);
    let posts_all = list_all.as_array().unwrap();
    assert!(posts_all.iter().any(|p| p["uid"] == pub_uid));
    assert!(posts_all.iter().any(|p| p["uid"] == unpub_uid));

    // 3. Meta endpoint (Excludes body)
    let (status, meta) = make_request(&mut router, "GET", &format!("{}/posts/meta", API_V1), None).await;
    assert_eq!(status, 200);
    let meta_posts = meta.as_array().unwrap();
    assert!(meta_posts.iter().all(|p| p.get("body").is_none() || p["body"].is_null()));

    // Cleanup
    make_request(&mut router, "DELETE", &format!("{}/posts/uid/{}", API_V1, pub_uid), None).await;
    make_request(&mut router, "DELETE", &format!("{}/posts/uid/{}", API_V1, unpub_uid), None).await;
}

#[tokio::test]
async fn test_tag_search() {
    let mut router = test_router();
    let (uid, _) = create_test_item(&mut router, "post", true).await;
    
    let (status, body) = make_request(&mut router, "GET", &format!("{}/posts/tag/test", API_V1), None).await;
    assert_eq!(status, 200);
    assert!(body.as_array().unwrap().iter().any(|p| p["uid"] == uid));

    // Non-existent tag returns empty
    let (_, body_empty) = make_request(&mut router, "GET", &format!("{}/posts/tag/nonexistent", API_V1), None).await;
    assert!(body_empty.as_array().unwrap().is_empty());

    make_request(&mut router, "DELETE", &format!("{}/posts/uid/{}", API_V1, uid), None).await;
}

// ===================================================================
// Validation & Error Handling (Must Fail Cases)
// ===================================================================

#[tokio::test]
async fn test_error_handling_nonexistent() {
    let mut router = test_router();
    let fake_uid = "nonexistent-123";

    let (status, _) = make_request(&mut router, "GET", &format!("{}/posts/uid/{}", API_V1, fake_uid), None).await;
    assert_eq!(status, 404);

    let (status, _) = make_request(&mut router, "PUT", &format!("{}/posts/uid/{}", API_V1, fake_uid), Some(json!({"title":"x"}))).await;
    assert_eq!(status, 404);

    let (status, _) = make_request(&mut router, "DELETE", &format!("{}/posts/uid/{}", API_V1, fake_uid), None).await;
    assert_eq!(status, 404);
}

#[tokio::test]
async fn test_invalid_creation_payloads() {
    let mut router = test_router();
    
    // Missing 'kind' field
    let (status, _) = make_request(&mut router, "POST", &format!("{}/posts", API_V1), Some(json!({"title":"No Kind"}))).await;
    assert!(status >= 400 && status < 500);

    // Empty title
    let (status, _) = make_request(&mut router, "POST", &format!("{}/posts", API_V1), Some(json!({"title":"", "kind":"post"}))).await;
    assert!(status >= 400 && status < 500);
}

#[tokio::test]
async fn test_isolation_and_wrong_endpoints() {
    let mut router = test_router();
    let (post_uid, _) = create_test_item(&mut router, "post", true).await;
    
    // Accessing post via pages endpoint
    let (status, _) = make_request(&mut router, "GET", &format!("{}/pages/uid/{}", API_V1, post_uid), None).await;
    assert_eq!(status, 404);

    // Double delete
    make_request(&mut router, "DELETE", &format!("{}/posts/uid/{}", API_V1, post_uid), None).await;
    let (status, _) = make_request(&mut router, "DELETE", &format!("{}/posts/uid/{}", API_V1, post_uid), None).await;
    assert_eq!(status, 404);
}

#[tokio::test]
async fn test_partial_update_preservation() {
    let mut router = test_router();
    let (uid, original) = create_test_item(&mut router, "post", true).await;
    
    // Only update title
    let (status, body) = make_request(&mut router, "PUT", &format!("{}/posts/uid/{}", API_V1, uid), Some(json!({"title": "New"}))).await;
    assert_eq!(status, 200);
    assert_eq!(body["title"], "New");
    assert_eq!(body["body"], original["body"], "Body should remain unchanged");
    
    make_request(&mut router, "DELETE", &format!("{}/posts/uid/{}", API_V1, uid), None).await;
}