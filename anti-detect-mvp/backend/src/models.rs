use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

// ============= User Models =============

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub role: String, // Use String instead of enum for database compatibility
    pub license_key: Option<String>,
    pub hardware_id: Option<String>,
    pub subscription_tier: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
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

impl<'r> sqlx::Decode<'r, sqlx::Postgres> for UserRole {
    fn decode(value: sqlx::postgres::PgValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
        let s: &str = <&str as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        match s.to_lowercase().as_str() {
            "admin" => Ok(UserRole::Admin),
            "user" => Ok(UserRole::User),
            _ => Err(format!("Invalid user role: {}", s).into()),
        }
    }
}

impl sqlx::Type<sqlx::Postgres> for UserRole {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<sqlx::Postgres>>::type_info()
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub role: String,
    pub created_at: NaiveDateTime,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            email: user.email,
            role: user.role,
            created_at: user.created_at,
        }
    }
}

// ============= License Models =============

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct License {
    pub id: Uuid,
    #[sqlx(rename = "license_key")]
    pub key: String,
    pub hardware_id: Option<String>,
    pub max_profiles: i32,
    pub features: Option<serde_json::Value>,
    pub expires_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub activated_at: Option<NaiveDateTime>,
    pub last_validation: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum LicensePlan {
    Trial,
    Basic,
    Pro,
    Enterprise,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateLicenseRequest {
    pub plan: LicensePlan,
    pub max_profiles: i32,
    pub duration_days: i32,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct ActivateLicenseRequest {
    pub hardware_id: String,
}

// ============= Profile Models =============

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Profile {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub name: String,
    pub fingerprint_config: serde_json::Value,
    pub proxy_id: Option<Uuid>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<String>,
    pub last_used: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateProfileRequest {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    pub proxy_id: Option<Uuid>,
    pub fingerprint_config: Option<serde_json::Value>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateProfileRequest {
    pub name: Option<String>,
    pub proxy_id: Option<Uuid>,
    pub fingerprint_config: Option<serde_json::Value>,
    pub notes: Option<String>,
}

// ============= Proxy Models =============

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Proxy {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub name: Option<String>,
    pub protocol: String,
    pub host: String,
    pub port: i32,
    pub username: Option<String>,
    #[serde(skip_serializing)]
    pub password: Option<String>,
    pub country: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ProxyType {
    Http,
    Https,
    Socks5,
}

impl std::fmt::Display for ProxyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProxyType::Http => write!(f, "http"),
            ProxyType::Https => write!(f, "https"),
            ProxyType::Socks5 => write!(f, "socks5"),
        }
    }
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

#[derive(Debug, Serialize, Deserialize)]
pub struct DashboardStats {
    pub total_users: i64,
    pub total_licenses: i64,
    pub total_profiles: i64,
    pub total_proxies: i64,
}
