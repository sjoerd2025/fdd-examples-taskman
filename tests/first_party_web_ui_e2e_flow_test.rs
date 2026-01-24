// @fdd-test:fdd-fdd-example-feature-first-party-web-ui-test-authenticate:ph-1
// @fdd-test:fdd-fdd-example-feature-first-party-web-ui-test-browse-tasks:ph-1
// @fdd-test:fdd-fdd-example-feature-first-party-web-ui-test-error-envelope:ph-1

use axum::{body::Body, http::Request};
use chrono::Utc;
use http_body_util::BodyExt;
use tower::ServiceExt;
use uuid::Uuid;

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
    v.get("access_token")
        .unwrap()
        .as_str()
        .unwrap()
        .to_string()
}

#[tokio::test]
async fn authenticate_and_establish_ui_session() {
    // @fdd-flow:fdd-fdd-example-feature-first-party-web-ui-flow-authenticate:ph-1

    let openapi = fdd_example::openapi::load_openapi("api/openapi.yaml")
        .await
        .unwrap();
    let pool = fdd_example::db::connect_sqlite("sqlite::memory:")
        .await
        .unwrap();
    fdd_example::db::init_schema(&pool).await.unwrap();
    let app = fdd_example::app::build_app(pool, openapi);

    // fdd-begin fdd-fdd-example-feature-first-party-web-ui-test-authenticate:ph-1:inst-start-anon
    let req = Request::builder()
        .method("GET")
        .uri("/")
        .body(Body::empty())
        .unwrap();
    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status().as_u16(), 200);
    // fdd-end fdd-fdd-example-feature-first-party-web-ui-test-authenticate:ph-1:inst-start-anon

    // fdd-begin fdd-fdd-example-feature-first-party-web-ui-test-authenticate:ph-1:inst-submit-valid-credentials
    let token = issue_token(app.clone(), "user-a@example.com").await;
    // fdd-end fdd-fdd-example-feature-first-party-web-ui-test-authenticate:ph-1:inst-submit-valid-credentials

    // fdd-begin fdd-fdd-example-feature-first-party-web-ui-test-authenticate:ph-1:inst-verify-task-list-entry
    let req = Request::builder()
        .method("GET")
        .uri("/tasks")
        .header("authorization", format!("Bearer {token}"))
        .body(Body::empty())
        .unwrap();
    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status().as_u16(), 200);
    // fdd-end fdd-fdd-example-feature-first-party-web-ui-test-authenticate:ph-1:inst-verify-task-list-entry
}

#[tokio::test]
async fn browse_tasks_with_filters_and_retry() {
    // @fdd-flow:fdd-fdd-example-feature-first-party-web-ui-flow-browse-tasks:ph-1

    let openapi = fdd_example::openapi::load_openapi("api/openapi.yaml")
        .await
        .unwrap();
    let pool = fdd_example::db::connect_sqlite("sqlite::memory:")
        .await
        .unwrap();
    fdd_example::db::init_schema(&pool).await.unwrap();
    let app = fdd_example::app::build_app(pool, openapi);
    let token = issue_token(app.clone(), "user-b@example.com").await;

    let team_payload = serde_json::json!({"name": "Alpha"});
    let req = Request::builder()
        .method("POST")
        .uri("/teams")
        .header("authorization", format!("Bearer {token}"))
        .header("content-type", "application/json")
        .body(Body::from(team_payload.to_string()))
        .unwrap();
    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status().as_u16(), 201);
    let body = resp.into_body().collect().await.unwrap().to_bytes();
    let team: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let team_id = team.get("id").unwrap().as_str().unwrap();

    let task_payload = serde_json::json!({"title": "Team task", "team_id": team_id});
    let req = Request::builder()
        .method("POST")
        .uri("/tasks")
        .header("authorization", format!("Bearer {token}"))
        .header("content-type", "application/json")
        .body(Body::from(task_payload.to_string()))
        .unwrap();
    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status().as_u16(), 201);
    let body = resp.into_body().collect().await.unwrap().to_bytes();
    let created: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let task_id = created.get("id").unwrap().as_str().unwrap();

    let update_payload = serde_json::json!({"status": "in-progress"});
    let req = Request::builder()
        .method("PATCH")
        .uri(format!("/tasks/{task_id}"))
        .header("authorization", format!("Bearer {token}"))
        .header("content-type", "application/json")
        .body(Body::from(update_payload.to_string()))
        .unwrap();
    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status().as_u16(), 200);

    // fdd-begin fdd-fdd-example-feature-first-party-web-ui-test-browse-tasks:ph-1:inst-open-with-token
    let req = Request::builder()
        .method("GET")
        .uri("/tasks")
        .header("authorization", format!("Bearer {token}"))
        .body(Body::empty())
        .unwrap();
    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status().as_u16(), 200);
    // fdd-end fdd-fdd-example-feature-first-party-web-ui-test-browse-tasks:ph-1:inst-open-with-token

    // fdd-begin fdd-fdd-example-feature-first-party-web-ui-test-browse-tasks:ph-1:inst-apply-filters
    let req = Request::builder()
        .method("GET")
        .uri(format!(
            "/tasks?team_id={team_id}&status=in-progress"
        ))
        .header("authorization", format!("Bearer {token}"))
        .body(Body::empty())
        .unwrap();
    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status().as_u16(), 200);
    // fdd-end fdd-fdd-example-feature-first-party-web-ui-test-browse-tasks:ph-1:inst-apply-filters

    // fdd-begin fdd-fdd-example-feature-first-party-web-ui-test-browse-tasks:ph-1:inst-verify-render-preserve
    let body = resp.into_body().collect().await.unwrap().to_bytes();
    let list: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let items = list.get("items").unwrap().as_array().unwrap();
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].get("status").unwrap().as_str().unwrap(), "in-progress");
    // fdd-end fdd-fdd-example-feature-first-party-web-ui-test-browse-tasks:ph-1:inst-verify-render-preserve
}

#[tokio::test]
async fn handle_error_envelope_consistently() {
    // @fdd-flow:fdd-fdd-example-feature-first-party-web-ui-flow-browse-tasks:ph-1

    let openapi = fdd_example::openapi::load_openapi("api/openapi.yaml")
        .await
        .unwrap();
    let pool = fdd_example::db::connect_sqlite("sqlite::memory:")
        .await
        .unwrap();
    fdd_example::db::init_schema(&pool).await.unwrap();
    let app = fdd_example::app::build_app(pool.clone(), openapi);
    let token = issue_token(app.clone(), "user-c@example.com").await;

    // fdd-begin fdd-fdd-example-feature-first-party-web-ui-test-error-envelope:ph-1:inst-trigger-400
    let req = Request::builder()
        .method("GET")
        .uri("/tasks?status=invalid")
        .header("authorization", format!("Bearer {token}"))
        .body(Body::empty())
        .unwrap();
    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status().as_u16(), 400);
    // fdd-end fdd-fdd-example-feature-first-party-web-ui-test-error-envelope:ph-1:inst-trigger-400

    // fdd-begin fdd-fdd-example-feature-first-party-web-ui-test-error-envelope:ph-1:inst-trigger-401
    let req = Request::builder()
        .method("GET")
        .uri("/tasks")
        .body(Body::empty())
        .unwrap();
    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status().as_u16(), 401);
    // fdd-end fdd-fdd-example-feature-first-party-web-ui-test-error-envelope:ph-1:inst-trigger-401

    let now = Utc::now().to_rfc3339();
    let other_task = fdd_example::db::TaskRecord {
        id: Uuid::new_v4().to_string(),
        title: "Other user task".into(),
        description: None,
        status: "to-do".into(),
        due_at: None,
        assignee_user_id: None,
        team_id: None,
        owner_user_id: "someone-else".into(),
        created_at: now.clone(),
        updated_at: now,
    };
    fdd_example::db::insert_task(&pool, &other_task).await.unwrap();

    // fdd-begin fdd-fdd-example-feature-first-party-web-ui-test-error-envelope:ph-1:inst-trigger-403
    let req = Request::builder()
        .method("GET")
        .uri(format!("/tasks/{}", other_task.id))
        .header("authorization", format!("Bearer {token}"))
        .body(Body::empty())
        .unwrap();
    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status().as_u16(), 403);
    // fdd-end fdd-fdd-example-feature-first-party-web-ui-test-error-envelope:ph-1:inst-trigger-403
}
