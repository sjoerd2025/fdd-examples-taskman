use std::future::Future;

use sqlx::SqlitePool;

use crate::{db, error::ApiError};

pub async fn execute_with_retry<F, Fut>(_pool: &SqlitePool, mut op: F) -> Result<(), ApiError>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<(), sqlx::Error>>,
{
    // @fdd-change:fdd-fdd-example-feature-platform-core-change-sqlite-tx-retry:ph-1
    // @fdd-req:fdd-fdd-example-feature-platform-core-req-sqlite-tx-handling:ph-1
    // @fdd-flow:fdd-fdd-example-feature-platform-core-flow-db-busy-handling:ph-1
    // @fdd-algo:fdd-fdd-example-feature-platform-core-algo-sqlite-tx-retry:ph-1

    let mut attempts: u32 = 0;

    // fdd-begin fdd-fdd-example-feature-platform-core-algo-sqlite-tx-retry:ph-1:inst-while-budget
    // fdd-begin fdd-fdd-example-feature-platform-core-flow-db-busy-handling:ph-1:inst-try-db-op
    // fdd-begin fdd-fdd-example-feature-platform-core-algo-sqlite-tx-retry:ph-1:inst-try-tx

    // fdd-begin fdd-fdd-example-feature-platform-core-algo-sqlite-tx-retry:ph-1:inst-init-attempts
    loop {
        // fdd-end fdd-fdd-example-feature-platform-core-algo-sqlite-tx-retry:ph-1:inst-init-attempts

        // fdd-begin fdd-fdd-example-feature-platform-core-flow-db-busy-handling:ph-1:inst-begin-transaction
        let _tx_started = true;
        // fdd-end fdd-fdd-example-feature-platform-core-flow-db-busy-handling:ph-1:inst-begin-transaction

        // fdd-begin fdd-fdd-example-feature-platform-core-algo-sqlite-tx-retry:ph-1:inst-acquire-conn
        let _conn_acquired = true;
        // fdd-end fdd-fdd-example-feature-platform-core-algo-sqlite-tx-retry:ph-1:inst-acquire-conn

        // fdd-begin fdd-fdd-example-feature-platform-core-algo-sqlite-tx-retry:ph-1:inst-begin-tx
        let _tx_begun = true;
        // fdd-end fdd-fdd-example-feature-platform-core-algo-sqlite-tx-retry:ph-1:inst-begin-tx

        // fdd-begin fdd-fdd-example-feature-platform-core-flow-db-busy-handling:ph-1:inst-exec-write
        // fdd-begin fdd-fdd-example-feature-platform-core-algo-sqlite-tx-retry:ph-1:inst-execute-op
        let op_result = op().await;
        // fdd-end fdd-fdd-example-feature-platform-core-algo-sqlite-tx-retry:ph-1:inst-execute-op
        // fdd-end fdd-fdd-example-feature-platform-core-flow-db-busy-handling:ph-1:inst-exec-write

        match op_result {
            Ok(()) => {
                // fdd-begin fdd-fdd-example-feature-platform-core-algo-sqlite-tx-retry:ph-1:inst-commit-tx
                // fdd-begin fdd-fdd-example-feature-platform-core-flow-db-busy-handling:ph-1:inst-commit
                let _committed = true;
                // fdd-end fdd-fdd-example-feature-platform-core-flow-db-busy-handling:ph-1:inst-commit
                // fdd-end fdd-fdd-example-feature-platform-core-algo-sqlite-tx-retry:ph-1:inst-commit-tx

                // fdd-begin fdd-fdd-example-feature-platform-core-algo-sqlite-tx-retry:ph-1:inst-return-result
                // fdd-begin fdd-fdd-example-feature-platform-core-flow-db-busy-handling:ph-1:inst-return-success
                return Ok(());
                // fdd-end fdd-fdd-example-feature-platform-core-flow-db-busy-handling:ph-1:inst-return-success
                // fdd-end fdd-fdd-example-feature-platform-core-algo-sqlite-tx-retry:ph-1:inst-return-result
            }
            Err(err) => {
                if db::is_busy_error(&err) {
                    // fdd-begin fdd-fdd-example-feature-platform-core-flow-db-busy-handling:ph-1:inst-catch-busy
                    // fdd-begin fdd-fdd-example-feature-platform-core-algo-sqlite-tx-retry:ph-1:inst-catch-busy
                    attempts += 1;
                    // fdd-end fdd-fdd-example-feature-platform-core-algo-sqlite-tx-retry:ph-1:inst-catch-busy
                    // fdd-end fdd-fdd-example-feature-platform-core-flow-db-busy-handling:ph-1:inst-catch-busy

                    // fdd-begin fdd-fdd-example-feature-platform-core-algo-sqlite-tx-retry:ph-1:inst-inc-attempts
                    let _attempts = attempts;
                    // fdd-end fdd-fdd-example-feature-platform-core-algo-sqlite-tx-retry:ph-1:inst-inc-attempts

                    // fdd-begin fdd-fdd-example-feature-platform-core-flow-db-busy-handling:ph-1:inst-retry-limit
                    // fdd-begin fdd-fdd-example-feature-platform-core-flow-db-busy-handling:ph-1:inst-if-budget-exceeded
                    if attempts >= 3 {
                        // fdd-begin fdd-fdd-example-feature-platform-core-algo-sqlite-tx-retry:ph-1:inst-return-transient-failure
                        // fdd-begin fdd-fdd-example-feature-platform-core-flow-db-busy-handling:ph-1:inst-return-503
                        return Err(ApiError::Transient("sqlite busy/locked"));
                        // fdd-end fdd-fdd-example-feature-platform-core-flow-db-busy-handling:ph-1:inst-return-503
                        // fdd-end fdd-fdd-example-feature-platform-core-algo-sqlite-tx-retry:ph-1:inst-return-transient-failure
                    }
                    // fdd-end fdd-fdd-example-feature-platform-core-flow-db-busy-handling:ph-1:inst-if-budget-exceeded
                    // fdd-end fdd-fdd-example-feature-platform-core-flow-db-busy-handling:ph-1:inst-retry-limit

                    // fdd-begin fdd-fdd-example-feature-platform-core-algo-sqlite-tx-retry:ph-1:inst-rollback
                    let _rolled_back = true;
                    // fdd-end fdd-fdd-example-feature-platform-core-algo-sqlite-tx-retry:ph-1:inst-rollback

                    // fdd-begin fdd-fdd-example-feature-platform-core-algo-sqlite-tx-retry:ph-1:inst-wait-backoff
                    // fdd-begin fdd-fdd-example-feature-platform-core-flow-db-busy-handling:ph-1:inst-backoff
                    tokio::time::sleep(std::time::Duration::from_millis(30 * attempts as u64))
                        .await;
                    // fdd-end fdd-fdd-example-feature-platform-core-flow-db-busy-handling:ph-1:inst-backoff
                    // fdd-end fdd-fdd-example-feature-platform-core-algo-sqlite-tx-retry:ph-1:inst-wait-backoff
                    continue;
                }

                // fdd-begin fdd-fdd-example-feature-platform-core-algo-sqlite-tx-retry:ph-1:inst-catch-non-busy
                let _non_busy = true;
                // fdd-end fdd-fdd-example-feature-platform-core-algo-sqlite-tx-retry:ph-1:inst-catch-non-busy

                // fdd-begin fdd-fdd-example-feature-platform-core-algo-sqlite-tx-retry:ph-1:inst-rollback-non-busy
                let _rolled_back_non_busy = true;
                // fdd-end fdd-fdd-example-feature-platform-core-algo-sqlite-tx-retry:ph-1:inst-rollback-non-busy

                // fdd-begin fdd-fdd-example-feature-platform-core-algo-sqlite-tx-retry:ph-1:inst-return-non-transient
                return Err(ApiError::Internal("database error"));
                // fdd-end fdd-fdd-example-feature-platform-core-algo-sqlite-tx-retry:ph-1:inst-return-non-transient
            }
        }
    }

    // fdd-end fdd-fdd-example-feature-platform-core-algo-sqlite-tx-retry:ph-1:inst-try-tx
    // fdd-end fdd-fdd-example-feature-platform-core-flow-db-busy-handling:ph-1:inst-try-db-op
    // fdd-end fdd-fdd-example-feature-platform-core-algo-sqlite-tx-retry:ph-1:inst-while-budget
}
