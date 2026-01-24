use std::net::SocketAddr;

use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let openapi_path =
        std::env::var("OPENAPI_PATH").unwrap_or_else(|_| "api/openapi.yaml".to_string());
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:./data/fdd-example.db?mode=rwc".to_string());

    std::fs::create_dir_all("data")?;

    let pool = fdd_example::db::connect_sqlite(&database_url).await?;
    fdd_example::db::init_schema(&pool).await?;

    // dev-seed-begin: insert demo data if empty
    if sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM tasks;")
        .fetch_one(&pool)
        .await
        .unwrap_or(0)
        == 0
    {
        use fdd_example::db;
        use chrono::Utc;
        use sha2::{Digest, Sha256};
        use uuid::Uuid;
        let demo_email = "demo@example.com";
        let mut hasher = Sha256::new();
        hasher.update(demo_email.as_bytes());
        let digest = hasher.finalize();
        let mut bytes = [0u8; 16];
        bytes.copy_from_slice(&digest[..16]);
        let demo_user_id = Uuid::from_bytes(bytes).to_string();
        let now = Utc::now().to_rfc3339();
        let demo_tasks = vec![
            ("Buy milk", "to-do"),
            ("Write docs", "in-progress"),
            ("Deploy release", "done"),
        ];
        for (title, status) in demo_tasks {
            db::insert_task(
                &pool,
                &db::TaskRecord {
                    id: Uuid::new_v4().to_string(),
                    title: title.into(),
                    description: None,
                    status: status.into(),
                    due_at: None,
                    assignee_user_id: None,
                    team_id: None,
                    owner_user_id: demo_user_id.clone(),
                    created_at: now.clone(),
                    updated_at: now.clone(),
                },
            )
            .await
            .ok();
        }
    }
    // dev-seed-end

    // @fdd-change:fdd-fdd-example-feature-platform-core-change-openapi-boundary:ph-1
    let openapi = fdd_example::openapi::load_openapi(&openapi_path).await?;

    let app = fdd_example::app::build_app(pool, openapi);

    let addr: SocketAddr = "127.0.0.1:8080".parse()?;
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
