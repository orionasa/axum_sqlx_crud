use axum::{
    extract::rejection::QueryRejection,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;

#[derive(Debug)]
pub enum ApiError {
    DatabaseError,
    ValidationError(String),
    QueryParseError(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            ApiError::DatabaseError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string())
            }
            ApiError::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg),
            ApiError::QueryParseError(msg) => (StatusCode::BAD_REQUEST, msg),
        };

        (
            status,
            Json(json!({
                "success": false,
                "error": {
                    "message": message,
                    "code": status.as_u16()
                }
            }))
        ).into_response()
    }
}

impl From<QueryRejection> for ApiError {
    fn from(rejection: QueryRejection) -> Self {
        ApiError::QueryParseError(format!(
            "Invalid query parameters: {}",
            rejection.body_text()
        ))
    }
}
