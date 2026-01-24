use chrono::{DateTime, Utc};
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};

pub async fn connect_sqlite(database_url: &str) -> Result<SqlitePool, sqlx::Error> {
    SqlitePoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
}

pub async fn init_schema(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    // @fdd-change:fdd-fdd-example-feature-business-domain-change-tasks-crud:ph-1
    // @fdd-change:fdd-fdd-example-feature-business-domain-change-assignment:ph-1
    // @fdd-change:fdd-fdd-example-feature-business-domain-change-due-dates:ph-1
    // @fdd-change:fdd-fdd-example-feature-business-domain-change-comments:ph-1
    // @fdd-change:fdd-fdd-example-feature-business-domain-change-teams:ph-1
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS tokens (
            token_hash TEXT PRIMARY KEY,
            user_email TEXT NOT NULL,
            created_at TEXT NOT NULL,
            expires_at TEXT NOT NULL,
            revoked_at TEXT
        );

        CREATE TABLE IF NOT EXISTS request_audit (
            id TEXT PRIMARY KEY,
            user_email TEXT,
            operation TEXT NOT NULL,
            status_code INTEGER NOT NULL,
            created_at TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS teams (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS team_memberships (
            team_id TEXT NOT NULL,
            user_id TEXT NOT NULL,
            role TEXT NOT NULL,
            PRIMARY KEY (team_id, user_id)
        );

        CREATE TABLE IF NOT EXISTS tasks (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            description TEXT,
            status TEXT NOT NULL,
            due_at TEXT,
            assignee_user_id TEXT,
            team_id TEXT,
            owner_user_id TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS comments (
            id TEXT PRIMARY KEY,
            task_id TEXT NOT NULL,
            author_user_id TEXT NOT NULL,
            body TEXT NOT NULL,
            created_at TEXT NOT NULL
        );
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn insert_token(
    pool: &SqlitePool,
    token_hash: &str,
    user_email: &str,
    created_at: DateTime<Utc>,
    expires_at: DateTime<Utc>,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO tokens (token_hash, user_email, created_at, expires_at, revoked_at)
        VALUES (?, ?, ?, ?, NULL);
        "#,
    )
    .bind(token_hash)
    .bind(user_email)
    .bind(created_at.to_rfc3339())
    .bind(expires_at.to_rfc3339())
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn revoke_token(
    pool: &SqlitePool,
    token_hash: &str,
    revoked_at: DateTime<Utc>,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE tokens
        SET revoked_at = ?
        WHERE token_hash = ?;
        "#,
    )
    .bind(revoked_at.to_rfc3339())
    .bind(token_hash)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn set_token_expiry(
    pool: &SqlitePool,
    token_hash: &str,
    expires_at: DateTime<Utc>,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE tokens
        SET expires_at = ?
        WHERE token_hash = ?;
        "#,
    )
    .bind(expires_at.to_rfc3339())
    .bind(token_hash)
    .execute(pool)
    .await?;

    Ok(())
}

pub struct TokenRecord {
    pub token_hash: String,
    pub user_email: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub revoked_at: Option<DateTime<Utc>>,
}

pub async fn find_token(
    pool: &SqlitePool,
    token_hash: &str,
) -> Result<Option<TokenRecord>, sqlx::Error> {
    let rec = sqlx::query_as::<_, (String, String, String, String, Option<String>)>(
        r#"
        SELECT token_hash, user_email, created_at, expires_at, revoked_at
        FROM tokens
        WHERE token_hash = ?
        LIMIT 1;
        "#,
    )
    .bind(token_hash)
    .fetch_optional(pool)
    .await?;

    Ok(rec.map(
        |(token_hash, user_email, created_at, expires_at, revoked_at)| TokenRecord {
            token_hash,
            user_email,
            created_at: DateTime::parse_from_rfc3339(&created_at)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now()),
            expires_at: DateTime::parse_from_rfc3339(&expires_at)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now()),
            revoked_at: revoked_at
                .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
                .map(|dt| dt.with_timezone(&Utc)),
        },
    ))
}

#[derive(Debug, Clone)]
pub struct TeamRecord {
    pub id: String,
    pub name: String,
}

pub async fn insert_team(pool: &SqlitePool, id: &str, name: &str) -> Result<(), sqlx::Error> {
    // @fdd-change:fdd-fdd-example-feature-business-domain-change-teams:ph-1
    sqlx::query(
        r#"
        INSERT INTO teams (id, name)
        VALUES (?, ?);
        "#,
    )
    .bind(id)
    .bind(name)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn insert_team_membership(
    pool: &SqlitePool,
    team_id: &str,
    user_id: &str,
    role: &str,
) -> Result<(), sqlx::Error> {
    // @fdd-change:fdd-fdd-example-feature-business-domain-change-assignment:ph-1
    // @fdd-change:fdd-fdd-example-feature-business-domain-change-teams:ph-1
    sqlx::query(
        r#"
        INSERT OR REPLACE INTO team_memberships (team_id, user_id, role)
        VALUES (?, ?, ?);
        "#,
    )
    .bind(team_id)
    .bind(user_id)
    .bind(role)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn find_team_role(
    pool: &SqlitePool,
    team_id: &str,
    user_id: &str,
) -> Result<Option<String>, sqlx::Error> {
    // @fdd-change:fdd-fdd-example-feature-business-domain-change-assignment:ph-1
    let rec = sqlx::query_as::<_, (String,)>(
        r#"
        SELECT role
        FROM team_memberships
        WHERE team_id = ? AND user_id = ?
        LIMIT 1;
        "#,
    )
    .bind(team_id)
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    Ok(rec.map(|(role,)| role))
}

pub async fn list_teams_for_user(
    pool: &SqlitePool,
    user_id: &str,
) -> Result<Vec<TeamRecord>, sqlx::Error> {
    // @fdd-change:fdd-fdd-example-feature-business-domain-change-teams:ph-1
    let rows = sqlx::query_as::<_, (String, String)>(
        r#"
        SELECT t.id, t.name
        FROM teams t
        WHERE EXISTS (
            SELECT 1
            FROM team_memberships m
            WHERE m.team_id = t.id AND m.user_id = ?
        )
        ORDER BY t.name ASC;
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|(id, name)| TeamRecord { id, name })
        .collect())
}

#[derive(Debug, Clone)]
pub struct TaskRecord {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub due_at: Option<String>,
    pub assignee_user_id: Option<String>,
    pub team_id: Option<String>,
    pub owner_user_id: String,
    pub created_at: String,
    pub updated_at: String,
}

pub async fn insert_task(pool: &SqlitePool, task: &TaskRecord) -> Result<(), sqlx::Error> {
    // @fdd-change:fdd-fdd-example-feature-business-domain-change-tasks-crud:ph-1
    // @fdd-change:fdd-fdd-example-feature-business-domain-change-due-dates:ph-1
    // @fdd-change:fdd-fdd-example-feature-business-domain-change-assignment:ph-1
    sqlx::query(
        r#"
        INSERT INTO tasks (
            id, title, description, status, due_at,
            assignee_user_id, team_id, owner_user_id,
            created_at, updated_at
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?);
        "#,
    )
    .bind(&task.id)
    .bind(&task.title)
    .bind(task.description.as_deref())
    .bind(&task.status)
    .bind(task.due_at.as_deref())
    .bind(task.assignee_user_id.as_deref())
    .bind(task.team_id.as_deref())
    .bind(&task.owner_user_id)
    .bind(&task.created_at)
    .bind(&task.updated_at)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn find_task(
    pool: &SqlitePool,
    task_id: &str,
) -> Result<Option<TaskRecord>, sqlx::Error> {
    // @fdd-change:fdd-fdd-example-feature-business-domain-change-tasks-crud:ph-1
    let rec = sqlx::query_as::<_, (
        String,
        String,
        Option<String>,
        String,
        Option<String>,
        Option<String>,
        Option<String>,
        String,
        String,
        String,
    )>(
        r#"
        SELECT id, title, description, status, due_at, assignee_user_id, team_id, owner_user_id, created_at, updated_at
        FROM tasks
        WHERE id = ?
        LIMIT 1;
        "#,
    )
    .bind(task_id)
    .fetch_optional(pool)
    .await?;

    Ok(rec.map(
        |(
            id,
            title,
            description,
            status,
            due_at,
            assignee_user_id,
            team_id,
            owner_user_id,
            created_at,
            updated_at,
        )| {
            TaskRecord {
                id,
                title,
                description,
                status,
                due_at,
                assignee_user_id,
                team_id,
                owner_user_id,
                created_at,
                updated_at,
            }
        },
    ))
}

#[allow(clippy::too_many_arguments)]
pub async fn update_task(
    pool: &SqlitePool,
    task_id: &str,
    title: &str,
    description: Option<&str>,
    status: &str,
    due_at: Option<&str>,
    assignee_user_id: Option<&str>,
    updated_at: &str,
) -> Result<(), sqlx::Error> {
    // @fdd-change:fdd-fdd-example-feature-business-domain-change-tasks-crud:ph-1
    // @fdd-change:fdd-fdd-example-feature-business-domain-change-due-dates:ph-1
    // @fdd-change:fdd-fdd-example-feature-business-domain-change-assignment:ph-1
    sqlx::query(
        r#"
        UPDATE tasks
        SET title = ?, description = ?, status = ?, due_at = ?, assignee_user_id = ?, updated_at = ?
        WHERE id = ?;
        "#,
    )
    .bind(title)
    .bind(description)
    .bind(status)
    .bind(due_at)
    .bind(assignee_user_id)
    .bind(updated_at)
    .bind(task_id)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn delete_task(pool: &SqlitePool, task_id: &str) -> Result<u64, sqlx::Error> {
    // @fdd-change:fdd-fdd-example-feature-business-domain-change-tasks-crud:ph-1
    let res = sqlx::query(
        r#"
        DELETE FROM tasks
        WHERE id = ?;
        "#,
    )
    .bind(task_id)
    .execute(pool)
    .await?;

    Ok(res.rows_affected())
}

pub async fn list_tasks_visible_to_user(
    pool: &SqlitePool,
    user_id: &str,
    team_id: Option<&str>,
    assignee_user_id: Option<&str>,
    due_before: Option<&str>,
    status: Option<&str>,
) -> Result<Vec<TaskRecord>, sqlx::Error> {
    // @fdd-change:fdd-fdd-example-feature-business-domain-change-tasks-crud:ph-1
    // @fdd-change:fdd-fdd-example-feature-business-domain-change-due-dates:ph-1
    // @fdd-change:fdd-fdd-example-feature-business-domain-change-assignment:ph-1
    let mut sql = String::from(
        r#"
        SELECT id, title, description, status, due_at, assignee_user_id, team_id, owner_user_id, created_at, updated_at
        FROM tasks t
        WHERE (
            (t.team_id IS NULL AND t.owner_user_id = ?)
            OR (
                t.team_id IS NOT NULL
                AND EXISTS (
                    SELECT 1 FROM team_memberships m
                    WHERE m.team_id = t.team_id AND m.user_id = ?
                )
            )
        )
        "#,
    );

    if team_id.is_some() {
        sql.push_str(" AND t.team_id = ?");
    }
    if assignee_user_id.is_some() {
        sql.push_str(" AND t.assignee_user_id = ?");
    }
    if due_before.is_some() {
        sql.push_str(" AND t.due_at IS NOT NULL AND t.due_at < ?");
    }
    if status.is_some() {
        sql.push_str(" AND t.status = ?");
    }
    sql.push_str(" ORDER BY t.created_at DESC;");

    let mut q = sqlx::query_as::<
        _,
        (
            String,
            String,
            Option<String>,
            String,
            Option<String>,
            Option<String>,
            Option<String>,
            String,
            String,
            String,
        ),
    >(&sql)
    .bind(user_id)
    .bind(user_id);

    if let Some(team_id) = team_id {
        q = q.bind(team_id);
    }
    if let Some(assignee_user_id) = assignee_user_id {
        q = q.bind(assignee_user_id);
    }
    if let Some(due_before) = due_before {
        q = q.bind(due_before);
    }
    if let Some(status) = status {
        q = q.bind(status);
    }

    let rows = q.fetch_all(pool).await?;

    Ok(rows
        .into_iter()
        .map(
            |(
                id,
                title,
                description,
                status,
                due_at,
                assignee_user_id,
                team_id,
                owner_user_id,
                created_at,
                updated_at,
            )| {
                TaskRecord {
                    id,
                    title,
                    description,
                    status,
                    due_at,
                    assignee_user_id,
                    team_id,
                    owner_user_id,
                    created_at,
                    updated_at,
                }
            },
        )
        .collect())
}

#[derive(Debug, Clone)]
pub struct CommentRecord {
    pub id: String,
    pub task_id: String,
    pub author_user_id: String,
    pub body: String,
    pub created_at: String,
}

pub async fn insert_comment(pool: &SqlitePool, comment: &CommentRecord) -> Result<(), sqlx::Error> {
    // @fdd-change:fdd-fdd-example-feature-business-domain-change-comments:ph-1
    sqlx::query(
        r#"
        INSERT INTO comments (id, task_id, author_user_id, body, created_at)
        VALUES (?, ?, ?, ?, ?);
        "#,
    )
    .bind(&comment.id)
    .bind(&comment.task_id)
    .bind(&comment.author_user_id)
    .bind(&comment.body)
    .bind(&comment.created_at)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn list_comments_for_task(
    pool: &SqlitePool,
    task_id: &str,
) -> Result<Vec<CommentRecord>, sqlx::Error> {
    // @fdd-change:fdd-fdd-example-feature-business-domain-change-comments:ph-1
    let rows = sqlx::query_as::<_, (String, String, String, String, String)>(
        r#"
        SELECT id, task_id, author_user_id, body, created_at
        FROM comments
        WHERE task_id = ?
        ORDER BY created_at ASC;
        "#,
    )
    .bind(task_id)
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(
            |(id, task_id, author_user_id, body, created_at)| CommentRecord {
                id,
                task_id,
                author_user_id,
                body,
                created_at,
            },
        )
        .collect())
}

pub fn hash_token(token: &str) -> String {
    let mut hasher = sha2::Sha256::new();
    use sha2::Digest;
    hasher.update(token.as_bytes());
    hex::encode(hasher.finalize())
}

pub fn is_busy_error(err: &sqlx::Error) -> bool {
    match err {
        sqlx::Error::Database(db_err) => {
            let msg = db_err.message().to_ascii_lowercase();
            msg.contains("database is locked")
                || msg.contains("database is busy")
                || db_err.code().map(|c| c == "SQLITE_BUSY").unwrap_or(false)
        }
        _ => false,
    }
}
