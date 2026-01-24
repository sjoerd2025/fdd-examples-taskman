// @fdd-test:fdd-fdd-example-feature-platform-core-test-openapi-requests:ph-1

use axum::{body::Body, http::Request};
use http_body_util::BodyExt;
use tower::ServiceExt;

#[tokio::test]
async fn invalid_request_returns_400_with_error_envelope() {
    // fdd-begin fdd-fdd-example-feature-platform-core-test-openapi-requests:ph-1:inst-prepare-invalid-request
    let openapi = fdd_example::openapi::load_openapi("api/openapi.yaml")
        .await
        .unwrap();

    let pool = fdd_example::db::connect_sqlite("sqlite::memory:")
        .await
        .unwrap();
    fdd_example::db::init_schema(&pool).await.unwrap();

    let app = fdd_example::app::build_app(pool, openapi);

    // fdd-end fdd-fdd-example-feature-platform-core-test-openapi-requests:ph-1:inst-prepare-invalid-request

    // fdd-begin fdd-fdd-example-feature-platform-core-test-openapi-requests:ph-1:inst-send-invalid-request
    let req = Request::builder()
        .method("POST")
        .uri("/auth/tokens")
        .header("content-type", "application/json")
        .body(Body::from(r#"{"email":"","password":"test"}"#))
        .unwrap();

    // fdd-end fdd-fdd-example-feature-platform-core-test-openapi-requests:ph-1:inst-send-invalid-request

    let resp = app.oneshot(req).await.unwrap();

    // fdd-begin fdd-fdd-example-feature-platform-core-test-openapi-requests:ph-1:inst-verify-400-shape
    assert_eq!(resp.status().as_u16(), 400);

    let body = resp.into_body().collect().await.unwrap().to_bytes();
    let v: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert!(v.get("code").is_some());
    assert!(v.get("message").is_some());
    // fdd-end fdd-fdd-example-feature-platform-core-test-openapi-requests:ph-1:inst-verify-400-shape
}
