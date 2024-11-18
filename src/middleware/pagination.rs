use axum::{
    async_trait,
    extract::{FromRequestParts, Query},
    http::{request::Parts, Request},
    middleware::Next,
    response::IntoResponse,
};
use serde::Deserialize;

use crate::error::ApiError;

// Constants for pagination limits
pub const MAX_PAGE_SIZE: u32 = 100;
pub const DEFAULT_PAGE_SIZE: u32 = 10;
pub const DEFAULT_PAGE: u32 = 1;

#[derive(Debug, Deserialize, Clone)]
pub struct PaginationParams {
    pub page: Option<String>,
    pub per_page: Option<String>,
}

impl PaginationParams {
    pub fn validate(&self) -> Result<(u32, u32), ApiError> {
        // Parse page number
        let page = match &self.page {
            Some(p) => p.parse::<u32>().map_err(|_| {
                ApiError::ValidationError("Page number must be a valid positive number".to_string())
            })?,
            None => DEFAULT_PAGE,
        };

        // Parse items per page
        let per_page = match &self.per_page {
            Some(p) => p.parse::<u32>().map_err(|_| {
                ApiError::ValidationError("Items per page must be a valid positive number".to_string())
            })?,
            None => DEFAULT_PAGE_SIZE,
        };

        if page == 0 {
            return Err(ApiError::ValidationError(
                "Page number must be greater than 0".to_string(),
            ));
        }

        if per_page == 0 {
            return Err(ApiError::ValidationError(
                "Items per page must be greater than 0".to_string(),
            ));
        }

        if per_page > MAX_PAGE_SIZE {
            return Err(ApiError::ValidationError(
                format!("Items per page cannot exceed {}", MAX_PAGE_SIZE)
            ));
        }

        Ok((page, per_page))
    }
}

impl Default for PaginationParams {
    fn default() -> Self {
        Self {
            page: Some(DEFAULT_PAGE.to_string()),
            per_page: Some(DEFAULT_PAGE_SIZE.to_string()),
        }
    }
}

pub struct Pagination(pub PaginationParams);

#[async_trait]
impl<S> FromRequestParts<S> for Pagination
where
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let params = parts
            .extensions
            .get::<PaginationParams>()
            .cloned()
            .unwrap_or_default();
        Ok(Pagination(params))
    }
}

pub async fn pagination_middleware(
    Query(params): Query<PaginationParams>,
    mut request: Request<axum::body::Body>,
    next: Next,
) -> Result<impl IntoResponse, ApiError> {
    let (page, per_page) = params.validate()?;

    request.extensions_mut().insert(PaginationParams {
        page: Some(page.to_string()),
        per_page: Some(per_page.to_string()),
    });

    Ok(next.run(request).await)
}
