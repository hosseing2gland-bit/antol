use axum::{extract::State, http::StatusCode, Json};
use backend::handlers::{dashboard, licenses};
use backend::models::{ActivateLicenseRequest, CreateLicenseRequest, LicensePlan};
use chrono::Duration;
use sqlx::PgPool;
use uuid::Uuid;

#[sqlx::test(migrations = "./tests/migrations")]
async fn create_license_uses_duration_and_request(pool: PgPool) {
    let request = CreateLicenseRequest {
        plan: LicensePlan::Pro,
        max_profiles: 15,
        duration_days: 45,
    };

    let (status, Json(license)) = licenses::create_license(State(pool.clone()), Json(request))
        .await
        .expect("license should be created");

    assert_eq!(status, StatusCode::CREATED);
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
        "expiry should be within expected duration window"
    );
}

#[sqlx::test(migrations = "./tests/migrations")]
async fn activate_license_uses_activate_license_request(pool: PgPool) {
    let (_, Json(license)) = licenses::create_license(
        State(pool.clone()),
        Json(CreateLicenseRequest {
            plan: LicensePlan::Basic,
            max_profiles: 5,
            duration_days: 30,
        }),
    )
    .await
    .expect("license creation should succeed");

    let Json(activated) = licenses::activate_license(
        State(pool.clone()),
        Json(ActivateLicenseRequest {
            key: license.key.clone(),
            hardware_id: "HW-TEST-1234".to_string(),
        }),
    )
    .await
    .expect("activation should succeed");

    assert_eq!(activated.hardware_id.as_deref(), Some("HW-TEST-1234"));
    assert!(activated.activated_at.is_some());
}

#[sqlx::test(migrations = "./tests/migrations")]
async fn dashboard_stats_report_totals(pool: PgPool) {
    let user_id: Uuid = sqlx::query_scalar(
        "INSERT INTO users (email, password_hash, role) VALUES ($1, $2, $3) RETURNING id",
    )
    .bind("user@example.com")
    .bind("hashed-password")
    .bind("user")
    .fetch_one(&pool)
    .await
    .expect("user insert should work");

    let (_, Json(_license)) = licenses::create_license(
        State(pool.clone()),
        Json(CreateLicenseRequest {
            plan: LicensePlan::Trial,
            max_profiles: 2,
            duration_days: 7,
        }),
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

    let Json(stats) = dashboard::get_stats(State(pool.clone()))
        .await
        .expect("stats should be returned");

    assert_eq!(stats.total_users, 1);
    assert_eq!(stats.total_licenses, 1);
    assert_eq!(stats.total_profiles, 1);
    assert_eq!(stats.total_proxies, 1);
}
