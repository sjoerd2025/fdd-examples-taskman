// @fdd-test:fdd-fdd-example-feature-first-party-web-ui-test-ui-performance:ph-1

use axum::{body::Body, http::Request};
use http_body_util::BodyExt;
use tower::ServiceExt;

#[tokio::test]
async fn ui_root_served_as_html_and_assets_have_cache_headers() {
    // @fdd-flow:fdd-fdd-example-feature-first-party-web-ui-flow-authenticate:ph-1
    // @fdd-algo:fdd-fdd-example-feature-first-party-web-ui-algo-apply-ui-caching:ph-1

    // fdd-begin fdd-fdd-example-feature-first-party-web-ui-test-authenticate:ph-1:inst-start-anon
    // fdd-begin fdd-fdd-example-feature-first-party-web-ui-flow-authenticate:ph-1:inst-open-entry
    let openapi = fdd_example::openapi::load_openapi("api/openapi.yaml")
        .await
        .unwrap();

    let pool = fdd_example::db::connect_sqlite("sqlite::memory:")
        .await
        .unwrap();
    fdd_example::db::init_schema(&pool).await.unwrap();

    let app = fdd_example::app::build_app(pool, openapi);

    let req = Request::builder()
        .method("GET")
        .uri("/")
        .body(Body::empty())
        .unwrap();

    let resp = app.clone().oneshot(req).await.unwrap();
    // fdd-end fdd-fdd-example-feature-first-party-web-ui-flow-authenticate:ph-1:inst-open-entry
    // fdd-end fdd-fdd-example-feature-first-party-web-ui-test-authenticate:ph-1:inst-start-anon

    assert_eq!(resp.status().as_u16(), 200);

    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    assert!(content_type.contains("text/html"));

    let body = resp.into_body().collect().await.unwrap().to_bytes();
    let html = String::from_utf8_lossy(&body);
    let asset_href = html
        .split("href=\"/assets/")
        .nth(1)
        .and_then(|tail| tail.split('"').next())
        .unwrap_or("");
    assert!(!asset_href.is_empty());

    // fdd-begin fdd-fdd-example-feature-first-party-web-ui-test-ui-performance:ph-1:inst-check-asset-cache
    // fdd-begin fdd-fdd-example-feature-first-party-web-ui-algo-apply-ui-caching:ph-1:inst-use-cache-headers
    let req_assets = Request::builder()
        .method("GET")
        .uri(format!("/assets/{asset_href}"))
        .body(Body::empty())
        .unwrap();

    let resp_assets = app.oneshot(req_assets).await.unwrap();
    // fdd-end fdd-fdd-example-feature-first-party-web-ui-algo-apply-ui-caching:ph-1:inst-use-cache-headers
    // fdd-end fdd-fdd-example-feature-first-party-web-ui-test-ui-performance:ph-1:inst-check-asset-cache

    assert_eq!(resp_assets.status().as_u16(), 200);

    let cache_control = resp_assets
        .headers()
        .get("cache-control")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    assert!(cache_control.contains("max-age"));

    let body = resp_assets.into_body().collect().await.unwrap().to_bytes();
    assert!(!body.is_empty());
}
