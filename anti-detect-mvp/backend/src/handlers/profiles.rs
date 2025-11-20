use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use crate::models::{Profile, CreateProfileRequest, UpdateProfileRequest};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn list_profiles(
    State(pool): State<PgPool>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let profiles = sqlx::query_as::<_, Profile>("SELECT * FROM profiles ORDER BY created_at DESC")
        .fetch_all(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(profiles))
}

pub async fn get_profile(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let profile = sqlx::query_as::<_, Profile>("SELECT * FROM profiles WHERE id = $1")
        .bind(id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Profile not found".to_string()))?;

    Ok(Json(profile))
}

pub async fn create_profile(
    State(pool): State<PgPool>,
    Json(req): Json<CreateProfileRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    // Check user's license limit
    let license = sqlx::query_as::<_, (i32,)>(
        "SELECT max_profiles FROM licenses WHERE user_id = $1 AND is_active = true AND expires_at > NOW()"
    )
    .bind(&req.user_id)
    .fetch_optional(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .ok_or((StatusCode::FORBIDDEN, "No active license found".to_string()))?;

    let current_count = sqlx::query_as::<_, (i64,)>(
        "SELECT COUNT(*) FROM profiles WHERE user_id = $1"
    )
    .bind(&req.user_id)
    .fetch_one(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .0;

    if current_count >= license.0 as i64 {
        return Err((StatusCode::FORBIDDEN, "Profile limit reached".to_string()));
    }

    let profile = sqlx::query_as::<_, Profile>(
        r#"
        INSERT INTO profiles (
            user_id, name, user_agent, screen_resolution, timezone,
            language, webgl_vendor, webgl_renderer, canvas_noise,
            audio_noise, fonts, proxy_id
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
        RETURNING *
        "#
    )
    .bind(&req.user_id)
    .bind(&req.name)
    .bind(&req.user_agent)
    .bind(&req.screen_resolution)
    .bind(&req.timezone)
    .bind(&req.language)
    .bind(&req.webgl_vendor)
    .bind(&req.webgl_renderer)
    .bind(req.canvas_noise.unwrap_or(true))
    .bind(req.audio_noise.unwrap_or(true))
    .bind(&req.fonts)
    .bind(&req.proxy_id)
    .fetch_one(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((StatusCode::CREATED, Json(profile)))
}

pub async fn update_profile(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateProfileRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut query = String::from("UPDATE profiles SET ");
    let mut updates = Vec::new();
    let mut param_count = 1;

    if let Some(name) = &req.name {
        updates.push(format!("name = ${}", param_count));
        param_count += 1;
    }
    if let Some(user_agent) = &req.user_agent {
        updates.push(format!("user_agent = ${}", param_count));
        param_count += 1;
    }
    if let Some(screen_resolution) = &req.screen_resolution {
        updates.push(format!("screen_resolution = ${}", param_count));
        param_count += 1;
    }
    if let Some(timezone) = &req.timezone {
        updates.push(format!("timezone = ${}", param_count));
        param_count += 1;
    }
    if let Some(language) = &req.language {
        updates.push(format!("language = ${}", param_count));
        param_count += 1;
    }
    if let Some(proxy_id) = &req.proxy_id {
        updates.push(format!("proxy_id = ${}", param_count));
        param_count += 1;
    }

    if updates.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "No fields to update".to_string()));
    }

    query.push_str(&updates.join(", "));
    query.push_str(&format!(" WHERE id = ${} RETURNING *", param_count));

    let mut sql_query = sqlx::query_as::<_, Profile>(&query);
    
    if let Some(name) = &req.name {
        sql_query = sql_query.bind(name);
    }
    if let Some(user_agent) = &req.user_agent {
        sql_query = sql_query.bind(user_agent);
    }
    if let Some(screen_resolution) = &req.screen_resolution {
        sql_query = sql_query.bind(screen_resolution);
    }
    if let Some(timezone) = &req.timezone {
        sql_query = sql_query.bind(timezone);
    }
    if let Some(language) = &req.language {
        sql_query = sql_query.bind(language);
    }
    if let Some(proxy_id) = &req.proxy_id {
        sql_query = sql_query.bind(proxy_id);
    }
    sql_query = sql_query.bind(id);

    let profile = sql_query
        .fetch_optional(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Profile not found".to_string()))?;

    Ok(Json(profile))
}

pub async fn delete_profile(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let result = sqlx::query("DELETE FROM profiles WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if result.rows_affected() == 0 {
        return Err((StatusCode::NOT_FOUND, "Profile not found".to_string()));
    }

    Ok(StatusCode::NO_CONTENT)
}
