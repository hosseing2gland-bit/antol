use crate::models::{ActivateLicenseRequest, CreateLicenseRequest, License, LicensePlan};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use chrono::Duration;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn list_licenses(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<License>>, (StatusCode, String)> {
    let licenses = sqlx::query_as::<_, License>("SELECT * FROM licenses ORDER BY created_at DESC")
        .fetch_all(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(licenses))
}

pub async fn get_license(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<License>, (StatusCode, String)> {
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
) -> Result<(StatusCode, Json<License>), (StatusCode, String)> {
    // Generate license key
    let license_key = format!(
        "{}-{}-{}-{}",
        Uuid::new_v4().to_string()[..8].to_uppercase(),
        Uuid::new_v4().to_string()[..8].to_uppercase(),
        Uuid::new_v4().to_string()[..8].to_uppercase(),
        Uuid::new_v4().to_string()[..8].to_uppercase()
    );

    if req.duration_days <= 0 {
        return Err((
            StatusCode::BAD_REQUEST,
            "Duration must be greater than zero".to_string(),
        ));
    }

    // Calculate expiry based on provided duration
    let expires_at = chrono::Local::now().naive_local() + Duration::days(req.duration_days as i64);

    // Create features JSON based on plan
    let features = serde_json::json!({
        "plan": format!("{:?}", req.plan).to_lowercase(),
        "max_profiles": req.max_profiles
    });

    let license = sqlx::query_as::<_, License>(
        r#"
        INSERT INTO licenses (license_key, max_profiles, features, expires_at)
        VALUES ($1, $2, $3, $4)
        RETURNING *
        "#,
    )
    .bind(&license_key)
    .bind(req.max_profiles)
    .bind(&features)
    .bind(expires_at)
    .fetch_one(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((StatusCode::CREATED, Json(license)))
}

pub async fn activate_license(
    State(pool): State<PgPool>,
    Path(license_key): Path<String>,
    Json(req): Json<ActivateLicenseRequest>,
) -> Result<Json<License>, (StatusCode, String)> {
    // Check if license exists and is valid
    let license = sqlx::query_as::<_, License>("SELECT * FROM licenses WHERE license_key = $1")
        .bind(&license_key)
        .fetch_optional(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "License not found".to_string()))?;

    if license.hardware_id.is_some() {
        return Err((
            StatusCode::CONFLICT,
            "License already activated".to_string(),
        ));
    }

    if let Some(expires_at) = license.expires_at {
        if expires_at < chrono::Local::now().naive_local() {
            return Err((StatusCode::BAD_REQUEST, "License has expired".to_string()));
        }
    }

    // Activate license
    let hardware_id = req.hardware_id;
    let updated_license = sqlx::query_as::<_, License>(
        r#"
        UPDATE licenses
        SET hardware_id = $1, activated_at = $2
        WHERE license_key = $3
        RETURNING *
        "#,
    )
    .bind(&hardware_id)
    .bind(chrono::Local::now().naive_local())
    .bind(&license_key)
    .fetch_one(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(updated_license))
}

pub async fn revoke_license(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<License>, (StatusCode, String)> {
    let license = sqlx::query_as::<_, License>(
        "UPDATE licenses SET user_id = NULL, hardware_id = NULL WHERE id = $1 RETURNING *",
    )
    .bind(id)
    .fetch_optional(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .ok_or((StatusCode::NOT_FOUND, "License not found".to_string()))?;

    Ok(Json(license))
}
