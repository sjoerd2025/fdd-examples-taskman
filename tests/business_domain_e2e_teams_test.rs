// @fdd-test:fdd-fdd-example-feature-business-domain-test-team-create-list:ph-1

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
async fn create_team_then_list_includes_it_and_only_visible_teams_returned() {
    let openapi = fdd_example::openapi::load_openapi("api/openapi.yaml")
        .await
        .unwrap();

    let pool = fdd_example::db::connect_sqlite("sqlite::memory:")
        .await
        .unwrap();
    fdd_example::db::init_schema(&pool).await.unwrap();

    let app = fdd_example::app::build_app(pool.clone(), openapi);

    let owner_token = issue_token(app.clone(), "owner@example.com").await;
    let outsider_token = issue_token(app.clone(), "outsider@example.com").await;

    // Create team
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
    let created: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let team_id = created.get("id").unwrap().as_str().unwrap().to_string();

    // Owner sees it
    let req = Request::builder()
        .method("GET")
        .uri("/teams")
        .header("authorization", format!("Bearer {owner_token}"))
        .body(Body::empty())
        .unwrap();

    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status().as_u16(), 200);

    let body = resp.into_body().collect().await.unwrap().to_bytes();
    let list: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let items = list.get("items").unwrap().as_array().unwrap();
    assert!(items
        .iter()
        .any(|t| t.get("id").unwrap().as_str().unwrap() == team_id));

    // Outsider sees nothing
    let req = Request::builder()
        .method("GET")
        .uri("/teams")
        .header("authorization", format!("Bearer {outsider_token}"))
        .body(Body::empty())
        .unwrap();

    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status().as_u16(), 200);

    let body = resp.into_body().collect().await.unwrap().to_bytes();
    let list: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let items = list.get("items").unwrap().as_array().unwrap();
    assert_eq!(items.len(), 0);
}
