use axum::{body::Body, http::Request, http::StatusCode};
use backend::create_app;
use backend::models::{
    ActivateLicenseRequest, CreateLicenseRequest, DashboardStats, License, LicensePlan,
};
use chrono::Duration;
use http_body_util::BodyExt;
use serde_json::json;
use sqlx::PgPool;
use tower::util::ServiceExt;
use uuid::Uuid;

#[sqlx::test(migrations = "./tests/migrations")]
async fn create_license_uses_duration_and_request(pool: PgPool) {
    let app = create_app(pool.clone());

    let payload = json!(CreateLicenseRequest {
        plan: LicensePlan::Pro,
        max_profiles: 15,
        duration_days: 45,
    });

    let response = app
        .clone()
        .oneshot(
            Request::post("/api/licenses")
                .header("content-type", "application/json")
                .body(Body::from(payload.to_string()))
                .expect("request should build"),
        )
        .await
        .expect("response should be returned");

    assert_eq!(response.status(), StatusCode::CREATED);

    let body = response
        .into_body()
        .collect()
        .await
        .expect("body should read")
        .to_bytes();
    let license: License = serde_json::from_slice(&body).expect("license should deserialize");

    assert_eq!(license.max_profiles, 15);

    let plan = license
        .features
        .as_ref()
        .and_then(|value| value.get("plan"))
        .and_then(|value| value.as_str());
    assert_eq!(plan, Some("pro"));

    let expires_at = license.expires_at.expect("license should have expiry");
    let min_expected = chrono::Local::now().naive_local() + Duration::days(44);
    let max_expected = chrono::Local::now().naive_local() + Duration::days(46);
    assert!(
        expires_at >= min_expected && expires_at <= max_expected,
        "expiry should be within expected duration window",
    );
}

#[sqlx::test(migrations = "./tests/migrations")]
async fn activate_license_uses_activate_license_request(pool: PgPool) {
    let app = create_app(pool.clone());

    let creation_payload = json!(CreateLicenseRequest {
        plan: LicensePlan::Basic,
        max_profiles: 5,
        duration_days: 30,
    });

    let creation_response = app
        .clone()
        .oneshot(
            Request::post("/api/licenses")
                .header("content-type", "application/json")
                .body(Body::from(creation_payload.to_string()))
                .expect("request should build"),
        )
        .await
        .expect("creation should return response");

    let license_body = creation_response
        .into_body()
        .collect()
        .await
        .expect("license body should read")
        .to_bytes();
    let license: License =
        serde_json::from_slice(&license_body).expect("license should deserialize");

    let activation_payload = json!(ActivateLicenseRequest {
        hardware_id: "HW-TEST-1234".to_string(),
    });

    let activation_response = app
        .clone()
        .oneshot(
            Request::post(format!("/api/licenses/activate/{}", license.key))
                .header("content-type", "application/json")
                .body(Body::from(activation_payload.to_string()))
                .expect("activation request should build"),
        )
        .await
        .expect("activation should return response");

    assert_eq!(activation_response.status(), StatusCode::OK);

    let activation_body = activation_response
        .into_body()
        .collect()
        .await
        .expect("activation body should read")
        .to_bytes();
    let activated: License =
        serde_json::from_slice(&activation_body).expect("activated license should deserialize");

    assert_eq!(activated.hardware_id.as_deref(), Some("HW-TEST-1234"));
    assert!(activated.activated_at.is_some());
}

#[sqlx::test(migrations = "./tests/migrations")]
async fn dashboard_stats_report_totals(pool: PgPool) {
    let app = create_app(pool.clone());

    let user_id: Uuid = sqlx::query_scalar(
        "INSERT INTO users (email, password_hash, role) VALUES ($1, $2, $3) RETURNING id",
    )
    .bind("user@example.com")
    .bind("hashed-password")
    .bind("user")
    .fetch_one(&pool)
    .await
    .expect("user insert should work");

    let payload = json!(CreateLicenseRequest {
        plan: LicensePlan::Trial,
        max_profiles: 2,
        duration_days: 7,
    });

    app.clone()
        .oneshot(
            Request::post("/api/licenses")
                .header("content-type", "application/json")
                .body(Body::from(payload.to_string()))
                .expect("request should build"),
        )
        .await
        .expect("license creation should succeed");

    sqlx::query(
        "INSERT INTO proxies (user_id, name, protocol, host, port) VALUES ($1, $2, $3, $4, $5)",
    )
    .bind(user_id)
    .bind("Proxy A")
    .bind("http")
    .bind("127.0.0.1")
    .bind(8080_i32)
    .execute(&pool)
    .await
    .expect("proxy insert should work");

    sqlx::query("INSERT INTO profiles (user_id, name, fingerprint_config) VALUES ($1, $2, $3)")
        .bind(user_id)
        .bind("Profile A")
        .bind(serde_json::json!({}))
        .execute(&pool)
        .await
        .expect("profile insert should work");

    let response = app
        .clone()
        .oneshot(
            Request::get("/api/dashboard/stats")
                .body(Body::empty())
                .expect("request should build"),
        )
        .await
        .expect("stats response should be returned");

    assert_eq!(response.status(), StatusCode::OK);

    let body = response
        .into_body()
        .collect()
        .await
        .expect("stats body should read")
        .to_bytes();
    let stats: DashboardStats = serde_json::from_slice(&body).expect("stats should deserialize");

    assert_eq!(stats.total_users, 1);
    assert_eq!(stats.total_licenses, 1);
    assert_eq!(stats.total_profiles, 1);
    assert_eq!(stats.total_proxies, 1);
}
