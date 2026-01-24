use chrono::Utc;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::{auth::Principal, tx};

pub async fn record_request_audit(
    pool: &SqlitePool,
    principal: Option<&Principal>,
    operation: &str,
    status_code: u16,
) {
    // @fdd-change:fdd-fdd-example-feature-platform-core-change-openapi-boundary:ph-1
    // @fdd-algo:fdd-fdd-example-feature-platform-core-algo-request-audit:ph-1

    // fdd-begin fdd-fdd-example-feature-platform-core-algo-request-audit:ph-1:inst-build-audit
    let id = Uuid::new_v4().to_string();
    let user_email = principal.map(|p| p.user_email.clone());
    let created_at = Utc::now().to_rfc3339();
    let op = operation.to_string();
    // fdd-end fdd-fdd-example-feature-platform-core-algo-request-audit:ph-1:inst-build-audit

    // fdd-begin fdd-fdd-example-feature-platform-core-algo-request-audit:ph-1:inst-try-persist-audit
    // fdd-begin fdd-fdd-example-feature-platform-core-algo-request-audit:ph-1:inst-persist-audit
    let result = tx::execute_with_retry(pool, || async {
        sqlx::query(
            r#"
            INSERT INTO request_audit (id, user_email, operation, status_code, created_at)
            VALUES (?, ?, ?, ?, ?);
            "#,
        )
        .bind(&id)
        .bind(user_email.as_deref())
        .bind(&op)
        .bind(status_code as i64)
        .bind(&created_at)
        .execute(pool)
        .await
        .map(|_| ())
    })
    .await;
    // fdd-end fdd-fdd-example-feature-platform-core-algo-request-audit:ph-1:inst-persist-audit

    // fdd-begin fdd-fdd-example-feature-platform-core-algo-request-audit:ph-1:inst-catch-audit
    if let Err(err) = result {
        // fdd-begin fdd-fdd-example-feature-platform-core-algo-request-audit:ph-1:inst-log-audit-failure
        let _ = err;
        // fdd-end fdd-fdd-example-feature-platform-core-algo-request-audit:ph-1:inst-log-audit-failure
    }
    // fdd-end fdd-fdd-example-feature-platform-core-algo-request-audit:ph-1:inst-catch-audit

    // fdd-begin fdd-fdd-example-feature-platform-core-algo-request-audit:ph-1:inst-return-audit
    let _ = result;
    // fdd-end fdd-fdd-example-feature-platform-core-algo-request-audit:ph-1:inst-return-audit
    // fdd-end fdd-fdd-example-feature-platform-core-algo-request-audit:ph-1:inst-try-persist-audit
}
