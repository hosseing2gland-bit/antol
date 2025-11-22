use axum::{extract::State, http::StatusCode, Json};
use sqlx::PgPool;

use crate::models::DashboardStats;

pub async fn get_stats(
    State(pool): State<PgPool>,
) -> Result<Json<DashboardStats>, (StatusCode, String)> {
    let (total_users,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
        .fetch_one(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let (total_licenses,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM licenses")
        .fetch_one(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let (total_profiles,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM profiles")
        .fetch_one(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let (total_proxies,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM proxies")
        .fetch_one(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(DashboardStats {
        total_users,
        total_licenses,
        total_profiles,
        total_proxies,
    }))
}
