use axum::{extract::State, Json};

use crate::{
    db::DbPool,
    error::ApiError,
    middleware::Pagination,
    models::{ApiResponse, Item, PaginatedResponse, PaginationMeta},
};

pub async fn get_items(
    pagination: Pagination,
    State(pool): State<DbPool>,
) -> Result<Json<ApiResponse<PaginatedResponse<Item>>>, ApiError> {
    let (page, per_page) = pagination.0.validate()?;
    let offset = (page - 1) * per_page;

    // Get total count
    let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM items")
        .fetch_one(&*pool)
        .await
        .map_err(|_| ApiError::DatabaseError)?;

    // Get paginated items
    let items: Vec<Item> = sqlx::query_as!(
        Item,
        "SELECT id, name FROM items ORDER BY id LIMIT $1 OFFSET $2",
        per_page as i64,
        offset as i64
    )
    .fetch_all(&*pool)
    .await
    .map_err(|_| ApiError::DatabaseError)?;

    let total_pages = ((total.0 as f64) / (per_page as f64)).ceil() as u32;

    Ok(Json(ApiResponse {
        success: true,
        data: PaginatedResponse {
            data: items,
            meta: PaginationMeta {
                current_page: page,
                per_page,
                total_items: total.0,
                total_pages,
            },
        },
    }))
}
