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
        LicensePlan::Trial => chrono::Local::now().naive_local() + Duration::days(7),
        LicensePlan::Basic => chrono::Local::now().naive_local() + Duration::days(30),
        LicensePlan::Pro => chrono::Local::now().naive_local() + Duration::days(90),
        LicensePlan::Enterprise => chrono::Local::now().naive_local() + Duration::days(365),
    };

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
        "#
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

    if license.hardware_id.is_some() {
        return Err((StatusCode::CONFLICT, "License already activated".to_string()));
    }

    if let Some(expires_at) = license.expires_at {
        if expires_at < chrono::Local::now().naive_local() {
            return Err((StatusCode::BAD_REQUEST, "License has expired".to_string()));
        }
    }

    // Activate license
    let hardware_id = user_id.to_string(); // Convert user_id to hardware_id
    let updated_license = sqlx::query_as::<_, License>(
        r#"
        UPDATE licenses 
        SET hardware_id = $1, activated_at = $2
        WHERE license_key = $3
        RETURNING *
        "#
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
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let license = sqlx::query_as::<_, License>(
        "UPDATE licenses SET user_id = NULL, hardware_id = NULL WHERE id = $1 RETURNING *"
    )
    .bind(id)
    .fetch_optional(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .ok_or((StatusCode::NOT_FOUND, "License not found".to_string()))?;

    Ok(Json(license))
}
