mod application;
mod domain;
mod handlers;
mod infrastructure;
mod middleware;
mod presentation;

use axum::{
    http::header,
    http::Method,
    middleware as axum_middleware,
    routing::get,
    Json, Router,
};
use handlers::{
    dashboard::{get_dashboard_metrics, get_ui_config},
    sedes::sedes_router,
};
use infrastructure::{repositories::sqlx_user_repository::SqlxUserRepository, seeder::seed_initial_super_admin, services::{jwt_service::JwtService, login_rate_limiter::LoginRateLimiter, password_hasher::PasswordHasher, refresh_token_service::RefreshTokenService}};
use middleware::tenant::tenant_middleware;
use presentation::auth::router::auth_router;
use serde::Serialize;
use sqlx::PgPool;
use std::{env, sync::Arc};
use tower_http::cors::{Any, CorsLayer};

use crate::application::auth::use_cases::AuthUseCases;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub auth_use_cases: Arc<AuthUseCases>,
    pub jwt_service: JwtService,
    pub login_rate_limiter: LoginRateLimiter,
}

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
    service: &'static str,
}

#[tokio::main]
async fn main() {
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| "postgresql://appuser:changeme@db:5432/appdb".to_string());
    let pool = PgPool::connect(&database_url).await.expect("database connection");
    let jwt_secret = env::var("JWT_SECRET").unwrap_or_else(|_| "change-this-jwt-secret".to_string());
    let jwt_ttl_seconds = env::var("JWT_TTL_SECONDS")
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
        .unwrap_or(60 * 60 * 4);
    let refresh_secret = env::var("REFRESH_TOKEN_SECRET").unwrap_or_else(|_| jwt_secret.clone());
    let refresh_ttl_seconds = env::var("REFRESH_TOKEN_TTL_SECONDS")
        .ok()
        .and_then(|value| value.parse::<i64>().ok())
        .unwrap_or(60 * 60 * 24 * 7);
    let login_rate_limit_max = env::var("LOGIN_RATE_LIMIT_MAX")
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(20);
    let login_rate_limit_window_seconds = env::var("LOGIN_RATE_LIMIT_WINDOW_SECONDS")
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
        .unwrap_or(60);
    let login_lock_max_attempts = env::var("LOGIN_LOCK_MAX_ATTEMPTS")
        .ok()
        .and_then(|value| value.parse::<i32>().ok())
        .unwrap_or(5);
    let login_lock_minutes = env::var("LOGIN_LOCK_MINUTES")
        .ok()
        .and_then(|value| value.parse::<i32>().ok())
        .unwrap_or(15);

    let migrations_dir = env::var("MIGRATIONS_DIR")
        .ok()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| {
            if std::path::Path::new("migrations").exists() {
                "migrations".to_string()
            } else {
                "/app/migrations".to_string()
            }
        });

    let migrations = std::fs::read_dir(&migrations_dir).expect("migrations dir");
    let mut paths: Vec<_> = migrations
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| {
            let path = entry.path();
            (path.extension().and_then(|ext| ext.to_str()) == Some("sql")).then_some(path)
        })
        .collect();
    paths.sort();

    for path in paths {
        let sql = std::fs::read_to_string(&path).expect("migration sql");
        for statement in sql.split(";") {
            let stmt = statement.trim();
            if !stmt.is_empty() {
                sqlx::query(stmt).execute(&pool).await.expect("apply migration");
            }
        }
    }

    seed_initial_super_admin(&pool).await.expect("seed super admin");

    let repository = Arc::new(SqlxUserRepository::new(pool.clone()));
    let jwt_service = JwtService::new(&jwt_secret, jwt_ttl_seconds);
    let auth_use_cases = Arc::new(AuthUseCases::new(
        repository,
        PasswordHasher::default(),
        jwt_service.clone(),
        RefreshTokenService::new(refresh_secret, refresh_ttl_seconds),
        login_lock_max_attempts,
        login_lock_minutes,
    ));

    let app_state = AppState {
        pool: pool.clone(),
        auth_use_cases,
        jwt_service,
        login_rate_limiter: LoginRateLimiter::new(
            login_rate_limit_max,
            login_rate_limit_window_seconds,
        ),
    };

    let app = Router::new()
        .route("/health", get(health))
        .route("/api/dashboard", get(get_dashboard_metrics))
        .route("/api/config/ui", get(get_ui_config))
        .nest("/api", sedes_router())
        .nest("/api/auth", auth_router())
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
                .allow_headers([
                    header::CONTENT_TYPE,
                    header::AUTHORIZATION,
                    header::ACCEPT,
                    header::HeaderName::from_static("x-tenant-id"),
                    header::HeaderName::from_static("x-tenant-slug"),
                ]),
        )
        .layer(axum_middleware::from_fn(tenant_middleware))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .expect("bind tcp listener");

    println!("Backend listening on http://0.0.0.0:8080");
    axum::serve(listener, app).await.expect("axum server");
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok", service: "backend" })
}
