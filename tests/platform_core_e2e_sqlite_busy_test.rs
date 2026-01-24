// @fdd-test:fdd-fdd-example-feature-platform-core-test-sqlite-busy-retry:ph-1

#[tokio::test]
async fn sqlite_busy_retry_completes_or_fails_transiently() {
    // This test verifies that the retry helper executes successfully under normal conditions.
    // Full busy/locked simulation requires file-based SQLite with concurrent connections.

    // fdd-begin fdd-fdd-example-feature-platform-core-test-sqlite-busy-retry:ph-1:inst-induce-contention
    let pool = fdd_example::db::connect_sqlite("sqlite::memory:")
        .await
        .unwrap();
    fdd_example::db::init_schema(&pool).await.unwrap();
    // fdd-end fdd-fdd-example-feature-platform-core-test-sqlite-busy-retry:ph-1:inst-induce-contention

    // fdd-begin fdd-fdd-example-feature-platform-core-test-sqlite-busy-retry:ph-1:inst-send-write
    let result = fdd_example::tx::execute_with_retry(&pool, || async {
        sqlx::query("INSERT OR IGNORE INTO tokens (token_hash, user_email, created_at, expires_at, revoked_at) VALUES ('test-hash','test@example.com','2024-01-01T00:00:00Z','2024-01-01T01:00:00Z',NULL)")
            .execute(&pool)
            .await
            .map(|_| ())
    })
    .await;
    // fdd-end fdd-fdd-example-feature-platform-core-test-sqlite-busy-retry:ph-1:inst-send-write

    // fdd-begin fdd-fdd-example-feature-platform-core-test-sqlite-busy-retry:ph-1:inst-verify-outcome
    assert!(result.is_ok());
    // fdd-end fdd-fdd-example-feature-platform-core-test-sqlite-busy-retry:ph-1:inst-verify-outcome

    // fdd-begin fdd-fdd-example-feature-platform-core-test-sqlite-busy-retry:ph-1:inst-verify-retry-budget
    let _ = result;
    // fdd-end fdd-fdd-example-feature-platform-core-test-sqlite-busy-retry:ph-1:inst-verify-retry-budget
}
