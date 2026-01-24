// @fdd-test:fdd-fdd-example-feature-business-domain-test-task-crud:ph-1

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
async fn task_crud_create_update_status_delete() {
    let openapi = fdd_example::openapi::load_openapi("api/openapi.yaml")
        .await
        .unwrap();

    let pool = fdd_example::db::connect_sqlite("sqlite::memory:")
        .await
        .unwrap();
    fdd_example::db::init_schema(&pool).await.unwrap();

    let app = fdd_example::app::build_app(pool.clone(), openapi);
    let token = issue_token(app.clone(), "user-a@example.com").await;

    // Create task
    let req = Request::builder()
        .method("POST")
        .uri("/tasks")
        .header("authorization", format!("Bearer {token}"))
        .header("content-type", "application/json")
        .body(Body::from(r#"{"title":"t1","description":"d1"}"#))
        .unwrap();

    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status().as_u16(), 201);

    let body = resp.into_body().collect().await.unwrap().to_bytes();
    let created: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let task_id = created.get("id").unwrap().as_str().unwrap().to_string();
    assert_eq!(created.get("status").unwrap().as_str().unwrap(), "to-do");

    // Update status to in-progress
    let req = Request::builder()
        .method("PATCH")
        .uri(format!("/tasks/{task_id}"))
        .header("authorization", format!("Bearer {token}"))
        .header("content-type", "application/json")
        .body(Body::from(r#"{"status":"in-progress"}"#))
        .unwrap();

    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status().as_u16(), 200);

    let body = resp.into_body().collect().await.unwrap().to_bytes();
    let updated: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(
        updated.get("status").unwrap().as_str().unwrap(),
        "in-progress"
    );

    // Delete task
    let req = Request::builder()
        .method("DELETE")
        .uri(format!("/tasks/{task_id}"))
        .header("authorization", format!("Bearer {token}"))
        .body(Body::empty())
        .unwrap();

    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status().as_u16(), 204);

    // Verify 404
    let req = Request::builder()
        .method("GET")
        .uri(format!("/tasks/{task_id}"))
        .header("authorization", format!("Bearer {token}"))
        .body(Body::empty())
        .unwrap();

    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status().as_u16(), 404);
}
