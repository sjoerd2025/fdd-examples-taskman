use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;

#[derive(Debug, Clone, Copy)]
pub enum ErrorCategory {
    Validation,
    Authentication,
    Authorization,
    NotFound,
    Conflict,
    Transient,
    Unknown,
}

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("validation error")]
    Validation(&'static str),
    #[error("authentication required")]
    Authentication(&'static str),
    #[error("permission denied")]
    Authorization(&'static str),
    #[error("not found")]
    NotFound(&'static str),
    #[error("conflict")]
    Conflict(&'static str),
    #[error("transient failure")]
    Transient(&'static str),
    #[error("internal error")]
    Internal(&'static str),
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub code: String,
    pub message: String,
}

impl ApiError {
    pub fn category(&self) -> ErrorCategory {
        match self {
            ApiError::Validation(_) => ErrorCategory::Validation,
            ApiError::Authentication(_) => ErrorCategory::Authentication,
            ApiError::Authorization(_) => ErrorCategory::Authorization,
            ApiError::NotFound(_) => ErrorCategory::NotFound,
            ApiError::Conflict(_) => ErrorCategory::Conflict,
            ApiError::Transient(_) => ErrorCategory::Transient,
            ApiError::Internal(_) => ErrorCategory::Unknown,
        }
    }

    pub fn error_code(&self) -> &'static str {
        match self {
            ApiError::Validation(_) => "validation_error",
            ApiError::Authentication(_) => "authentication_required",
            ApiError::Authorization(_) => "permission_denied",
            ApiError::NotFound(_) => "not_found",
            ApiError::Conflict(_) => "conflict",
            ApiError::Transient(_) => "transient_failure",
            ApiError::Internal(_) => "internal_error",
        }
    }

    pub fn message(&self) -> &'static str {
        match self {
            ApiError::Validation(m)
            | ApiError::Authentication(m)
            | ApiError::Authorization(m)
            | ApiError::NotFound(m)
            | ApiError::Conflict(m)
            | ApiError::Transient(m)
            | ApiError::Internal(m) => m,
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        // @fdd-change:fdd-fdd-example-feature-platform-core-change-openapi-boundary:ph-1
        // @fdd-req:fdd-fdd-example-feature-platform-core-req-openapi-enforcement:ph-1
        // @fdd-algo:fdd-fdd-example-feature-platform-core-algo-map-errors:ph-1
        let category = self.category();

        // fdd-begin fdd-fdd-example-feature-platform-core-algo-map-errors:ph-1:inst-identify-category
        let status = match category {
            // fdd-begin fdd-fdd-example-feature-platform-core-algo-map-errors:ph-1:inst-if-validation
            // fdd-begin fdd-fdd-example-feature-platform-core-algo-map-errors:ph-1:inst-choose-400
            ErrorCategory::Validation => StatusCode::BAD_REQUEST,
            // fdd-end fdd-fdd-example-feature-platform-core-algo-map-errors:ph-1:inst-choose-400
            // fdd-end fdd-fdd-example-feature-platform-core-algo-map-errors:ph-1:inst-if-validation

            // fdd-begin fdd-fdd-example-feature-platform-core-algo-map-errors:ph-1:inst-if-authentication
            // fdd-begin fdd-fdd-example-feature-platform-core-algo-map-errors:ph-1:inst-choose-401
            ErrorCategory::Authentication => StatusCode::UNAUTHORIZED,
            // fdd-end fdd-fdd-example-feature-platform-core-algo-map-errors:ph-1:inst-choose-401
            // fdd-end fdd-fdd-example-feature-platform-core-algo-map-errors:ph-1:inst-if-authentication

            // fdd-begin fdd-fdd-example-feature-platform-core-algo-map-errors:ph-1:inst-if-authorization
            // fdd-begin fdd-fdd-example-feature-platform-core-algo-map-errors:ph-1:inst-choose-403
            ErrorCategory::Authorization => StatusCode::FORBIDDEN,
            // fdd-end fdd-fdd-example-feature-platform-core-algo-map-errors:ph-1:inst-choose-403
            // fdd-end fdd-fdd-example-feature-platform-core-algo-map-errors:ph-1:inst-if-authorization

            // fdd-begin fdd-fdd-example-feature-platform-core-algo-map-errors:ph-1:inst-if-not-found
            // fdd-begin fdd-fdd-example-feature-platform-core-algo-map-errors:ph-1:inst-choose-404
            ErrorCategory::NotFound => StatusCode::NOT_FOUND,
            // fdd-end fdd-fdd-example-feature-platform-core-algo-map-errors:ph-1:inst-choose-404
            // fdd-end fdd-fdd-example-feature-platform-core-algo-map-errors:ph-1:inst-if-not-found

            // fdd-begin fdd-fdd-example-feature-platform-core-algo-map-errors:ph-1:inst-if-conflict
            // fdd-begin fdd-fdd-example-feature-platform-core-algo-map-errors:ph-1:inst-choose-409
            ErrorCategory::Conflict => StatusCode::CONFLICT,
            // fdd-end fdd-fdd-example-feature-platform-core-algo-map-errors:ph-1:inst-choose-409
            // fdd-end fdd-fdd-example-feature-platform-core-algo-map-errors:ph-1:inst-if-conflict

            // fdd-begin fdd-fdd-example-feature-platform-core-algo-map-errors:ph-1:inst-if-transient
            // fdd-begin fdd-fdd-example-feature-platform-core-algo-map-errors:ph-1:inst-choose-503
            ErrorCategory::Transient => StatusCode::SERVICE_UNAVAILABLE,
            // fdd-end fdd-fdd-example-feature-platform-core-algo-map-errors:ph-1:inst-choose-503
            // fdd-end fdd-fdd-example-feature-platform-core-algo-map-errors:ph-1:inst-if-transient

            // fdd-begin fdd-fdd-example-feature-platform-core-algo-map-errors:ph-1:inst-default-500
            ErrorCategory::Unknown => StatusCode::INTERNAL_SERVER_ERROR,
            // fdd-end fdd-fdd-example-feature-platform-core-algo-map-errors:ph-1:inst-default-500
        };
        // fdd-end fdd-fdd-example-feature-platform-core-algo-map-errors:ph-1:inst-identify-category

        // fdd-begin fdd-fdd-example-feature-platform-core-algo-map-errors:ph-1:inst-ensure-schema
        let body = ErrorResponse {
            code: self.error_code().to_string(),
            message: self.message().to_string(),
        };

        // fdd-begin fdd-fdd-example-feature-platform-core-algo-map-errors:ph-1:inst-return-selected
        let resp = (status, Json(body)).into_response();
        // fdd-end fdd-fdd-example-feature-platform-core-algo-map-errors:ph-1:inst-return-selected
        // fdd-end fdd-fdd-example-feature-platform-core-algo-map-errors:ph-1:inst-ensure-schema

        resp
    }
}
