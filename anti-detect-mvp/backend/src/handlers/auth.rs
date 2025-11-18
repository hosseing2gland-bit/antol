use axum::{extract::State, http::StatusCode, Json};
use bcrypt::{hash, DEFAULT_COST};
use sqlx::PgPool;
use uuid::Uuid;
use crate::models::user::{User, NewUser};

pub async fn register(
    State(pool): State<PgPool>,
    Json(payload): Json<NewUser>,
) -> Result<Json<User>, StatusCode> {
    let password_hash = hash(payload.password, DEFAULT_COST).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let user = sqlx::query_as!(
        User,
        "INSERT INTO users (id, email, password_hash) VALUES ($1, $2, $3) RETURNING *",
        Uuid::new_v4(),
        payload.email,
        password_hash
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(user))
}

pub async fn login() -> &'static str {
    "Login endpoint"
}
