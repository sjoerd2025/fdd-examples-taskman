use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, HeaderMap},
};
use chrono::Utc;
use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::app::AppState;
use crate::{db, error::ApiError};

#[derive(Clone, Debug)]
pub struct Principal {
    pub user_email: String,
    pub user_id: Uuid,
}

fn stable_user_id(email: &str) -> Uuid {
    let mut hasher = Sha256::new();
    hasher.update(email.as_bytes());
    let digest = hasher.finalize();

    let mut bytes = [0u8; 16];
    bytes.copy_from_slice(&digest[..16]);
    Uuid::from_bytes(bytes)
}

fn extract_bearer(headers: &HeaderMap) -> Option<String> {
    headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .map(|s| s.to_string())
}

#[async_trait]
impl FromRequestParts<AppState> for Principal {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        // @fdd-change:fdd-fdd-example-feature-platform-core-change-token-auth:ph-1
        // @fdd-req:fdd-fdd-example-feature-platform-core-req-token-auth:ph-1
        // @fdd-flow:fdd-fdd-example-feature-platform-core-flow-authenticated-call:ph-1
        // @fdd-algo:fdd-fdd-example-feature-platform-core-algo-verify-bearer-token:ph-1
        // @fdd-state:fdd-fdd-example-feature-platform-core-state-access-token:ph-1

        let headers = &parts.headers;

        // fdd-begin fdd-fdd-example-feature-platform-core-flow-authenticated-call:ph-1:inst-authenticate-bearer

        // fdd-begin fdd-fdd-example-feature-platform-core-algo-verify-bearer-token:ph-1:inst-extract-auth-header
        let raw = extract_bearer(headers);
        // fdd-end fdd-fdd-example-feature-platform-core-algo-verify-bearer-token:ph-1:inst-extract-auth-header

        // fdd-begin fdd-fdd-example-feature-platform-core-algo-verify-bearer-token:ph-1:inst-parse-bearer
        let auth_header = headers
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok());
        // fdd-end fdd-fdd-example-feature-platform-core-algo-verify-bearer-token:ph-1:inst-parse-bearer

        // fdd-begin fdd-fdd-example-feature-platform-core-algo-verify-bearer-token:ph-1:inst-if-missing-header
        // fdd-begin fdd-fdd-example-feature-platform-core-flow-authenticated-call:ph-1:inst-if-token-invalid
        let token = match (auth_header, raw) {
            (None, _) => {
                // fdd-begin fdd-fdd-example-feature-platform-core-algo-verify-bearer-token:ph-1:inst-return-missing-token
                // fdd-begin fdd-fdd-example-feature-platform-core-flow-authenticated-call:ph-1:inst-return-401-invalid
                return Err(ApiError::Authentication("missing bearer token"));
                // fdd-end fdd-fdd-example-feature-platform-core-flow-authenticated-call:ph-1:inst-return-401-invalid
                // fdd-end fdd-fdd-example-feature-platform-core-algo-verify-bearer-token:ph-1:inst-return-missing-token
            }
            (Some(h), None) => {
                // fdd-begin fdd-fdd-example-feature-platform-core-algo-verify-bearer-token:ph-1:inst-if-invalid-format
                // fdd-begin fdd-fdd-example-feature-platform-core-algo-verify-bearer-token:ph-1:inst-return-invalid-format
                // fdd-begin fdd-fdd-example-feature-platform-core-flow-authenticated-call:ph-1:inst-return-401-invalid
                let _ = h;
                return Err(ApiError::Authentication("invalid bearer token format"));
                // fdd-end fdd-fdd-example-feature-platform-core-flow-authenticated-call:ph-1:inst-return-401-invalid
                // fdd-end fdd-fdd-example-feature-platform-core-algo-verify-bearer-token:ph-1:inst-return-invalid-format
                // fdd-end fdd-fdd-example-feature-platform-core-algo-verify-bearer-token:ph-1:inst-if-invalid-format
            }
            (_, Some(t)) => t,
        };
        // fdd-end fdd-fdd-example-feature-platform-core-flow-authenticated-call:ph-1:inst-if-token-invalid
        // fdd-end fdd-fdd-example-feature-platform-core-algo-verify-bearer-token:ph-1:inst-if-missing-header

        // fdd-begin fdd-fdd-example-feature-platform-core-algo-verify-bearer-token:ph-1:inst-lookup-token
        let token_hash = db::hash_token(&token);
        let record = db::find_token(&state.pool, &token_hash)
            .await
            .map_err(|_| ApiError::Internal("token lookup failed"))?;
        // fdd-end fdd-fdd-example-feature-platform-core-algo-verify-bearer-token:ph-1:inst-lookup-token

        // fdd-begin fdd-fdd-example-feature-platform-core-algo-verify-bearer-token:ph-1:inst-if-token-not-found
        let record = match record {
            Some(r) => r,
            None => {
                // fdd-begin fdd-fdd-example-feature-platform-core-algo-verify-bearer-token:ph-1:inst-return-unknown-token
                // fdd-begin fdd-fdd-example-feature-platform-core-flow-authenticated-call:ph-1:inst-return-401-invalid
                return Err(ApiError::Authentication("unknown token"));
                // fdd-end fdd-fdd-example-feature-platform-core-flow-authenticated-call:ph-1:inst-return-401-invalid
                // fdd-end fdd-fdd-example-feature-platform-core-algo-verify-bearer-token:ph-1:inst-return-unknown-token
            }
        };
        // fdd-end fdd-fdd-example-feature-platform-core-algo-verify-bearer-token:ph-1:inst-if-token-not-found

        // fdd-begin fdd-fdd-example-feature-platform-core-algo-verify-bearer-token:ph-1:inst-if-token-inactive
        let now = Utc::now();
        if record.expires_at <= now {
            // fdd-begin fdd-fdd-example-feature-platform-core-state-access-token:ph-1:inst-active-to-expired
            // fdd-begin fdd-fdd-example-feature-platform-core-state-access-token:ph-1:inst-expired-stays-expired
            // fdd-begin fdd-fdd-example-feature-platform-core-algo-verify-bearer-token:ph-1:inst-return-inactive-token
            // fdd-begin fdd-fdd-example-feature-platform-core-flow-authenticated-call:ph-1:inst-return-401-invalid
            return Err(ApiError::Authentication("expired token"));
            // fdd-end fdd-fdd-example-feature-platform-core-flow-authenticated-call:ph-1:inst-return-401-invalid
            // fdd-end fdd-fdd-example-feature-platform-core-algo-verify-bearer-token:ph-1:inst-return-inactive-token
            // fdd-end fdd-fdd-example-feature-platform-core-state-access-token:ph-1:inst-expired-stays-expired
            // fdd-end fdd-fdd-example-feature-platform-core-state-access-token:ph-1:inst-active-to-expired
        }
        if record.revoked_at.is_some() {
            // fdd-begin fdd-fdd-example-feature-platform-core-state-access-token:ph-1:inst-active-to-revoked
            // fdd-begin fdd-fdd-example-feature-platform-core-state-access-token:ph-1:inst-revoked-stays-revoked
            // fdd-begin fdd-fdd-example-feature-platform-core-algo-verify-bearer-token:ph-1:inst-return-inactive-token
            // fdd-begin fdd-fdd-example-feature-platform-core-flow-authenticated-call:ph-1:inst-return-401-invalid
            return Err(ApiError::Authentication("revoked token"));
            // fdd-end fdd-fdd-example-feature-platform-core-flow-authenticated-call:ph-1:inst-return-401-invalid
            // fdd-end fdd-fdd-example-feature-platform-core-algo-verify-bearer-token:ph-1:inst-return-inactive-token
            // fdd-end fdd-fdd-example-feature-platform-core-state-access-token:ph-1:inst-revoked-stays-revoked
            // fdd-end fdd-fdd-example-feature-platform-core-state-access-token:ph-1:inst-active-to-revoked
        }
        // fdd-end fdd-fdd-example-feature-platform-core-algo-verify-bearer-token:ph-1:inst-if-token-inactive

        // fdd-begin fdd-fdd-example-feature-platform-core-flow-authenticated-call:ph-1:inst-evaluate-policy
        let allowed = true;
        // fdd-end fdd-fdd-example-feature-platform-core-flow-authenticated-call:ph-1:inst-evaluate-policy

        // fdd-begin fdd-fdd-example-feature-platform-core-flow-authenticated-call:ph-1:inst-if-policy-deny
        if !allowed {
            // fdd-begin fdd-fdd-example-feature-platform-core-flow-authenticated-call:ph-1:inst-return-403
            return Err(ApiError::Authorization("policy denied"));
            // fdd-end fdd-fdd-example-feature-platform-core-flow-authenticated-call:ph-1:inst-return-403
        }
        // fdd-end fdd-fdd-example-feature-platform-core-flow-authenticated-call:ph-1:inst-if-policy-deny

        // fdd-begin fdd-fdd-example-feature-platform-core-algo-verify-bearer-token:ph-1:inst-build-principal
        let user_id = stable_user_id(&record.user_email);
        // fdd-end fdd-fdd-example-feature-platform-core-algo-verify-bearer-token:ph-1:inst-build-principal

        // fdd-begin fdd-fdd-example-feature-platform-core-algo-verify-bearer-token:ph-1:inst-return-principal
        let principal = Principal {
            user_email: record.user_email,
            user_id,
        };
        // fdd-end fdd-fdd-example-feature-platform-core-algo-verify-bearer-token:ph-1:inst-return-principal

        // fdd-end fdd-fdd-example-feature-platform-core-flow-authenticated-call:ph-1:inst-authenticate-bearer

        Ok(principal)
    }
}
