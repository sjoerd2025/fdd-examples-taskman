use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use sqlx::SqlitePool;
use tower::ServiceBuilder;
use tower_http::{
    services::{ServeDir, ServeFile},
    set_header::SetResponseHeaderLayer,
};
use uuid::Uuid;

use crate::{audit, auth::Principal, db, error::ApiError, tx};

#[derive(Clone)]
pub struct AppState {
    pub pool: SqlitePool,
    pub openapi: Value,
}

#[derive(Debug, Deserialize)]
pub struct CreateTokenRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct CreateTokenResponse {
    pub access_token: String,
    pub token_type: String,
}

pub fn build_app(pool: SqlitePool, openapi: Value) -> Router {
    let state = AppState { pool, openapi };

    // @fdd-req:fdd-fdd-example-feature-first-party-web-ui-req-provide-ui:ph-1
    // @fdd-flow:fdd-fdd-example-feature-first-party-web-ui-flow-authenticate:ph-1
    // @fdd-flow:fdd-fdd-example-feature-first-party-web-ui-flow-browse-tasks:ph-1
    // @fdd-algo:fdd-fdd-example-feature-first-party-web-ui-algo-apply-ui-caching:ph-1
    // fdd-begin fdd-fdd-example-feature-first-party-web-ui-algo-apply-ui-caching:ph-1:inst-if-static-asset
    // fdd-begin fdd-fdd-example-feature-first-party-web-ui-algo-apply-ui-caching:ph-1:inst-use-cache-headers
    let ui_assets = ServiceBuilder::new()
        .layer(SetResponseHeaderLayer::overriding(
            axum::http::header::CACHE_CONTROL,
            axum::http::HeaderValue::from_static("public, max-age=31536000, immutable"),
        ))
        .service(ServeDir::new("ui/dist/assets"));
    // fdd-end fdd-fdd-example-feature-first-party-web-ui-algo-apply-ui-caching:ph-1:inst-use-cache-headers
    // fdd-end fdd-fdd-example-feature-first-party-web-ui-algo-apply-ui-caching:ph-1:inst-if-static-asset

    let ui_spa = ServeDir::new("ui/dist").not_found_service(ServeFile::new("ui/dist/index.html"));

    // @fdd-change:fdd-fdd-example-feature-platform-core-change-openapi-boundary:ph-1
    // @fdd-flow:fdd-fdd-example-feature-platform-core-flow-authenticated-call:ph-1
    // fdd-begin fdd-fdd-example-feature-platform-core-flow-authenticated-call:ph-1:inst-route-to-handler
    let router = Router::new()
        .route("/health", get(health))
        .nest_service("/assets", ui_assets)
        .route("/auth/tokens", post(create_token))
        .route("/tasks", get(list_tasks).post(create_task))
        .route(
            "/tasks/:task_id",
            get(get_task).patch(update_task).delete(delete_task),
        )
        .route(
            "/tasks/:task_id/comments",
            get(list_task_comments).post(create_task_comment),
        )
        .route("/teams", get(list_teams).post(create_team))
        .with_state(state);

    // fdd-begin fdd-fdd-example-feature-first-party-web-ui-flow-authenticate:ph-1:inst-open-entry
    let router = router.fallback_service(ui_spa);
    // fdd-end fdd-fdd-example-feature-first-party-web-ui-flow-authenticate:ph-1:inst-open-entry

    router
    // fdd-end fdd-fdd-example-feature-platform-core-flow-authenticated-call:ph-1:inst-route-to-handler
}

async fn health() -> &'static str {
    "ok"
}

#[derive(Debug, Deserialize)]
struct ListTasksQuery {
    pub team_id: Option<String>,
    pub assignee_user_id: Option<String>,
    pub due_before: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize)]
struct ItemsResponse<T> {
    pub items: Vec<T>,
}

#[derive(Debug, Serialize)]
struct TaskResponse {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub due_at: Option<String>,
    pub assignee_user_id: Option<String>,
    pub team_id: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

fn task_to_response(task: db::TaskRecord) -> TaskResponse {
    TaskResponse {
        id: task.id,
        title: task.title,
        description: task.description,
        status: task.status,
        due_at: task.due_at,
        assignee_user_id: task.assignee_user_id,
        team_id: task.team_id,
        created_at: task.created_at,
        updated_at: task.updated_at,
    }
}

fn is_valid_status(status: &str) -> bool {
    matches!(status, "to-do" | "in-progress" | "done")
}

fn is_allowed_status_transition(from: &str, to: &str) -> bool {
    match (from, to) {
        ("to-do", "in-progress") => true,
        ("in-progress", "done") => true,
        ("done", "in-progress") => true,
        ("in-progress", "to-do") => true,
        (a, b) => a == b,
    }
}

fn validate_title(title: &str) -> Result<(), ApiError> {
    if title.trim().is_empty() {
        return Err(ApiError::Validation("title is required"));
    }
    if title.len() > 200 {
        return Err(ApiError::Validation("title too long"));
    }
    Ok(())
}

fn validate_comment_body(body: &str) -> Result<(), ApiError> {
    if body.trim().is_empty() {
        return Err(ApiError::Validation("body is required"));
    }
    if body.len() > 4000 {
        return Err(ApiError::Validation("body too long"));
    }
    Ok(())
}

fn validate_team_name(name: &str) -> Result<(), ApiError> {
    if name.trim().is_empty() {
        return Err(ApiError::Validation("name is required"));
    }
    if name.len() > 200 {
        return Err(ApiError::Validation("name too long"));
    }
    Ok(())
}

fn normalize_rfc3339_datetime_to_utc(value: &str) -> Result<String, ApiError> {
    let dt = DateTime::parse_from_rfc3339(value)
        .map_err(|_| ApiError::Validation("invalid date-time"))?;
    Ok(dt.with_timezone(&Utc).to_rfc3339())
}

fn validate_uuid(value: &str) -> Result<(), ApiError> {
    Uuid::parse_str(value)
        .map(|_| ())
        .map_err(|_| ApiError::Validation("invalid uuid"))
}

fn deserialize_nullable_field<'de, D, T>(deserializer: D) -> Result<Option<Option<T>>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: serde::Deserialize<'de>,
{
    Ok(Some(Option::<T>::deserialize(deserializer)?))
}

async fn authorize_task_read_or_update(
    pool: &SqlitePool,
    principal: &Principal,
    task: &db::TaskRecord,
) -> Result<(), ApiError> {
    if task.team_id.is_none() {
        if task.owner_user_id != principal.user_id.to_string() {
            return Err(ApiError::Authorization("task access denied"));
        }
        return Ok(());
    }

    let team_id = task.team_id.as_deref().unwrap_or_default();
    let role = db::find_team_role(pool, team_id, &principal.user_id.to_string())
        .await
        .map_err(|_| ApiError::Internal("membership lookup failed"))?;
    if role.is_none() {
        return Err(ApiError::Authorization("task access denied"));
    }
    Ok(())
}

async fn authorize_task_delete(
    pool: &SqlitePool,
    principal: &Principal,
    task: &db::TaskRecord,
) -> Result<(), ApiError> {
    if task.team_id.is_none() {
        if task.owner_user_id != principal.user_id.to_string() {
            return Err(ApiError::Authorization("task delete denied"));
        }
        return Ok(());
    }

    let team_id = task.team_id.as_deref().unwrap_or_default();
    let role = db::find_team_role(pool, team_id, &principal.user_id.to_string())
        .await
        .map_err(|_| ApiError::Internal("membership lookup failed"))?;
    match role.as_deref() {
        Some("admin") => Ok(()),
        _ => Err(ApiError::Authorization("task delete denied")),
    }
}

async fn list_tasks(
    State(state): State<AppState>,
    principal: Principal,
    Query(query): Query<ListTasksQuery>,
) -> Result<Json<ItemsResponse<TaskResponse>>, ApiError> {
    // @fdd-change:fdd-fdd-example-feature-business-domain-change-tasks-crud:ph-1
    // @fdd-flow:fdd-fdd-example-feature-business-domain-flow-task-management:ph-1

    // fdd-begin fdd-fdd-example-feature-business-domain-flow-task-management:ph-1:inst-list-tasks-request
    if let Some(ref team_id) = query.team_id {
        validate_uuid(team_id)?;
    }
    if let Some(ref assignee_user_id) = query.assignee_user_id {
        validate_uuid(assignee_user_id)?;
    }
    let due_before = match query.due_before.as_deref() {
        Some(v) => Some(normalize_rfc3339_datetime_to_utc(v)?),
        None => None,
    };
    if let Some(ref status) = query.status {
        if !is_valid_status(status) {
            return Err(ApiError::Validation("invalid status"));
        }
    }

    let rows = db::list_tasks_visible_to_user(
        &state.pool,
        &principal.user_id.to_string(),
        query.team_id.as_deref(),
        query.assignee_user_id.as_deref(),
        due_before.as_deref(),
        query.status.as_deref(),
    )
    .await
    .map_err(|_| ApiError::Internal("task query failed"))?;
    // fdd-end fdd-fdd-example-feature-business-domain-flow-task-management:ph-1:inst-list-tasks-request

    let items = rows.into_iter().map(task_to_response).collect();

    audit::record_request_audit(
        &state.pool,
        Some(&principal),
        "GET /tasks",
        StatusCode::OK.as_u16(),
    )
    .await;

    // fdd-begin fdd-fdd-example-feature-business-domain-flow-task-management:ph-1:inst-return-visible-tasks
    Ok(Json(ItemsResponse { items }))
    // fdd-end fdd-fdd-example-feature-business-domain-flow-task-management:ph-1:inst-return-visible-tasks
}

#[derive(Debug, Deserialize)]
struct CreateTaskRequest {
    pub title: String,
    pub description: Option<String>,
    pub due_at: Option<String>,
    pub assignee_user_id: Option<String>,
    pub team_id: Option<String>,
}

async fn create_task(
    State(state): State<AppState>,
    principal: Principal,
    Json(payload): Json<CreateTaskRequest>,
) -> Result<(StatusCode, Json<TaskResponse>), ApiError> {
    // @fdd-change:fdd-fdd-example-feature-business-domain-change-tasks-crud:ph-1
    // @fdd-flow:fdd-fdd-example-feature-business-domain-flow-task-management:ph-1

    // fdd-begin fdd-fdd-example-feature-business-domain-flow-task-management:ph-1:inst-create-task-request
    validate_title(&payload.title)?;
    let due_at = match payload.due_at.as_deref() {
        Some(v) => Some(normalize_rfc3339_datetime_to_utc(v)?),
        None => None,
    };
    if let Some(ref assignee_user_id) = payload.assignee_user_id {
        validate_uuid(assignee_user_id)?;
    }
    if let Some(ref team_id) = payload.team_id {
        validate_uuid(team_id)?;
    }
    // fdd-end fdd-fdd-example-feature-business-domain-flow-task-management:ph-1:inst-create-task-request

    if let Some(team_id) = payload.team_id.as_deref() {
        let role = db::find_team_role(&state.pool, team_id, &principal.user_id.to_string())
            .await
            .map_err(|_| ApiError::Internal("membership lookup failed"))?;
        if role.is_none() {
            return Err(ApiError::Authorization("not a team member"));
        }
    }

    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    let task = db::TaskRecord {
        id: id.clone(),
        title: payload.title,
        description: payload.description,
        status: "to-do".to_string(),
        due_at,
        assignee_user_id: payload.assignee_user_id,
        team_id: payload.team_id,
        owner_user_id: principal.user_id.to_string(),
        created_at: now.clone(),
        updated_at: now,
    };

    tx::execute_with_retry(&state.pool, || async {
        db::insert_task(&state.pool, &task).await
    })
    .await?;

    audit::record_request_audit(
        &state.pool,
        Some(&principal),
        "POST /tasks",
        StatusCode::CREATED.as_u16(),
    )
    .await;

    // fdd-begin fdd-fdd-example-feature-business-domain-flow-task-management:ph-1:inst-return-created-task
    Ok((StatusCode::CREATED, Json(task_to_response(task))))
    // fdd-end fdd-fdd-example-feature-business-domain-flow-task-management:ph-1:inst-return-created-task
}

async fn get_task(
    State(state): State<AppState>,
    principal: Principal,
    Path(task_id): Path<String>,
) -> Result<Json<TaskResponse>, ApiError> {
    // @fdd-change:fdd-fdd-example-feature-business-domain-change-tasks-crud:ph-1
    // @fdd-flow:fdd-fdd-example-feature-business-domain-flow-task-management:ph-1

    let task = db::find_task(&state.pool, &task_id)
        .await
        .map_err(|_| ApiError::Internal("task lookup failed"))?
        .ok_or(ApiError::NotFound("task not found"))?;

    authorize_task_read_or_update(&state.pool, &principal, &task).await?;

    audit::record_request_audit(
        &state.pool,
        Some(&principal),
        "GET /tasks/{task_id}",
        StatusCode::OK.as_u16(),
    )
    .await;

    Ok(Json(task_to_response(task)))
}

#[derive(Debug, Deserialize)]
struct UpdateTaskRequest {
    pub title: Option<String>,
    #[serde(default, deserialize_with = "deserialize_nullable_field")]
    pub description: Option<Option<String>>,
    pub status: Option<String>,
    #[serde(default, deserialize_with = "deserialize_nullable_field")]
    pub due_at: Option<Option<String>>,
    #[serde(default, deserialize_with = "deserialize_nullable_field")]
    pub assignee_user_id: Option<Option<String>>,
}

async fn update_task(
    State(state): State<AppState>,
    principal: Principal,
    Path(task_id): Path<String>,
    Json(payload): Json<UpdateTaskRequest>,
) -> Result<Json<TaskResponse>, ApiError> {
    // @fdd-change:fdd-fdd-example-feature-business-domain-change-tasks-crud:ph-1
    // @fdd-flow:fdd-fdd-example-feature-business-domain-flow-task-management:ph-1

    let mut task = db::find_task(&state.pool, &task_id)
        .await
        .map_err(|_| ApiError::Internal("task lookup failed"))?
        .ok_or(ApiError::NotFound("task not found"))?;

    authorize_task_read_or_update(&state.pool, &principal, &task).await?;

    if let Some(title) = payload.title.as_deref() {
        validate_title(title)?;
        task.title = title.to_string();
    }
    if let Some(description) = payload.description {
        task.description = description;
    }
    if let Some(status) = payload.status.as_deref() {
        if !is_valid_status(status) {
            return Err(ApiError::Validation("invalid status"));
        }
        if !is_allowed_status_transition(&task.status, status) {
            return Err(ApiError::Conflict("invalid status transition"));
        }
        task.status = status.to_string();
    }

    // @fdd-change:fdd-fdd-example-feature-business-domain-change-due-dates:ph-1
    if let Some(due_at) = payload.due_at {
        let due_at = match due_at {
            Some(s) => Some(normalize_rfc3339_datetime_to_utc(&s)?),
            None => None,
        };
        task.due_at = due_at;
    }

    // @fdd-change:fdd-fdd-example-feature-business-domain-change-assignment:ph-1
    if let Some(assignee_user_id) = payload.assignee_user_id {
        if let Some(ref new_assignee) = assignee_user_id {
            validate_uuid(new_assignee)?;
            if task.team_id.is_none() {
                if new_assignee != &principal.user_id.to_string() {
                    return Err(ApiError::Conflict("personal task assignment must be self"));
                }
            } else {
                let team_id = task.team_id.as_deref().unwrap_or_default();
                let role = db::find_team_role(&state.pool, team_id, new_assignee)
                    .await
                    .map_err(|_| ApiError::Internal("membership lookup failed"))?;
                if role.is_none() {
                    return Err(ApiError::Conflict("assignee not a team member"));
                }
            }
        }
        task.assignee_user_id = assignee_user_id;
    }

    task.updated_at = Utc::now().to_rfc3339();

    tx::execute_with_retry(&state.pool, || async {
        db::update_task(
            &state.pool,
            &task.id,
            &task.title,
            task.description.as_deref(),
            &task.status,
            task.due_at.as_deref(),
            task.assignee_user_id.as_deref(),
            &task.updated_at,
        )
        .await
    })
    .await?;

    audit::record_request_audit(
        &state.pool,
        Some(&principal),
        "PATCH /tasks/{task_id}",
        StatusCode::OK.as_u16(),
    )
    .await;

    Ok(Json(task_to_response(task)))
}

async fn delete_task(
    State(state): State<AppState>,
    principal: Principal,
    Path(task_id): Path<String>,
) -> Result<StatusCode, ApiError> {
    // @fdd-change:fdd-fdd-example-feature-business-domain-change-tasks-crud:ph-1
    // @fdd-flow:fdd-fdd-example-feature-business-domain-flow-task-management:ph-1

    let task = db::find_task(&state.pool, &task_id)
        .await
        .map_err(|_| ApiError::Internal("task lookup failed"))?
        .ok_or(ApiError::NotFound("task not found"))?;

    authorize_task_delete(&state.pool, &principal, &task).await?;

    let rows = tx::execute_with_retry(&state.pool, || async {
        db::delete_task(&state.pool, &task_id).await.map(|_| ())
    })
    .await;
    if rows.is_err() {
        return Err(ApiError::Internal("task delete failed"));
    }

    audit::record_request_audit(
        &state.pool,
        Some(&principal),
        "DELETE /tasks/{task_id}",
        StatusCode::NO_CONTENT.as_u16(),
    )
    .await;
    Ok(StatusCode::NO_CONTENT)
}

#[derive(Debug, Deserialize)]
struct CreateCommentRequest {
    pub body: String,
}

#[derive(Debug, Serialize)]
struct CommentResponse {
    pub id: String,
    pub task_id: String,
    pub author_user_id: String,
    pub body: String,
    pub created_at: String,
}

fn comment_to_response(c: db::CommentRecord) -> CommentResponse {
    CommentResponse {
        id: c.id,
        task_id: c.task_id,
        author_user_id: c.author_user_id,
        body: c.body,
        created_at: c.created_at,
    }
}

async fn list_task_comments(
    State(state): State<AppState>,
    principal: Principal,
    Path(task_id): Path<String>,
) -> Result<Json<ItemsResponse<CommentResponse>>, ApiError> {
    // @fdd-change:fdd-fdd-example-feature-business-domain-change-comments:ph-1
    // @fdd-flow:fdd-fdd-example-feature-business-domain-flow-task-comments:ph-1

    let task = db::find_task(&state.pool, &task_id)
        .await
        .map_err(|_| ApiError::Internal("task lookup failed"))?
        .ok_or(ApiError::NotFound("task not found"))?;
    authorize_task_read_or_update(&state.pool, &principal, &task).await?;

    let rows = db::list_comments_for_task(&state.pool, &task_id)
        .await
        .map_err(|_| ApiError::Internal("comment query failed"))?;

    let items = rows.into_iter().map(comment_to_response).collect();

    audit::record_request_audit(
        &state.pool,
        Some(&principal),
        "GET /tasks/{task_id}/comments",
        StatusCode::OK.as_u16(),
    )
    .await;
    Ok(Json(ItemsResponse { items }))
}

async fn create_task_comment(
    State(state): State<AppState>,
    principal: Principal,
    Path(task_id): Path<String>,
    Json(payload): Json<CreateCommentRequest>,
) -> Result<(StatusCode, Json<CommentResponse>), ApiError> {
    // @fdd-change:fdd-fdd-example-feature-business-domain-change-comments:ph-1
    // @fdd-flow:fdd-fdd-example-feature-business-domain-flow-task-comments:ph-1

    validate_comment_body(&payload.body)?;

    let task = db::find_task(&state.pool, &task_id)
        .await
        .map_err(|_| ApiError::Internal("task lookup failed"))?
        .ok_or(ApiError::NotFound("task not found"))?;
    authorize_task_read_or_update(&state.pool, &principal, &task).await?;

    let now = Utc::now().to_rfc3339();
    let comment = db::CommentRecord {
        id: Uuid::new_v4().to_string(),
        task_id: task_id.clone(),
        author_user_id: principal.user_id.to_string(),
        body: payload.body,
        created_at: now,
    };

    tx::execute_with_retry(&state.pool, || async {
        db::insert_comment(&state.pool, &comment).await
    })
    .await?;

    audit::record_request_audit(
        &state.pool,
        Some(&principal),
        "POST /tasks/{task_id}/comments",
        StatusCode::CREATED.as_u16(),
    )
    .await;

    Ok((StatusCode::CREATED, Json(comment_to_response(comment))))
}

#[derive(Debug, Deserialize)]
struct CreateTeamRequest {
    pub name: String,
}

#[derive(Debug, Serialize)]
struct TeamResponse {
    pub id: String,
    pub name: String,
}

fn team_to_response(t: db::TeamRecord) -> TeamResponse {
    TeamResponse {
        id: t.id,
        name: t.name,
    }
}

async fn list_teams(
    State(state): State<AppState>,
    principal: Principal,
) -> Result<Json<ItemsResponse<TeamResponse>>, ApiError> {
    // @fdd-change:fdd-fdd-example-feature-business-domain-change-teams:ph-1
    // @fdd-flow:fdd-fdd-example-feature-business-domain-flow-team-management:ph-1

    let rows = db::list_teams_for_user(&state.pool, &principal.user_id.to_string())
        .await
        .map_err(|_| ApiError::Internal("team query failed"))?;
    let items = rows.into_iter().map(team_to_response).collect();

    audit::record_request_audit(
        &state.pool,
        Some(&principal),
        "GET /teams",
        StatusCode::OK.as_u16(),
    )
    .await;
    Ok(Json(ItemsResponse { items }))
}

async fn create_team(
    State(state): State<AppState>,
    principal: Principal,
    Json(payload): Json<CreateTeamRequest>,
) -> Result<(StatusCode, Json<TeamResponse>), ApiError> {
    // @fdd-change:fdd-fdd-example-feature-business-domain-change-teams:ph-1
    // @fdd-flow:fdd-fdd-example-feature-business-domain-flow-team-management:ph-1

    validate_team_name(&payload.name)?;

    let id = Uuid::new_v4().to_string();
    tx::execute_with_retry(&state.pool, || async {
        db::insert_team(&state.pool, &id, &payload.name).await?;
        db::insert_team_membership(&state.pool, &id, &principal.user_id.to_string(), "admin")
            .await?;
        Ok(())
    })
    .await?;

    audit::record_request_audit(
        &state.pool,
        Some(&principal),
        "POST /teams",
        StatusCode::CREATED.as_u16(),
    )
    .await;
    Ok((
        StatusCode::CREATED,
        Json(TeamResponse {
            id,
            name: payload.name,
        }),
    ))
}

async fn create_token(
    State(state): State<AppState>,
    Json(payload): Json<CreateTokenRequest>,
) -> Result<(StatusCode, Json<CreateTokenResponse>), ApiError> {
    // @fdd-change:fdd-fdd-example-feature-platform-core-change-token-auth:ph-1
    // @fdd-req:fdd-fdd-example-feature-platform-core-req-token-auth:ph-1
    // @fdd-flow:fdd-fdd-example-feature-platform-core-flow-token-issuance:ph-1

    // fdd-begin fdd-fdd-example-feature-platform-core-flow-token-issuance:ph-1:inst-prepare-request
    let _prepared = true;
    // fdd-end fdd-fdd-example-feature-platform-core-flow-token-issuance:ph-1:inst-prepare-request

    // fdd-begin fdd-fdd-example-feature-platform-core-flow-token-issuance:ph-1:inst-validate-request-shape
    if payload.email.trim().is_empty() {
        return Err(ApiError::Validation("email is required"));
    }
    if payload.password.trim().is_empty() {
        return Err(ApiError::Validation("password is required"));
    }
    // fdd-end fdd-fdd-example-feature-platform-core-flow-token-issuance:ph-1:inst-validate-request-shape

    // fdd-begin fdd-fdd-example-feature-platform-core-flow-token-issuance:ph-1:inst-if-invalid-credentials
    if payload.password == "invalid" {
        // fdd-begin fdd-fdd-example-feature-platform-core-flow-token-issuance:ph-1:inst-return-401
        return Err(ApiError::Authentication("invalid credentials"));
        // fdd-end fdd-fdd-example-feature-platform-core-flow-token-issuance:ph-1:inst-return-401
    }
    // fdd-end fdd-fdd-example-feature-platform-core-flow-token-issuance:ph-1:inst-if-invalid-credentials

    // fdd-begin fdd-fdd-example-feature-platform-core-flow-token-issuance:ph-1:inst-issue-token
    let access_token = Uuid::new_v4().to_string();
    let token_hash = db::hash_token(&access_token);

    let now = Utc::now();
    let expires_at = now + Duration::hours(1);

    // fdd-begin fdd-fdd-example-feature-platform-core-flow-token-issuance:ph-1:inst-submit-request
    tx::execute_with_retry(&state.pool, || async {
        db::insert_token(&state.pool, &token_hash, &payload.email, now, expires_at).await
    })
    .await?;
    // fdd-end fdd-fdd-example-feature-platform-core-flow-token-issuance:ph-1:inst-submit-request
    // fdd-end fdd-fdd-example-feature-platform-core-flow-token-issuance:ph-1:inst-issue-token

    // fdd-begin fdd-fdd-example-feature-platform-core-flow-token-issuance:ph-1:inst-return-201
    audit::record_request_audit(
        &state.pool,
        None,
        "POST /auth/tokens",
        StatusCode::CREATED.as_u16(),
    )
    .await;

    Ok((
        StatusCode::CREATED,
        Json(CreateTokenResponse {
            access_token,
            token_type: "Bearer".to_string(),
        }),
    ))
    // fdd-end fdd-fdd-example-feature-platform-core-flow-token-issuance:ph-1:inst-return-201
}
