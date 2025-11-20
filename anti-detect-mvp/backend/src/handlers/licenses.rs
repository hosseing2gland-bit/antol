use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use crate::models::{License, CreateLicenseRequest, LicensePlan};
use sqlx::PgPool;
use uuid::Uuid;
use chrono::{Utc, Duration};

pub async fn list_licenses(
    State(pool): State<PgPool>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let licenses = sqlx::query_as::<_, License>("SELECT * FROM licenses ORDER BY created_at DESC")
        .fetch_all(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(licenses))
}

pub async fn get_license(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let license = sqlx::query_as::<_, License>("SELECT * FROM licenses WHERE id = $1")
        .bind(id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "License not found".to_string()))?;

    Ok(Json(license))
}

pub async fn create_license(
    State(pool): State<PgPool>,
    Json(req): Json<CreateLicenseRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    // Generate license key
    let license_key = format!(
        "{}-{}-{}-{}",
        Uuid::new_v4().to_string()[..8].to_uppercase(),
        Uuid::new_v4().to_string()[..8].to_uppercase(),
        Uuid::new_v4().to_string()[..8].to_uppercase(),
        Uuid::new_v4().to_string()[..8].to_uppercase()
    );

    // Calculate expiry based on plan
    let expires_at = match req.plan {
        LicensePlan::Trial => Utc::now() + Duration::days(7),
        LicensePlan::Basic => Utc::now() + Duration::days(30),
        LicensePlan::Pro => Utc::now() + Duration::days(90),
        LicensePlan::Enterprise => Utc::now() + Duration::days(365),
    };

    let license = sqlx::query_as::<_, License>(
        r#"
        INSERT INTO licenses (license_key, plan, max_profiles, expires_at, is_active)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING *
        "#
    )
    .bind(&license_key)
    .bind(&req.plan)
    .bind(req.max_profiles)
    .bind(expires_at)
    .bind(true)
    .fetch_one(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((StatusCode::CREATED, Json(license)))
}

pub async fn activate_license(
    State(pool): State<PgPool>,
    Path(license_key): Path<String>,
    Json(user_id): Json<Uuid>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    // Check if license exists and is valid
    let license = sqlx::query_as::<_, License>(
        "SELECT * FROM licenses WHERE license_key = $1"
    )
    .bind(&license_key)
    .fetch_optional(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .ok_or((StatusCode::NOT_FOUND, "License not found".to_string()))?;

    if !license.is_active {
        return Err((StatusCode::BAD_REQUEST, "License is not active".to_string()));
    }

    if license.user_id.is_some() {
        return Err((StatusCode::CONFLICT, "License already activated".to_string()));
    }

    if license.expires_at < Utc::now() {
        return Err((StatusCode::BAD_REQUEST, "License has expired".to_string()));
    }

    // Activate license
    let updated_license = sqlx::query_as::<_, License>(
        r#"
        UPDATE licenses 
        SET user_id = $1, activated_at = $2
        WHERE license_key = $3
        RETURNING *
        "#
    )
    .bind(user_id)
    .bind(Utc::now())
    .bind(&license_key)
    .fetch_one(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(updated_license))
}

pub async fn revoke_license(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let license = sqlx::query_as::<_, License>(
        "UPDATE licenses SET is_active = false WHERE id = $1 RETURNING *"
    )
    .bind(id)
    .fetch_optional(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .ok_or((StatusCode::NOT_FOUND, "License not found".to_string()))?;

    Ok(Json(license))
}
