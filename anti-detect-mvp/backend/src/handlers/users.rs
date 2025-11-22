use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use crate::models::{User, CreateUserRequest, UpdateUserRequest};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn list_users(
    State(pool): State<PgPool>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let users = sqlx::query_as::<_, User>("SELECT * FROM users ORDER BY created_at DESC")
        .fetch_all(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(users))
}

pub async fn get_user(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "User not found".to_string()))?;

    Ok(Json(user))
}

pub async fn create_user(
    State(pool): State<PgPool>,
    Json(req): Json<CreateUserRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    // Hash password
    let hashed_password = bcrypt::hash(&req.password, bcrypt::DEFAULT_COST)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let role = req.role.unwrap_or(crate::models::UserRole::User);
    
    let user = sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (email, password_hash, role)
        VALUES ($1, $2, $3)
        RETURNING *
        "#
    )
    .bind(&req.email)
    .bind(&hashed_password)
    .bind(&role.to_string())
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        if e.to_string().contains("duplicate key") {
            (StatusCode::CONFLICT, "Email already exists".to_string())
        } else {
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        }
    })?;

    Ok((StatusCode::CREATED, Json(user)))
}

pub async fn update_user(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateUserRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut query = String::from("UPDATE users SET ");
    let mut updates = Vec::new();
    let mut param_count = 1;

    if let Some(_email) = &req.email {
        updates.push(format!("email = ${}", param_count));
        param_count += 1;
    }
    if let Some(_role) = &req.role {
        updates.push(format!("role = ${}", param_count));
        param_count += 1;
    }
    // Note: is_active field removed from schema

    if updates.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "No fields to update".to_string()));
    }

    query.push_str(&updates.join(", "));
    query.push_str(&format!(" WHERE id = ${} RETURNING *", param_count));

    let mut sql_query = sqlx::query_as::<_, User>(&query);
    
    if let Some(email) = &req.email {
        sql_query = sql_query.bind(email);
    }
    if let Some(role) = &req.role {
        let role_str = role.to_string();
        sql_query = sql_query.bind(role_str);
    }
    sql_query = sql_query.bind(id);

    let user = sql_query
        .fetch_optional(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "User not found".to_string()))?;

    Ok(Json(user))
}

pub async fn delete_user(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let result = sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if result.rows_affected() == 0 {
        return Err((StatusCode::NOT_FOUND, "User not found".to_string()));
    }

    Ok(StatusCode::NO_CONTENT)
}
