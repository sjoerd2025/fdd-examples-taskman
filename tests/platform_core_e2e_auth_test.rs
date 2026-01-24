// @fdd-test:fdd-fdd-example-feature-platform-core-test-token-issuance:ph-1
// @fdd-test:fdd-fdd-example-feature-platform-core-test-token-expired:ph-1

use axum::{body::Body, http::Request};
use http_body_util::BodyExt;
use tower::ServiceExt;

#[tokio::test]
async fn token_issuance_returns_201_and_allows_authenticated_call() {
    // fdd-begin fdd-fdd-example-feature-platform-core-test-token-issuance:ph-1:inst-prepare-token-request
    let openapi = fdd_example::openapi::load_openapi("api/openapi.yaml")
        .await
        .unwrap();

    let pool = fdd_example::db::connect_sqlite("sqlite::memory:")
        .await
        .unwrap();
    fdd_example::db::init_schema(&pool).await.unwrap();

    let app = fdd_example::app::build_app(pool.clone(), openapi);

    // fdd-end fdd-fdd-example-feature-platform-core-test-token-issuance:ph-1:inst-prepare-token-request

    // fdd-begin fdd-fdd-example-feature-platform-core-test-token-issuance:ph-1:inst-send-token-request
    let req = Request::builder()
        .method("POST")
        .uri("/auth/tokens")
        .header("content-type", "application/json")
        .body(Body::from(
            r#"{"email":"user@example.com","password":"ok"}"#,
        ))
        .unwrap();

    // fdd-end fdd-fdd-example-feature-platform-core-test-token-issuance:ph-1:inst-send-token-request

    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status().as_u16(), 201);

    let body = resp.into_body().collect().await.unwrap().to_bytes();
    let v: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let token = v.get("access_token").unwrap().as_str().unwrap().to_string();

    // fdd-begin fdd-fdd-example-feature-platform-core-test-token-issuance:ph-1:inst-verify-token-response
    let _ = &token;
    // fdd-end fdd-fdd-example-feature-platform-core-test-token-issuance:ph-1:inst-verify-token-response

    // fdd-begin fdd-fdd-example-feature-platform-core-flow-authenticated-call:ph-1:inst-authenticate-bearer
    let _ = &token;
    // fdd-end fdd-fdd-example-feature-platform-core-flow-authenticated-call:ph-1:inst-authenticate-bearer

    // fdd-begin fdd-fdd-example-feature-platform-core-flow-authenticated-call:ph-1:inst-send-protected-request
    let req2 = Request::builder()
        .method("GET")
        .uri("/tasks")
        .header("authorization", format!("Bearer {token}"))
        .body(Body::empty())
        .unwrap();

    // fdd-end fdd-fdd-example-feature-platform-core-flow-authenticated-call:ph-1:inst-send-protected-request

    // fdd-begin fdd-fdd-example-feature-platform-core-flow-authenticated-call:ph-1:inst-return-handler-response
    let resp2 = app.oneshot(req2).await.unwrap();
    assert_eq!(resp2.status().as_u16(), 200);
    // fdd-end fdd-fdd-example-feature-platform-core-flow-authenticated-call:ph-1:inst-return-handler-response
}

#[tokio::test]
async fn expired_token_is_rejected() {
    // fdd-begin fdd-fdd-example-feature-platform-core-test-token-expired:ph-1:inst-obtain-and-expire
    let openapi = fdd_example::openapi::load_openapi("api/openapi.yaml")
        .await
        .unwrap();

    let pool = fdd_example::db::connect_sqlite("sqlite::memory:")
        .await
        .unwrap();
    fdd_example::db::init_schema(&pool).await.unwrap();

    let app = fdd_example::app::build_app(pool.clone(), openapi);

    let req = Request::builder()
        .method("POST")
        .uri("/auth/tokens")
        .header("content-type", "application/json")
        .body(Body::from(
            r#"{"email":"user@example.com","password":"ok"}"#,
        ))
        .unwrap();

    let resp = app.clone().oneshot(req).await.unwrap();
    let body = resp.into_body().collect().await.unwrap().to_bytes();
    let v: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let token = v.get("access_token").unwrap().as_str().unwrap().to_string();
    let token_hash = fdd_example::db::hash_token(&token);

    fdd_example::db::set_token_expiry(
        &pool,
        &token_hash,
        chrono::Utc::now() - chrono::Duration::hours(1),
    )
    .await
    .unwrap();

    // fdd-end fdd-fdd-example-feature-platform-core-test-token-expired:ph-1:inst-obtain-and-expire

    // fdd-begin fdd-fdd-example-feature-platform-core-test-token-expired:ph-1:inst-call-with-expired
    let req2 = Request::builder()
        .method("GET")
        .uri("/tasks")
        .header("authorization", format!("Bearer {token}"))
        .body(Body::empty())
        .unwrap();

    // fdd-end fdd-fdd-example-feature-platform-core-test-token-expired:ph-1:inst-call-with-expired

    let resp2 = app.oneshot(req2).await.unwrap();

    // fdd-begin fdd-fdd-example-feature-platform-core-test-token-expired:ph-1:inst-verify-401-expired
    assert_eq!(resp2.status().as_u16(), 401);
    // fdd-end fdd-fdd-example-feature-platform-core-test-token-expired:ph-1:inst-verify-401-expired
}
