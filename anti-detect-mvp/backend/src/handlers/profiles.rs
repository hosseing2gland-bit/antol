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
    // Generate default values based on what's provided
    let user_agent = req.user_agent.unwrap_or_else(|| "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36".to_string());
    let timezone = req.timezone.unwrap_or_else(|| "America/New_York".to_string());
    let locale = req.locale.unwrap_or_else(|| "en-US".to_string());
    let canvas_noise = req.canvas_noise.unwrap_or(true);
    let webgl_noise = req.webgl_noise.unwrap_or(true);
    let audio_noise = req.audio_noise.unwrap_or(true);
    
    // Generate random fingerprint
    let fingerprint = serde_json::json!({
        "canvas": canvas_noise,
        "webgl": webgl_noise,
        "audio": audio_noise
    });

    let profile = sqlx::query_as::<_, Profile>(
        r#"
        INSERT INTO profiles (
            user_id, name, fingerprint, proxy_id, user_agent, timezone,
            locale, webgl_vendor, webgl_renderer, canvas_noise,
            webgl_noise, audio_noise, is_active
        )
        VALUES (
            '00000000-0000-0000-0000-000000000000'::uuid, $1, $2, $3, $4, $5,
            $6, 'Intel Inc.', 'Intel Iris OpenGL Engine', $7,
            $8, $9, true
        )
        RETURNING *
        "#
    )
    .bind(&req.name)
    .bind(&fingerprint)
    .bind(&req.proxy_id)
    .bind(&user_agent)
    .bind(&timezone)
    .bind(&locale)
    .bind(canvas_noise)
    .bind(webgl_noise)
    .bind(audio_noise)
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
    if let Some(proxy_id) = &req.proxy_id {
        updates.push(format!("proxy_id = ${}", param_count));
        param_count += 1;
    }
    if let Some(is_active) = req.is_active {
        updates.push(format!("is_active = ${}", param_count));
        param_count += 1;
    }
    if let Some(canvas_noise) = req.canvas_noise {
        updates.push(format!("canvas_noise = ${}", param_count));
        param_count += 1;
    }
    if let Some(webgl_noise) = req.webgl_noise {
        updates.push(format!("webgl_noise = ${}", param_count));
        param_count += 1;
    }
    if let Some(audio_noise) = req.audio_noise {
        updates.push(format!("audio_noise = ${}", param_count));
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
    if let Some(proxy_id) = &req.proxy_id {
        sql_query = sql_query.bind(proxy_id);
    }
    if let Some(is_active) = req.is_active {
        sql_query = sql_query.bind(is_active);
    }
    if let Some(canvas_noise) = req.canvas_noise {
        sql_query = sql_query.bind(canvas_noise);
    }
    if let Some(webgl_noise) = req.webgl_noise {
        sql_query = sql_query.bind(webgl_noise);
    }
    if let Some(audio_noise) = req.audio_noise {
        sql_query = sql_query.bind(audio_noise);
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
