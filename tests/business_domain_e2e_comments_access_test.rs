// @fdd-test:fdd-fdd-example-feature-business-domain-test-comment-access:ph-1

use axum::{body::Body, http::Request};
use http_body_util::BodyExt;
use sha2::{Digest, Sha256};
use tower::ServiceExt;
use uuid::Uuid;

fn stable_user_id(email: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(email.as_bytes());
    let digest = hasher.finalize();

    let mut bytes = [0u8; 16];
    bytes.copy_from_slice(&digest[..16]);
    Uuid::from_bytes(bytes).to_string()
}

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
async fn comments_enforce_task_access() {
    let openapi = fdd_example::openapi::load_openapi("api/openapi.yaml")
        .await
        .unwrap();

    let pool = fdd_example::db::connect_sqlite("sqlite::memory:")
        .await
        .unwrap();
    fdd_example::db::init_schema(&pool).await.unwrap();

    let app = fdd_example::app::build_app(pool.clone(), openapi);

    let owner_email = "owner@example.com";
    let member_email = "member@example.com";

    let owner_token = issue_token(app.clone(), owner_email).await;
    let member_token = issue_token(app.clone(), member_email).await;

    // Create team as owner
    let req = Request::builder()
        .method("POST")
        .uri("/teams")
        .header("authorization", format!("Bearer {owner_token}"))
        .header("content-type", "application/json")
        .body(Body::from(r#"{"name":"Team A"}"#))
        .unwrap();

    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status().as_u16(), 201);

    let body = resp.into_body().collect().await.unwrap().to_bytes();
    let team: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let team_id = team.get("id").unwrap().as_str().unwrap().to_string();

    // Create team task
    let req = Request::builder()
        .method("POST")
        .uri("/tasks")
        .header("authorization", format!("Bearer {owner_token}"))
        .header("content-type", "application/json")
        .body(Body::from(format!(
            r#"{{"title":"t1","team_id":"{team_id}"}}"#
        )))
        .unwrap();

    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status().as_u16(), 201);

    let body = resp.into_body().collect().await.unwrap().to_bytes();
    let task: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let task_id = task.get("id").unwrap().as_str().unwrap().to_string();

    // Non-member cannot comment
    let req = Request::builder()
        .method("POST")
        .uri(format!("/tasks/{task_id}/comments"))
        .header("authorization", format!("Bearer {member_token}"))
        .header("content-type", "application/json")
        .body(Body::from(r#"{"body":"hello"}"#))
        .unwrap();

    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status().as_u16(), 403);

    // Add membership fixture
    let member_id = stable_user_id(member_email);
    fdd_example::db::insert_team_membership(&pool, &team_id, &member_id, "member")
        .await
        .unwrap();

    // Member can comment
    let req = Request::builder()
        .method("POST")
        .uri(format!("/tasks/{task_id}/comments"))
        .header("authorization", format!("Bearer {member_token}"))
        .header("content-type", "application/json")
        .body(Body::from(r#"{"body":"hello"}"#))
        .unwrap();

    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status().as_u16(), 201);

    // List includes the comment
    let req = Request::builder()
        .method("GET")
        .uri(format!("/tasks/{task_id}/comments"))
        .header("authorization", format!("Bearer {member_token}"))
        .body(Body::empty())
        .unwrap();

    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status().as_u16(), 200);

    let body = resp.into_body().collect().await.unwrap().to_bytes();
    let list: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert!(!list.get("items").unwrap().as_array().unwrap().is_empty());
}
