pub mod email;
pub mod handlers;
pub mod models;
pub mod two_factor;

use axum::{
    routing::{get, post},
    Router,
};
use sqlx::PgPool;
use tower_http::cors::CorsLayer;

pub fn create_app(pool: PgPool) -> Router {
    Router::new()
        .route("/", get(|| async { "Anti-Detect Browser Backend API" }))
        .route("/api/auth/login", post(handlers::auth::login))
        .route("/api/auth/register", post(handlers::auth::register))
        .route(
            "/api/users",
            get(handlers::users::list_users).post(handlers::users::create_user),
        )
        .route(
            "/api/users/:id",
            get(handlers::users::get_user)
                .put(handlers::users::update_user)
                .delete(handlers::users::delete_user),
        )
        .route(
            "/api/licenses",
            get(handlers::licenses::list_licenses).post(handlers::licenses::create_license),
        )
        .route("/api/licenses/:id", get(handlers::licenses::get_license))
        .route(
            "/api/licenses/:id/revoke",
            post(handlers::licenses::revoke_license),
        )
        .route(
            "/api/licenses/activate",
            post(handlers::licenses::activate_license),
        )
        .route(
            "/api/profiles",
            get(handlers::profiles::list_profiles).post(handlers::profiles::create_profile),
        )
        .route(
            "/api/profiles/:id",
            get(handlers::profiles::get_profile)
                .put(handlers::profiles::update_profile)
                .delete(handlers::profiles::delete_profile),
        )
        .route(
            "/api/proxies",
            get(handlers::proxies::list_proxies).post(handlers::proxies::create_proxy),
        )
        .route(
            "/api/proxies/:id",
            get(handlers::proxies::get_proxy)
                .put(handlers::proxies::update_proxy)
                .delete(handlers::proxies::delete_proxy),
        )
        .route("/api/proxies/:id/test", post(handlers::proxies::test_proxy))
        .route("/api/dashboard/stats", get(handlers::dashboard::get_stats))
        .layer(CorsLayer::permissive())
        .with_state(pool)
}
