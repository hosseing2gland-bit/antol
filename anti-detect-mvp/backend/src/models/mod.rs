use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use validator::Validate;

// User Models
#[derive(Debug, Serialize, Deserialize, FromRow, Clone, PartialEq)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub role: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 6))]
    pub password: String,
    pub role: String,
    pub is_active: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub email: Option<String>,
    pub role: Option<String>,
    pub is_active: Option<bool>,
}

// License Models
#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct License {
    pub id: Uuid,
    pub license_key: String,
    pub plan: String,
    pub max_profiles: i32,
    pub user_id: Option<Uuid>,
    pub is_active: bool,
    pub expires_at: DateTime<Utc>,
    pub activated_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LicensePlan {
    Trial,
    Basic,
    Pro,
    Enterprise,
}

impl ToString for LicensePlan {
    fn to_string(&self) -> String {
        match self {
            LicensePlan::Trial => "trial".to_string(),
            LicensePlan::Basic => "basic".to_string(),
            LicensePlan::Pro => "pro".to_string(),
            LicensePlan::Enterprise => "enterprise".to_string(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateLicenseRequest {
    pub plan: LicensePlan,
    pub max_profiles: i32,
}

// Profile Models
#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Profile {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub user_agent: String,
    pub screen_resolution: String,
    pub timezone: String,
    pub language: String,
    pub webgl_vendor: Option<String>,
    pub webgl_renderer: Option<String>,
    pub canvas_noise: bool,
    pub audio_noise: bool,
    pub fonts: Option<Vec<String>>,
    pub proxy_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateProfileRequest {
    pub user_id: Uuid,
    #[validate(length(min = 1))]
    pub name: String,
    pub user_agent: String,
    pub screen_resolution: String,
    pub timezone: String,
    pub language: String,
    pub webgl_vendor: Option<String>,
    pub webgl_renderer: Option<String>,
    pub canvas_noise: Option<bool>,
    pub audio_noise: Option<bool>,
    pub fonts: Option<Vec<String>>,
    pub proxy_id: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProfileRequest {
    pub name: Option<String>,
    pub user_agent: Option<String>,
    pub screen_resolution: Option<String>,
    pub timezone: Option<String>,
    pub language: Option<String>,
    pub proxy_id: Option<Uuid>,
}

// Proxy Models
#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Proxy {
    pub id: Uuid,
    pub user_id: Uuid,
    pub proxy_type: String,
    pub host: String,
    pub port: i32,
    pub username: Option<String>,
    pub password: Option<String>,
    pub country: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateProxyRequest {
    pub user_id: Uuid,
    pub proxy_type: String,
    #[validate(length(min = 1))]
    pub host: String,
    #[validate(range(min = 1, max = 65535))]
    pub port: i32,
    pub username: Option<String>,
    pub password: Option<String>,
    pub country: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProxyRequest {
    pub proxy_type: Option<String>,
    pub host: Option<String>,
    pub port: Option<i32>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub country: Option<String>,
}
