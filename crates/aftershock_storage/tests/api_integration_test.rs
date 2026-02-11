//! Integration tests for aftershock_storage API
//!
//! These tests use axum's Router directly with tower::ServiceExt,
//! allowing all API tests to run in-memory without starting a TCP server.

use axum::Router;
use std::env;

fn setup_test_env() {
    // Load .env from project root (2 levels up from manifest dir: crates/aftershock_storage -> root)
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());
    let env_path = std::path::Path::new(&manifest_dir).join("../../.env");
    
    if env_path.exists() {
        // Load from the specific path
        let _ = dotenvy::from_path(&env_path);
    }
    
    // If DATABASE_URL is a relative path starting with ./, convert to absolute
    if let Ok(db_url) = env::var("DATABASE_URL") {
        if db_url.starts_with("./") {
            let project_root = std::path::Path::new(&manifest_dir).parent().unwrap().parent().unwrap();
            let abs_path = project_root.join(&db_url[2..]);
            // Use std::env::set_var only if not already set (set before lazy lock init)
            unsafe {
                env::set_var("DATABASE_URL", abs_path.to_str().unwrap());
            }
        }
    }
}
use http_body_util::BodyExt;
use serde_json::{Value, json};
use tower::{Service, ServiceExt};

const API_V1: &str = "/api/v1";

/// Get the test router
fn test_router() -> Router {
    setup_test_env();
    aftershock_storage::migration::run_migrations().expect("Failed to run migrations");
    aftershock_storage::create_router()
}

/// Make a test request
async fn make_request(
    router: &mut Router,
    method: &str,
    uri: &str,
    body: Option<Value>,
) -> (u16, Value) {
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

    let body_bytes = response
        .into_body()
        .collect()
        .await
        .expect("Failed to collect body")
        .to_bytes();

    let json_body = if body_bytes.is_empty() {
        Value::Null
    } else {
        // Try to parse as JSON, fallback to string if it fails
        serde_json::from_slice(&body_bytes).unwrap_or_else(|_| {
            // If JSON parsing fails, wrap the text in a JSON object
            let text = String::from_utf8_lossy(&body_bytes);
            json!({ "_text": text.to_string() })
        })
    };

    (status, json_body)
}

/// Create a test post and return (uid, full_response)
async fn create_test_post(router: &mut Router, published: bool) -> (String, Value) {
    let new_post = json!({
        "title": format!("Test Post {}", uuid::Uuid::new_v4()),
        "kind": "post",
        "body": "Test post body content.",
        "tags": ["test"],
        "published": published
    });

    let (status, body) = make_request(router, "POST", &format!("{}/posts", API_V1), Some(new_post)).await;
    assert_eq!(status, 200, "Create post failed: {:?}", body);
    
    assert!(body["uid"].as_str().is_some(), "Response should contain uid");
    assert_eq!(body["kind"].as_str(), Some("post"), "Kind should be 'post'");
    assert_eq!(body["published"].as_bool(), Some(published), "Published field mismatch");
    
    let uid = body["uid"].as_str().unwrap().to_string();
    (uid, body)
}

/// Create a test page and return (uid, full_response)
async fn create_test_page(router: &mut Router, published: bool) -> (String, Value) {
    let new_page = json!({
        "title": format!("TestPage{}", uuid::Uuid::new_v4().simple()),
        "kind": "page",
        "body": "Test page body content.",
        "tags": ["test"],
        "published": published
    });

    let (status, body) = make_request(router, "POST", &format!("{}/pages", API_V1), Some(new_page)).await;
    assert_eq!(status, 200, "Create page failed: {:?}", body);
    
    assert!(body["uid"].as_str().is_some(), "Response should contain uid");
    assert_eq!(body["kind"].as_str(), Some("page"), "Kind should be 'page'");
    assert_eq!(body["published"].as_bool(), Some(published), "Published field mismatch");
    
    let uid = body["uid"].as_str().unwrap().to_string();
    (uid, body)
}

/// Delete a post by uid - returns true if actually deleted (200), false if already gone (404)
async fn delete_post(router: &mut Router, uid: &str) -> bool {
    let (status, _) = make_request(router, "DELETE", &format!("{}/posts/uid/{}", API_V1, uid), None).await;
    assert!(status == 200 || status == 404, "Delete post unexpected status: {}", status);
    status == 200
}

/// Delete a page by uid - returns true if actually deleted (200), false if already gone (404)
async fn delete_page(router: &mut Router, uid: &str) -> bool {
    let (status, _) = make_request(router, "DELETE", &format!("{}/pages/uid/{}", API_V1, uid), None).await;
    assert!(status == 200 || status == 404, "Delete page unexpected status: {}", status);
    status == 200
}

// ===================================================================
// Basic CRUD Tests
// ===================================================================

#[tokio::test]
async fn test_create_and_delete_post() {
    let mut router = test_router();
    let (uid, body) = create_test_post(&mut router, true).await;
    
    // Verify creation response structure
    assert!(body.get("uid").is_some(), "Response should contain uid");
    assert!(body.get("title").is_some(), "Response should contain title");
    assert!(body.get("body").is_some(), "Response should contain body");
    assert!(body.get("created_at").is_some(), "Response should contain created_at");
    assert!(body.get("updated_at").is_some(), "Response should contain updated_at");
    
    // Delete and verify
    let deleted = delete_post(&mut router, &uid).await;
    assert!(deleted, "Post should be successfully deleted");
    
    // Verify deletion
    let (status, _) = make_request(&mut router, "GET", &format!("{}/posts/uid/{}", API_V1, uid), None).await;
    assert_eq!(status, 404, "Post should not exist after deletion");
}

#[tokio::test]
async fn test_create_and_delete_page() {
    let mut router = test_router();
    let (uid, body) = create_test_page(&mut router, true).await;
    
    // Verify creation response structure
    assert!(body.get("uid").is_some(), "Response should contain uid");
    assert!(body.get("title").is_some(), "Response should contain title");
    assert_eq!(body["kind"], "page", "Kind should be 'page'");
    
    // Delete and verify
    let deleted = delete_page(&mut router, &uid).await;
    assert!(deleted, "Page should be successfully deleted");
    
    // Verify deletion
    let (status, _) = make_request(&mut router, "GET", &format!("{}/pages/uid/{}", API_V1, uid), None).await;
    assert_eq!(status, 404, "Page should not exist after deletion");
}

#[tokio::test]
async fn test_get_post_by_uid() {
    let mut router = test_router();
    let (uid, post) = create_test_post(&mut router, true).await;
    
    let (status, body) = make_request(&mut router, "GET", &format!("{}/posts/uid/{}", API_V1, uid), None).await;
    assert_eq!(status, 200, "Get post should return 200");
    assert_eq!(body["uid"].as_str().unwrap(), uid, "UID mismatch");
    assert_eq!(body["title"], post["title"], "Title mismatch");
    assert_eq!(body["body"], post["body"], "Body mismatch");
    assert_eq!(body["published"], true, "Published should be true");
    assert_eq!(body["kind"], "post", "Kind should be 'post'");
    
    delete_post(&mut router, &uid).await;
}

#[tokio::test]
async fn test_get_page_by_uid() {
    let mut router = test_router();
    let (uid, page) = create_test_page(&mut router, true).await;
    
    let (status, body) = make_request(&mut router, "GET", &format!("{}/pages/uid/{}", API_V1, uid), None).await;
    assert_eq!(status, 200, "Get page should return 200");
    assert_eq!(body["uid"].as_str().unwrap(), uid, "UID mismatch");
    assert_eq!(body["title"], page["title"], "Title mismatch");
    assert_eq!(body["kind"], "page", "Kind should be 'page'");
    
    delete_page(&mut router, &uid).await;
}

#[tokio::test]
async fn test_update_post() {
    let mut router = test_router();
    let (uid, original) = create_test_post(&mut router, true).await;
    
    let update = json!({
        "title": "Updated Title",
        "body": "Updated body content"
    });
    
    let (status, body) = make_request(&mut router, "PUT", &format!("{}/posts/uid/{}", API_V1, uid), Some(update)).await;
    assert_eq!(status, 200, "Update post should return 200");
    assert_eq!(body["uid"], original["uid"], "UID should remain unchanged");
    assert_eq!(body["title"], "Updated Title", "Title should be updated");
    assert_eq!(body["body"], "Updated body content", "Body should be updated");
    assert_eq!(body["kind"], "post", "Kind should remain 'post'");
    
    // Verify the update persisted
    let (status, body) = make_request(&mut router, "GET", &format!("{}/posts/uid/{}", API_V1, uid), None).await;
    assert_eq!(status, 200);
    assert_eq!(body["title"], "Updated Title");
    
    delete_post(&mut router, &uid).await;
}

#[tokio::test]
async fn test_update_page() {
    let mut router = test_router();
    let (uid, original) = create_test_page(&mut router, true).await;
    
    let update = json!({ "title": "Updated Page Title" });
    
    let (status, body) = make_request(&mut router, "PUT", &format!("{}/pages/uid/{}", API_V1, uid), Some(update)).await;
    assert_eq!(status, 200, "Update page should return 200");
    assert_eq!(body["uid"], original["uid"], "UID should remain unchanged");
    assert_eq!(body["title"], "Updated Page Title", "Title should be updated");
    assert_eq!(body["kind"], "page", "Kind should remain 'page'");
    
    // Verify the update persisted
    let (status, body) = make_request(&mut router, "GET", &format!("{}/pages/uid/{}", API_V1, uid), None).await;
    assert_eq!(status, 200);
    assert_eq!(body["title"], "Updated Page Title");
    
    delete_page(&mut router, &uid).await;
}

// ===================================================================
// List and Query Tests
// ===================================================================

#[tokio::test]
async fn test_list_published_posts_only() {
    let mut router = test_router();
    
    // Create a published and an unpublished post
    let (pub_uid, _) = create_test_post(&mut router, true).await;
    let (unpub_uid, _) = create_test_post(&mut router, false).await;
    
    // Get published posts
    let (status, body) = make_request(&mut router, "GET", &format!("{}/posts", API_V1), None).await;
    assert_eq!(status, 200, "List published posts should return 200");
    
    let posts = body.as_array().expect("Expected array");
    
    // Published post should be included
    assert!(
        posts.iter().any(|p| p["uid"].as_str().unwrap() == pub_uid),
        "Published post should appear in list"
    );
    
    // Published field should be true for all returned posts
    for post in posts {
        assert_eq!(
            post["published"].as_bool(), 
            Some(true),
            "All posts in /posts endpoint should have published=true"
        );
    }
    
    // Cleanup
    delete_post(&mut router, &pub_uid).await;
    delete_post(&mut router, &unpub_uid).await;
}

#[tokio::test]
async fn test_list_all_posts_includes_unpublished() {
    let mut router = test_router();
    let (pub_uid, _) = create_test_post(&mut router, true).await;
    let (unpub_uid, _) = create_test_post(&mut router, false).await;
    
    // Get all posts (including unpublished)
    let (status, body) = make_request(&mut router, "GET", &format!("{}/posts/all", API_V1), None).await;
    assert_eq!(status, 200, "List all posts should return 200");
    
    let posts = body.as_array().expect("Expected array");
    
    // Both published and unpublished should be included
    assert!(
        posts.iter().any(|p| p["uid"].as_str().unwrap() == pub_uid),
        "Published post should appear in all posts list"
    );
    assert!(
        posts.iter().any(|p| p["uid"].as_str().unwrap() == unpub_uid),
        "Unpublished post should appear in all posts list"
    );
    
    // Cleanup
    delete_post(&mut router, &pub_uid).await;
    delete_post(&mut router, &unpub_uid).await;
}

#[tokio::test]
async fn test_list_all_pages_includes_unpublished() {
    let mut router = test_router();
    let (pub_uid, _) = create_test_page(&mut router, true).await;
    let (unpub_uid, _) = create_test_page(&mut router, false).await;
    
    // Get all pages (including unpublished)
    let (status, body) = make_request(&mut router, "GET", &format!("{}/pages/all", API_V1), None).await;
    assert_eq!(status, 200, "List all pages should return 200");
    
    let pages = body.as_array().expect("Expected array");
    
    // Both published and unpublished should be included
    assert!(
        pages.iter().any(|p| p["uid"].as_str().unwrap() == pub_uid),
        "Published page should appear in all pages list"
    );
    assert!(
        pages.iter().any(|p| p["uid"].as_str().unwrap() == unpub_uid),
        "Unpublished page should appear in all pages list"
    );
    
    // Cleanup
    delete_page(&mut router, &pub_uid).await;
    delete_page(&mut router, &unpub_uid).await;
}

#[tokio::test]
async fn test_get_posts_meta_excludes_body() {
    let mut router = test_router();
    let (uid, _) = create_test_post(&mut router, true).await;
    
    let (status, body) = make_request(&mut router, "GET", &format!("{}/posts/meta", API_V1), None).await;
    assert_eq!(status, 200, "Get posts meta should return 200");
    
    let posts = body.as_array().expect("Expected array");
    assert!(!posts.is_empty(), "Meta endpoint should return at least one post");
    
    for post in posts {
        // Meta should exclude body field
        assert!(
            post.get("body").is_none() || post["body"].is_null(),
            "Meta endpoint should not include body: {:?}", 
            post
        );
        // But should include other important fields
        assert!(post.get("uid").is_some(), "Meta should include uid");
        assert!(post.get("title").is_some(), "Meta should include title");
        assert!(post.get("created_at").is_some(), "Meta should include created_at");
    }
    
    delete_post(&mut router, &uid).await;
}

#[tokio::test]
async fn test_get_pages_meta_excludes_body() {
    let mut router = test_router();
    let (uid, _) = create_test_page(&mut router, true).await;
    
    let (status, body) = make_request(&mut router, "GET", &format!("{}/pages/meta", API_V1), None).await;
    assert_eq!(status, 200, "Get pages meta should return 200");
    
    let pages = body.as_array().expect("Expected array");
    assert!(!pages.is_empty(), "Meta endpoint should return at least one page");
    
    for page in pages {
        // Meta should exclude body field
        assert!(
            page.get("body").is_none() || page["body"].is_null(),
            "Meta endpoint should not include body"
        );
        // But should include other important fields
        assert!(page.get("uid").is_some(), "Meta should include uid");
        assert!(page.get("title").is_some(), "Meta should include title");
    }
    
    delete_page(&mut router, &uid).await;
}

#[tokio::test]
async fn test_get_posts_by_tag() {
    let mut router = test_router();
    let (uid, _) = create_test_post(&mut router, true).await;
    
    let (status, body) = make_request(&mut router, "GET", &format!("{}/posts/tag/test", API_V1), None).await;
    assert_eq!(status, 200, "Get posts by tag should return 200");
    
    let posts = body.as_array().expect("Expected array");
    assert!(
        posts.iter().any(|p| p["uid"].as_str().unwrap() == uid),
        "Created post should be in returned list"
    );
    
    // All returned posts should have the search tag
    for post in posts {
        let tags = post["tags"].as_array().expect("Post should have tags");
        assert!(
            tags.iter().any(|t| t.as_str() == Some("test")),
            "All returned posts should have 'test' tag"
        );
    }
    
    delete_post(&mut router, &uid).await;
}

#[tokio::test]
async fn test_get_pages_by_tag() {
    let mut router = test_router();
    let (uid, _) = create_test_page(&mut router, true).await;
    
    let (status, body) = make_request(&mut router, "GET", &format!("{}/pages/tag/test", API_V1), None).await;
    assert_eq!(status, 200, "Get pages by tag should return 200");
    
    let pages = body.as_array().expect("Expected array");
    assert!(
        pages.iter().any(|p| p["uid"].as_str().unwrap() == uid),
        "Created page should be in returned list"
    );
    
    // All returned pages should have the search tag
    for page in pages {
        let tags = page["tags"].as_array().expect("Page should have tags");
        assert!(
            tags.iter().any(|t| t.as_str() == Some("test")),
            "All returned pages should have 'test' tag"
        );
    }
    
    delete_page(&mut router, &uid).await;
}

#[tokio::test]
async fn test_get_posts_by_nonexistent_tag_returns_empty() {
    let mut router = test_router();
    
    let (status, body) = make_request(&mut router, "GET", &format!("{}/posts/tag/nonexistent-tag-{}"
        , API_V1, uuid::Uuid::new_v4().simple()), None).await;
    assert_eq!(status, 200, "Search for nonexistent tag should return 200");
    
    let posts = body.as_array().expect("Expected array");
    assert!(posts.is_empty(), "Search for nonexistent tag should return empty array");
}

// ===================================================================
// Error Handling Tests
// ===================================================================

#[tokio::test]
async fn test_get_nonexistent_post_returns_404() {
    let mut router = test_router();
    
    let (status, body) = make_request(&mut router, "GET", &format!("{}/posts/uid/nonexistent-post-{}", API_V1, uuid::Uuid::new_v4()), None).await;
    assert_eq!(status, 404, "Get nonexistent post should return 404");
    // Response can be text error message or JSON with error field
    let has_error = body.get("error").is_some() || body == Value::Null || body.get("_text").is_some();
    assert!(has_error, "Should have error info or be empty, got: {:?}", body);
}

#[tokio::test]
async fn test_get_nonexistent_page_returns_404() {
    let mut router = test_router();
    
    let (status, _body) = make_request(&mut router, "GET", &format!("{}/pages/uid/nonexistent-page-{}", API_V1, uuid::Uuid::new_v4()), None).await;
    assert_eq!(status, 404, "Get nonexistent page should return 404");
}

#[tokio::test]
async fn test_update_nonexistent_post_returns_404() {
    let mut router = test_router();
    
    let update = json!({ "title": "Updated Title" });
    let (status, _) = make_request(&mut router, "PUT", &format!("{}/posts/uid/nonexistent-post-{}", API_V1, uuid::Uuid::new_v4()), Some(update)).await;
    assert_eq!(status, 404, "Update nonexistent post should return 404");
}

#[tokio::test]
async fn test_update_nonexistent_page_returns_404() {
    let mut router = test_router();
    
    let update = json!({ "title": "Updated Title" });
    let (status, _) = make_request(&mut router, "PUT", &format!("{}/pages/uid/nonexistent-page-{}", API_V1, uuid::Uuid::new_v4()), Some(update)).await;
    assert_eq!(status, 404, "Update nonexistent page should return 404");
}

#[tokio::test]
async fn test_delete_nonexistent_post_returns_404() {
    let mut router = test_router();
    
    let (status, _) = make_request(&mut router, "DELETE", &format!("{}/posts/uid/nonexistent-post-{}", API_V1, uuid::Uuid::new_v4()), None).await;
    assert_eq!(status, 404, "Delete nonexistent post should return 404");
}

#[tokio::test]
async fn test_delete_nonexistent_page_returns_404() {
    let mut router = test_router();
    
    let (status, _) = make_request(&mut router, "DELETE", &format!("{}/pages/uid/nonexistent-page-{}", API_V1, uuid::Uuid::new_v4()), None).await;
    assert_eq!(status, 404, "Delete nonexistent page should return 404");
}

// ===================================================================
// Full Lifecycle Tests
// ===================================================================

#[tokio::test]
async fn test_post_full_lifecycle() {
    let mut router = test_router();
    
    // 1. Create
    let (uid, create_body) = create_test_post(&mut router, true).await;
    assert!(!uid.is_empty(), "UID should not be empty");
    assert_eq!(create_body["published"], true);
    
    let original_created_at = create_body["created_at"].as_i64().unwrap();
    let _original_updated_at = create_body["updated_at"].as_i64().unwrap();
    
    // 2. Read
    let (status, read_body) = make_request(&mut router, "GET", &format!("{}/posts/uid/{}", API_V1, uid), None).await;
    assert_eq!(status, 200, "Read should return 200");
    assert_eq!(read_body["uid"], uid);
    assert_eq!(read_body["created_at"].as_i64().unwrap(), original_created_at);
    
    // 3. Update title and body
    let update = json!({
        "title": "Updated Title",
        "body": "Updated body"
    });
    let (status, update_body) = make_request(&mut router, "PUT", &format!("{}/posts/uid/{}", API_V1, uid), Some(update)).await;
    assert_eq!(status, 200, "Update should return 200");
    assert_eq!(update_body["title"], "Updated Title");
    assert_eq!(update_body["body"], "Updated body");
    assert_eq!(update_body["uid"], uid);
    assert_eq!(update_body["created_at"].as_i64().unwrap(), original_created_at, "Created at should not change");
    // Updated_at might change, which is expected
    
    // 4. Verify update persisted
    let (status, verify_body) = make_request(&mut router, "GET", &format!("{}/posts/uid/{}", API_V1, uid), None).await;
    assert_eq!(status, 200);
    assert_eq!(verify_body["title"], "Updated Title");
    
    // 5. Update published status
    let publish_update = json!({ "published": false });
    let (status, _) = make_request(&mut router, "PUT", &format!("{}/posts/uid/{}", API_V1, uid), Some(publish_update)).await;
    assert_eq!(status, 200, "Update published status should return 200");
    
    // Verify in list endpoints
    let (status, list_body) = make_request(&mut router, "GET", &format!("{}/posts", API_V1), None).await;
    assert_eq!(status, 200);
    let posts = list_body.as_array().unwrap();
    assert!(
        !posts.iter().any(|p| p["uid"].as_str().unwrap() == uid),
        "Unpublished post should not appear in published list"
    );
    
    // 6. Delete
    let (status, _) = make_request(&mut router, "DELETE", &format!("{}/posts/uid/{}", API_V1, uid), None).await;
    assert_eq!(status, 200, "Delete should return 200");
    
    // 7. Verify deletion
    let (status, _) = make_request(&mut router, "GET", &format!("{}/posts/uid/{}", API_V1, uid), None).await;
    assert_eq!(status, 404, "Post should not exist after deletion");
}

#[tokio::test]
async fn test_page_full_lifecycle() {
    let mut router = test_router();
    
    // 1. Create
    let (uid, create_body) = create_test_page(&mut router, true).await;
    assert!(!uid.is_empty(), "UID should not be empty");
    assert_eq!(create_body["kind"], "page");
    
    // 2. Read
    let (status, read_body) = make_request(&mut router, "GET", &format!("{}/pages/uid/{}", API_V1, uid), None).await;
    assert_eq!(status, 200, "Read should return 200");
    assert_eq!(read_body["uid"], uid);
    assert_eq!(read_body["kind"], "page");
    
    // 3. Update
    let update = json!({ 
        "title": "Updated Page Title",
        "body": "Updated page body"
    });
    let (status, update_body) = make_request(&mut router, "PUT", &format!("{}/pages/uid/{}", API_V1, uid), Some(update)).await;
    assert_eq!(status, 200, "Update should return 200");
    assert_eq!(update_body["title"], "Updated Page Title");
    assert_eq!(update_body["kind"], "page", "Kind should remain 'page'");
    
    // 4. Get current tags (Update with tags is not yet supported)
    // Tags should remain unchanged from initial creation
    let (status, tag_body) = make_request(&mut router, "GET", &format!("{}/pages/uid/{}", API_V1, uid), None).await;
    assert_eq!(status, 200);
    let tags = tag_body["tags"].as_array().unwrap();
    // Initial creation has 1 tag: ["test"]
    assert_eq!(tags.len(), 1, "Tags count should remain unchanged after update without tags support");
    
    // 5. Delete
    let (status, _) = make_request(&mut router, "DELETE", &format!("{}/pages/uid/{}", API_V1, uid), None).await;
    assert_eq!(status, 200, "Delete should return 200");
    
    // 6. Verify deletion
    let (status, _) = make_request(&mut router, "GET", &format!("{}/pages/uid/{}", API_V1, uid), None).await;
    assert_eq!(status, 404, "Page should not exist after deletion");
}

// ===================================================================
// Validation Tests
// ===================================================================

#[tokio::test]
async fn test_create_post_with_multiple_tags() {
    let mut router = test_router();
    
    let new_post = json!({
        "title": "Multi-tag Post",
        "kind": "post",
        "body": "Post with multiple tags.",
        "tags": ["rust", "api", "test", "integration"],
        "published": true
    });

    let (status, body) = make_request(&mut router, "POST", &format!("{}/posts", API_V1), Some(new_post)).await;
    assert_eq!(status, 200, "Create post with multiple tags should succeed");
    
    let tags = body["tags"].as_array().expect("Tags should be array");
    assert_eq!(tags.len(), 4);
    assert!(tags.iter().any(|t| t.as_str() == Some("rust")));
    assert!(tags.iter().any(|t| t.as_str() == Some("api")));
    assert!(tags.iter().any(|t| t.as_str() == Some("test")));
    assert!(tags.iter().any(|t| t.as_str() == Some("integration")));
    
    let uid = body["uid"].as_str().unwrap();
    delete_post(&mut router, uid).await;
}

#[tokio::test]
async fn test_update_partial_fields_preserves_others() {
    let mut router = test_router();
    let (uid, original) = create_test_post(&mut router, true).await;
    
    let original_body = original["body"].as_str().unwrap();
    let original_tags = original["tags"].clone();
    
    // Update only title
    let update = json!({ "title": "Only Title Updated" });
    let (status, body) = make_request(&mut router, "PUT", &format!("{}/posts/uid/{}", API_V1, uid), Some(update)).await;
    assert_eq!(status, 200);
    
    // Other fields should be preserved
    assert_eq!(body["title"], "Only Title Updated");
    assert_eq!(body["body"].as_str().unwrap(), original_body, "Body should be preserved");
    assert_eq!(body["tags"], original_tags, "Tags should be preserved");
    
    delete_post(&mut router, &uid).await;
}

// =============================================================================
// MUST SUCCEED TESTS (reduce false positives)
// =============================================================================

/// Test that accessing a resource after creation returns the same data
#[tokio::test]
async fn test_must_read_created_post_success() {
    let mut router = test_router();
    let (uid, created) = create_test_post(&mut router, true).await;
    
    // Must be able to read the same post back
    let (status, read) = make_request(&mut router, "GET", &format!("{}/posts/uid/{}", API_V1, uid), None).await;
    assert_eq!(status, 200, "Reading a just-created post must succeed");
    assert_eq!(read["uid"], created["uid"], "UID must match");
    assert_eq!(read["title"], created["title"], "Title must match");
    assert_eq!(read["body"], created["body"], "Body must match");
    assert_eq!(read["published"], created["published"], "Published flag must match");
    
    delete_post(&mut router, &uid).await;
}

/// Test that API handles empty body (may accept or reject)
#[tokio::test]
async fn test_empty_body_handling() {
    let mut router = test_router();
    
    let new_post = json!({
        "title": "Post with empty body",
        "kind": "post",
        "body": "",
        "published": true
    });
    
    let (status, body) = make_request(&mut router, "POST", &format!("{}/posts", API_V1), Some(new_post)).await;
    // API may accept or reject empty body - this test documents current behavior
    if status == 200 {
        assert_eq!(body["body"].as_str(), Some(""), "Empty body must be preserved");
        let uid = body["uid"].as_str().unwrap();
        delete_post(&mut router, uid).await;
    } else {
        // If API rejects empty body, it should return an error status (422)
        assert!(
            status >= 400 && status < 500,
            "Rejection of empty body should return 4xx status, got {}",
            status
        );
    }
}

/// Test update with all fields must succeed and persist all changes
#[tokio::test]
async fn test_must_update_all_fields_success() {
    let mut router = test_router();
    let (uid, _) = create_test_post(&mut router, false).await;
    
    let update = json!({
        "title": "Completely Updated Title",
        "body": "Completely updated body content.",
        "published": true,
        "deleted": false
    });
    
    let (status, updated) = make_request(&mut router, "PUT", &format!("{}/posts/uid/{}", API_V1, uid), Some(update)).await;
    assert_eq!(status, 200, "Full update must succeed");
    assert_eq!(updated["title"], "Completely Updated Title");
    assert_eq!(updated["body"], "Completely updated body content.");
    assert_eq!(updated["published"], true);
    
    // Verify persistence - read again to ensure update was saved
    let (status, read) = make_request(&mut router, "GET", &format!("{}/posts/uid/{}", API_V1, uid), None).await;
    assert_eq!(status, 200, "Reading after update must succeed");
    assert_eq!(read["title"], "Completely Updated Title", "Update must persist");
    assert_eq!(read["published"], true, "Published flag must persist");
    
    delete_post(&mut router, &uid).await;
}

/// Test that list filters work correctly - published posts must appear in published list
#[tokio::test]
async fn test_must_list_only_published_posts_success() {
    let mut router = test_router();
    
    // Create unpublished post
    let (unpub_uid, _) = create_test_post(&mut router, false).await;
    
    // Create published post
    let (pub_uid, _) = create_test_post(&mut router, true).await;
    
    // Get published list
    let (status, list) = make_request(&mut router, "GET", &format!("{}/posts?published=true", API_V1), None).await;
    assert_eq!(status, 200);
    
    // API may return array directly or wrapped in object with items field
    let items = if list.is_array() {
        list.as_array().unwrap()
    } else {
        list["items"].as_array().expect("Response should be array or have items field")
    };
    
    let pub_uids: Vec<_> = items.iter().filter_map(|i| i["uid"].as_str()).collect();
    
    assert!(
        pub_uids.contains(&pub_uid.as_str()),
        "Published post must appear in published list"
    );
    assert!(
        !pub_uids.contains(&unpub_uid.as_str()),
        "Unpublished post must NOT appear in published list"
    );
    
    delete_post(&mut router, &unpub_uid).await;
    delete_post(&mut router, &pub_uid).await;
}

// =============================================================================
// MUST FAIL TESTS (reduce false negatives)
// =============================================================================

/// Test that accessing non-existent UID must fail with 404
#[tokio::test]
async fn test_must_fail_get_nonexistent_post() {
    let mut router = test_router();
    let fake_uid = "nonexistent-uid-12345";
    
    let (status, _) = make_request(&mut router, "GET", &format!("{}/posts/uid/{}", API_V1, fake_uid), None).await;
    assert_eq!(status, 404, "Getting non-existent post must return 404");
}

/// Test that accessing non-existent UID must fail with 404 (page)
#[tokio::test]
async fn test_must_fail_get_nonexistent_page() {
    let mut router = test_router();
    let fake_uid = "nonexistent-page-uid-67890";
    
    let (status, _) = make_request(&mut router, "GET", &format!("{}/pages/uid/{}", API_V1, fake_uid), None).await;
    assert_eq!(status, 404, "Getting non-existent page must return 404");
}

/// Test that updating non-existent post must fail
#[tokio::test]
async fn test_must_fail_update_nonexistent_post() {
    let mut router = test_router();
    let fake_uid = "fake-update-uid-99999";
    
    let update = json!({
        "title": "This should not work"
    });
    
    let (status, _) = make_request(&mut router, "PUT", &format!("{}/posts/uid/{}", API_V1, fake_uid), Some(update)).await;
    assert_eq!(status, 404, "Updating non-existent post must return 404");
}

/// Test that deleting non-existent post must fail
#[tokio::test]
async fn test_must_fail_delete_nonexistent_post() {
    let mut router = test_router();
    let fake_uid = "fake-delete-uid-88888";
    
    let (status, _) = make_request(&mut router, "DELETE", &format!("{}/posts/uid/{}", API_V1, fake_uid), None).await;
    assert_eq!(status, 404, "Deleting non-existent post must return 404");
}

/// Test that creating post without required 'kind' field must fail
#[tokio::test]
async fn test_must_fail_create_without_kind() {
    let mut router = test_router();
    
    let bad_post = json!({
        "title": "No kind post",
        "body": "This post has no kind",
        "published": true
    });
    
    let (status, _) = make_request(&mut router, "POST", &format!("{}/posts", API_V1), Some(bad_post)).await;
    // API returns 422 for deserialization/validation errors
    assert!(
        status >= 400 && status < 500,
        "Creating post without kind must return 4xx, got {}",
        status
    );
}

/// Test that creating post with empty title must fail
#[tokio::test]
async fn test_must_fail_create_with_empty_title() {
    let mut router = test_router();
    
    let bad_post = json!({
        "title": "",
        "kind": "post",
        "body": "This post has empty title",
        "published": true
    });
    
    let (status, _) = make_request(&mut router, "POST", &format!("{}/posts", API_V1), Some(bad_post)).await;
    // API returns 422 for validation errors
    assert!(
        status >= 400 && status < 500,
        "Creating post with empty title must return 4xx, got {}",
        status
    );
}

/// Test that accessing deleted post must fail
#[tokio::test]
async fn test_must_fail_access_deleted_post() {
    let mut router = test_router();
    let (uid, _) = create_test_post(&mut router, true).await;
    
    // Delete the post
    let (status, _) = make_request(&mut router, "DELETE", &format!("{}/posts/uid/{}", API_V1, uid), None).await;
    assert_eq!(status, 200, "Delete must succeed first");
    
    // Try to access deleted post
    let (status, _) = make_request(&mut router, "GET", &format!("{}/posts/uid/{}", API_V1, uid), None).await;
    assert_eq!(status, 404, "Accessing deleted post must return 404");
    
    // Cleanup already done: post is deleted
}

/// Test that double-deleting returns 404 on second attempt
#[tokio::test]
async fn test_must_fail_double_delete() {
    let mut router = test_router();
    let (uid, _) = create_test_post(&mut router, true).await;
    
    // First delete
    let (status, _) = make_request(&mut router, "DELETE", &format!("{}/posts/uid/{}", API_V1, &uid), None).await;
    assert_eq!(status, 200, "First delete must succeed");
    
    // Second delete
    let (status, _) = make_request(&mut router, "DELETE", &format!("{}/posts/uid/{}", API_V1, &uid), None).await;
    assert_eq!(status, 404, "Double-deleting must return 404");
}

// =============================================================================
// SELF-CONTAINED CRUD CYCLE TESTS (each test creates its own data and cleans up)
// =============================================================================

/// Complete CRUD lifecycle for a single post - ensures creation, reading, update, and deletion work
/// Note: GET /posts/uid/{uid} only returns published posts, so we create as published initially
/// This is necessary because the API filters unpublished content from individual retrieval
#[tokio::test]
async fn test_post_crud_cycle_isolation() {
    let mut router = test_router();
    
    // 1. CREATE (as published since GET endpoint filters unpublished)
    let new_post = json!({
        "title": "CRUD Cycle Test Post",
        "kind": "post",
        "body": "Initial body",
        "tags": ["crud", "test"],
        "published": true
    });
    let (status, created) = make_request(&mut router, "POST", &format!("{}/posts", API_V1), Some(new_post)).await;
    assert_eq!(status, 200, "Create must succeed");
    let uid = created["uid"].as_str().unwrap().to_string();
    
    // 2. READ (verify creation - this works because post is published)
    let (status, read1) = make_request(&mut router, "GET", &format!("{}/posts/uid/{}", API_V1, uid), None).await;
    assert_eq!(status, 200, "Read after create must succeed");
    assert_eq!(read1["title"], "CRUD Cycle Test Post");
    assert_eq!(read1["published"], true, "Post must be published as created");
    
    // 3. UPDATE (testing that both content and publish status can be changed)
    let update = json!({
        "title": "Updated CRUD Title",
        "body": "Updated body",
        "published": false
    });
    let (status, updated) = make_request(&mut router, "PUT", &format!("{}/posts/uid/{}", API_V1, uid), Some(update)).await;
    assert_eq!(status, 200, "Update must succeed");
    assert_eq!(updated["title"], "Updated CRUD Title");
    assert_eq!(updated["published"], false, "Post must be unpublished after update");
    
    // 3b. VERIFY UNPUBLISHED - Read via admin/all endpoint (GET /posts/uid/{uid} won't return unpublished)
    // We use list endpoint to verify the update persisted
    let (status, list) = make_request(&mut router, "GET", &format!("{}/posts/all", API_V1), None).await;
    assert_eq!(status, 200, "List must succeed");
    let found = list.as_array().unwrap().iter().any(|p| p["uid"] == uid && p["published"] == false);
    assert!(found, "Unpublished post must be visible in all posts list");
    
    // Re-publish for deletion test
    let republish = json!({ "published": true });
    let (status, _) = make_request(&mut router, "PUT", &format!("{}/posts/uid/{}", API_V1, uid), Some(republish)).await;
    assert_eq!(status, 200, "Re-publish must succeed");
    
    // 4. READ (verify republish persisted)
    let (status, read2) = make_request(&mut router, "GET", &format!("{}/posts/uid/{}", API_V1, uid), None).await;
    assert_eq!(status, 200, "Read after re-publish must succeed");
    assert_eq!(read2["title"], "Updated CRUD Title");
    assert_eq!(read2["body"], "Updated body");
    assert_eq!(read2["published"], true, "Post must be published after re-publish");
    
    // 5. DELETE
    let (status, _) = make_request(&mut router, "DELETE", &format!("{}/posts/uid/{}", API_V1, &uid), None).await;
    assert_eq!(status, 200, "Delete must succeed");
    
    // 6. VERIFY GONE
    let (status, _) = make_request(&mut router, "GET", &format!("{}/posts/uid/{}", API_V1, uid), None).await;
    assert_eq!(status, 404, "Post must be gone after delete");
}
/// Complete lifecycle for page with same pattern
/// Note: GET /pages/uid/{uid} only returns published pages, so we create as published
#[tokio::test]
async fn test_page_crud_cycle_isolation() {
    let mut router = test_router();
    
    // 1. CREATE (as published since GET endpoint filters unpublished)
    let new_page = json!({
        "title": "CRUD Cycle Test Page",
        "kind": "page",
        "body": "Initial page body",
        "tags": ["isolated", "crud"],
        "published": true
    });
    let (status, created) = make_request(&mut router, "POST", &format!("{}/pages", API_V1), Some(new_page)).await;
    assert_eq!(status, 200, "Create page must succeed");
    let uid = created["uid"].as_str().unwrap().to_string();
    
    // 2. READ (verify creation - this works because page is published)
    let (status, read) = make_request(&mut router, "GET", &format!("{}/pages/uid/{}", API_V1, uid), None).await;
    assert_eq!(status, 200, "Read page must succeed");
    assert_eq!(read["kind"], "page");
    assert_eq!(read["published"], true, "Page must be published as created");
    
    // 3. UPDATE
    let update = json!({ 
        "title": "Updated Page Title",
        "body": "Updated page body"
    });
    let (status, updated) = make_request(&mut router, "PUT", &format!("{}/pages/uid/{}", API_V1, uid), Some(update)).await;
    assert_eq!(status, 200, "Update page must succeed");
    assert_eq!(updated["title"], "Updated Page Title");
    assert_eq!(updated["body"], "Updated page body");
    assert_eq!(updated["published"], true, "Page must remain published");
    
    // 4. DELETE
    let (status, _) = make_request(&mut router, "DELETE", &format!("{}/pages/uid/{}", API_V1, &uid), None).await;
    assert_eq!(status, 200, "Delete page must succeed");
    
    // 5. VERIFY GONE
    let (status, _) = make_request(&mut router, "GET", &format!("{}/pages/uid/{}", API_V1, uid), None).await;
    assert_eq!(status, 404, "Page must be gone after delete");
}

/// Test that multiple posts can exist independently
#[tokio::test]
async fn test_multiple_posts_isolation() {
    let mut router = test_router();
    
    // Create multiple independent posts (all published so they can be retrieved via /posts/uid/{uid})
    let (uid1, post1) = create_test_post(&mut router, true).await;
    let (uid2, _post2) = create_test_post(&mut router, true).await;
    let (uid3, post3) = create_test_post(&mut router, true).await;
    
    // Update only post2
    let update = json!({ "title": "Only Post2 Modified" });
    let (status, _) = make_request(&mut router, "PUT", &format!("{}/posts/uid/{}", API_V1, &uid2), Some(update)).await;
    assert_eq!(status, 200);
    
    // Verify post1 unchanged
    let (status, check1) = make_request(&mut router, "GET", &format!("{}/posts/uid/{}", API_V1, &uid1), None).await;
    assert_eq!(status, 200);
    assert_eq!(check1["title"], post1["title"], "Post1 must be unchanged");
    
    // Verify post2 changed
    let (status, check2) = make_request(&mut router, "GET", &format!("{}/posts/uid/{}", API_V1, &uid2), None).await;
    assert_eq!(status, 200);
    assert_eq!(check2["title"], "Only Post2 Modified", "Post2 must be changed");
    
    // Verify post3 unchanged
    let (status, check3) = make_request(&mut router, "GET", &format!("{}/posts/uid/{}", API_V1, &uid3), None).await;
    assert_eq!(status, 200);
    assert_eq!(check3["title"], post3["title"], "Post3 must be unchanged");
    
    // Cleanup all
    delete_post(&mut router, &uid1).await;
    delete_post(&mut router, &uid2).await;
    delete_post(&mut router, &uid3).await;
}

/// Test that operations on wrong endpoint fail correctly (post on page endpoint)
#[tokio::test]
async fn test_endpoint_validation_must_fail() {
    let mut router = test_router();
    
    // Create a post
    let (post_uid, _) = create_test_post(&mut router, true).await;
    
    // Try to access post through pages endpoint - must fail
    let (status, _) = make_request(&mut router, "GET", &format!("{}/pages/uid/{}", API_V1, &post_uid), None).await;
    assert_eq!(status, 404, "Accessing post through pages endpoint must fail");
    
    // Try to update post through pages endpoint - must fail
    let update = json!({ "title": "Wrong endpoint" });
    let (status, _) = make_request(&mut router, "PUT", &format!("{}/pages/uid/{}", API_V1, &post_uid), Some(update)).await;
    assert_eq!(status, 404, "Updating post through pages endpoint must fail");
    
    // Post still accessible through correct endpoint
    let (status, _) = make_request(&mut router, "GET", &format!("{}/posts/uid/{}", API_V1, &post_uid), None).await;
    assert_eq!(status, 200, "Post must still be accessible through correct endpoint");
    
    delete_post(&mut router, &post_uid).await;
}
