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
    // Generate default fingerprint config if not provided
    let fingerprint_config = req.fingerprint_config.unwrap_or_else(|| {
        serde_json::json!({
            "user_agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36",
            "timezone": "America/New_York",
            "locale": "en-US",
            "canvas_noise": true,
            "webgl_noise": true,
            "audio_noise": true,
            "webgl_vendor": "Google Inc.",
            "webgl_renderer": "ANGLE (Intel)"
        })
    });

    let profile = sqlx::query_as::<_, Profile>(
        r#"
        INSERT INTO profiles (
            user_id, name, fingerprint_config, proxy_id, notes
        )
        VALUES (
            '00000000-0000-0000-0000-000000000000'::uuid, $1, $2, $3, $4
        )
        RETURNING *
        "#
    )
    .bind(&req.name)
    .bind(&fingerprint_config)
    .bind(&req.proxy_id)
    .bind(&req.notes)
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
    if let Some(fingerprint_config) = &req.fingerprint_config {
        updates.push(format!("fingerprint_config = ${}", param_count));
        param_count += 1;
    }
    if let Some(notes) = &req.notes {
        updates.push(format!("notes = ${}", param_count));
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
    if let Some(fingerprint_config) = &req.fingerprint_config {
        sql_query = sql_query.bind(fingerprint_config);
    }
    if let Some(notes) = &req.notes {
        sql_query = sql_query.bind(notes);
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
