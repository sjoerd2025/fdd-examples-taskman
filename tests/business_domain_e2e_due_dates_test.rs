// @fdd-test:fdd-fdd-example-feature-business-domain-test-due-date-update:ph-1

use axum::{body::Body, http::Request};
use http_body_util::BodyExt;
use tower::ServiceExt;

async fn issue_token(app: axum::Router, email: &str) -> String {
    let payload = serde_json::json!({"email": email, "password": "ok"});
    let req = Request::builder()
        .method("POST")
        .uri("/auth/tokens")
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap();

    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status().as_u16(), 201);

    let body = resp.into_body().collect().await.unwrap().to_bytes();
    let v: serde_json::Value = serde_json::from_slice(&body).unwrap();
    v.get("access_token").unwrap().as_str().unwrap().to_string()
}

#[tokio::test]
async fn due_date_update_and_clear() {
    let openapi = fdd_example::openapi::load_openapi("api/openapi.yaml")
        .await
        .unwrap();

    let pool = fdd_example::db::connect_sqlite("sqlite::memory:")
        .await
        .unwrap();
    fdd_example::db::init_schema(&pool).await.unwrap();

    let app = fdd_example::app::build_app(pool.clone(), openapi);
    let token = issue_token(app.clone(), "user@example.com").await;

    // Create task
    let req = Request::builder()
        .method("POST")
        .uri("/tasks")
        .header("authorization", format!("Bearer {token}"))
        .header("content-type", "application/json")
        .body(Body::from(r#"{"title":"t1"}"#))
        .unwrap();

    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status().as_u16(), 201);

    let body = resp.into_body().collect().await.unwrap().to_bytes();
    let created: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let task_id = created.get("id").unwrap().as_str().unwrap().to_string();

    // Set due date (with timezone offset, should normalize to UTC)
    let req = Request::builder()
        .method("PATCH")
        .uri(format!("/tasks/{task_id}"))
        .header("authorization", format!("Bearer {token}"))
        .header("content-type", "application/json")
        .body(Body::from(r#"{"due_at":"2030-01-01T12:00:00+03:00"}"#))
        .unwrap();

    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status().as_u16(), 200);

    let body = resp.into_body().collect().await.unwrap().to_bytes();
    let updated: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let due_at = updated.get("due_at").unwrap().as_str().unwrap();
    assert!(due_at.starts_with("2030-01-01T09:00:00"));

    // Clear due date
    let req = Request::builder()
        .method("PATCH")
        .uri(format!("/tasks/{task_id}"))
        .header("authorization", format!("Bearer {token}"))
        .header("content-type", "application/json")
        .body(Body::from(r#"{"due_at":null}"#))
        .unwrap();

    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status().as_u16(), 200);

    let body = resp.into_body().collect().await.unwrap().to_bytes();
    let cleared: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert!(cleared.get("due_at").unwrap().is_null());
}
