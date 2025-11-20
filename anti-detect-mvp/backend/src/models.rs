use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use validator::Validate;

// ============= User Models =============

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub role: UserRole,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone, PartialEq)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
pub enum UserRole {
    Admin,
    User,
}

impl std::fmt::Display for UserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserRole::Admin => write!(f, "admin"),
            UserRole::User => write!(f, "user"),
        }
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
    pub role: Option<UserRole>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateUserRequest {
    #[validate(email)]
    pub email: Option<String>,
    pub is_active: Option<bool>,
    pub role: Option<UserRole>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: UserResponse,
}

#[derive(Debug, Serialize, Clone)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub role: UserRole,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            email: user.email,
            role: user.role,
            is_active: user.is_active,
            created_at: user.created_at,
        }
    }
}

// ============= License Models =============

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct License {
    pub id: Uuid,
    pub key: String,
    pub user_id: Option<Uuid>,
    pub plan: LicensePlan,
    pub max_profiles: i32,
    pub expires_at: DateTime<Utc>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub activated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone)]
#[sqlx(type_name = "license_plan", rename_all = "lowercase")]
pub enum LicensePlan {
    Trial,
    Basic,
    Pro,
    Enterprise,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateLicenseRequest {
    pub plan: LicensePlan,
    pub max_profiles: i32,
    pub duration_days: i32,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ActivateLicenseRequest {
    pub key: String,
    pub hardware_id: String,
}

// ============= Profile Models =============

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Profile {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub fingerprint: serde_json::Value,
    pub proxy_id: Option<Uuid>,
    pub user_agent: String,
    pub timezone: String,
    pub locale: String,
    pub webgl_vendor: String,
    pub webgl_renderer: String,
    pub canvas_noise: bool,
    pub webgl_noise: bool,
    pub audio_noise: bool,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub last_used_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateProfileRequest {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    pub proxy_id: Option<Uuid>,
    pub user_agent: Option<String>,
    pub timezone: Option<String>,
    pub locale: Option<String>,
    pub canvas_noise: Option<bool>,
    pub webgl_noise: Option<bool>,
    pub audio_noise: Option<bool>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateProfileRequest {
    pub name: Option<String>,
    pub proxy_id: Option<Uuid>,
    pub is_active: Option<bool>,
    pub canvas_noise: Option<bool>,
    pub webgl_noise: Option<bool>,
    pub audio_noise: Option<bool>,
}

// ============= Proxy Models =============

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Proxy {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub proxy_type: ProxyType,
    pub host: String,
    pub port: i32,
    pub username: Option<String>,
    #[serde(skip_serializing)]
    pub password: Option<String>,
    pub is_active: bool,
    pub last_checked_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone)]
#[sqlx(type_name = "proxy_type", rename_all = "lowercase")]
pub enum ProxyType {
    Http,
    Https,
    Socks5,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateProxyRequest {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    pub proxy_type: ProxyType,
    #[validate(length(min = 1))]
    pub host: String,
    #[validate(range(min = 1, max = 65535))]
    pub port: i32,
    pub username: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProxyRequest {
    pub name: Option<String>,
    pub is_active: Option<bool>,
    pub username: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ProxyTestResponse {
    pub success: bool,
    pub ip: Option<String>,
    pub country: Option<String>,
    pub message: Option<String>,
}

// ============= Stats Models =============

#[derive(Debug, Serialize)]
pub struct DashboardStats {
    pub total_users: i64,
    pub active_users: i64,
    pub total_licenses: i64,
    pub active_licenses: i64,
    pub total_profiles: i64,
    pub active_profiles: i64,
    pub total_proxies: i64,
    pub active_proxies: i64,
}
