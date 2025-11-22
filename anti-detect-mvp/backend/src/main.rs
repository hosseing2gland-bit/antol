use axum::{
    routing::{get, post},
    Router,
};
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

mod email;
mod handlers;
mod models;
mod state;
mod two_factor;
use crate::{email::EmailService, state::AppState};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://admin:admin123@localhost/antidetect".to_string());

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to Postgres");

    // Skip migrations if tables already exist (for demo)
    // sqlx::migrate!("./migrations")
    //     .run(&pool)
    //     .await
    //     .expect("Failed to run migrations");

    let email_service = match EmailService::new() {
        Ok(service) if service.mailer.is_some() => service,
        Ok(service) => {
            let reason = service
                .disabled_reason
                .clone()
                .unwrap_or_else(|| "Email configuration missing".to_string());
            println!("⚠️ Email service disabled: {}", reason);
            service
        }
        Err(err) => {
            println!("⚠️ Email service unavailable: {err}");
            EmailService::disabled("noreply@antidetect.local".to_string(), err.to_string())
        }
    };

    let state = AppState {
        pool: pool.clone(),
        email_service: std::sync::Arc::new(email_service),
    };

    let app = Router::new()
        .route("/", get(|| async { "Anti-Detect Browser Backend API" }))
        .route("/api/auth/login", post(handlers::auth::login))
        .route("/api/auth/register", post(handlers::auth::register))
        .route(
            "/api/auth/password-reset",
            post(handlers::auth::request_password_reset),
        )
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
            "/api/licenses/activate/:license_key",
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
        .layer(CorsLayer::permissive())
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("✅ Server running on http://{}", addr);
    println!("📚 API Documentation: http://{}/api", addr);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind to address");

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
