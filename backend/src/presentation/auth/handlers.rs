use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    Json,
};
use serde::Serialize;

use crate::{
    application::auth::{
        commands::{
            CreateTenantAdminCommand, CreateTenantUserCommand, LoginCommand, LogoutCommand, RefreshTokenCommand,
            RegisterSuperAdminCommand,
        },
        errors::AppError,
    },
    presentation::auth::{
        extractor::AuthUser,
        policy::{AuthenticatedAccess, SuperAdminOnly, TenantAdminOnly},
    },
    AppState,
};

#[derive(Serialize)]
pub struct ApiError {
    pub message: String,
}

pub async fn login(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(command): Json<LoginCommand>,
) -> Result<Json<crate::application::auth::commands::LoginResult>, (StatusCode, Json<ApiError>)> {
    let rate_key = headers
        .get("x-forwarded-for")
        .and_then(|value| value.to_str().ok())
        .map(str::to_string)
        .unwrap_or_else(|| command.email.to_lowercase());

    if !state.login_rate_limiter.allow(&rate_key) {
        return Err(map_error(AppError::TooManyRequests));
    }

    let result = state.auth_use_cases.login(command).await.map_err(map_error)?;
    Ok(Json(result))
}

pub async fn refresh(
    State(state): State<AppState>,
    Json(command): Json<RefreshTokenCommand>,
) -> Result<Json<crate::application::auth::commands::LoginResult>, (StatusCode, Json<ApiError>)> {
    let result = state
        .auth_use_cases
        .refresh_session(command)
        .await
        .map_err(map_error)?;

    Ok(Json(result))
}

pub async fn logout(
    State(state): State<AppState>,
    auth: AuthUser<AuthenticatedAccess>,
    Json(command): Json<LogoutCommand>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ApiError>)> {
    sqlx::query(
        "INSERT INTO auth_access_token_revocations (token_id, user_id, expires_at) VALUES ($1, $2, to_timestamp($3)) ON CONFLICT (token_id) DO NOTHING",
    )
    .bind(auth.token_id)
    .bind(auth.user.id)
    .bind(auth.expires_at)
    .execute(&state.pool)
    .await
    .map_err(|_| map_error(AppError::Internal))?;

    state.auth_use_cases.logout(&auth.user, command).await.map_err(map_error)?;
    Ok(Json(serde_json::json!({ "message": "Sesión cerrada" })))
}

pub async fn current_user(
    State(state): State<AppState>,
    auth: AuthUser<AuthenticatedAccess>,
) -> Json<crate::application::auth::commands::CurrentUserView> {
    Json(state.auth_use_cases.current_user(&auth.user, auth.branch))
}

pub async fn register_super_admin(
    State(state): State<AppState>,
    Json(command): Json<RegisterSuperAdminCommand>,
) -> Result<Json<crate::application::auth::commands::AuthUserView>, (StatusCode, Json<ApiError>)> {
    let result = state
        .auth_use_cases
        .register_super_admin(command)
        .await
        .map_err(map_error)?;
    Ok(Json(result))
}

pub async fn create_tenant_admin(
    State(state): State<AppState>,
    auth: AuthUser<SuperAdminOnly>,
    Json(command): Json<CreateTenantAdminCommand>,
) -> Result<(StatusCode, Json<crate::application::auth::commands::AuthUserView>), (StatusCode, Json<ApiError>)> {
    let result = state
        .auth_use_cases
        .create_tenant_admin(&auth.into_inner(), command)
        .await
        .map_err(map_error)?;
    Ok((StatusCode::CREATED, Json(result)))
}

pub async fn create_tenant_user(
    State(state): State<AppState>,
    auth: AuthUser<TenantAdminOnly>,
    Json(command): Json<CreateTenantUserCommand>,
) -> Result<(StatusCode, Json<crate::application::auth::commands::AuthUserView>), (StatusCode, Json<ApiError>)> {
    let result = state
        .auth_use_cases
        .create_tenant_user(&auth.into_inner(), command)
        .await
        .map_err(map_error)?;
    Ok((StatusCode::CREATED, Json(result)))
}

fn map_error(error: AppError) -> (StatusCode, Json<ApiError>) {
    let status = error.status_code();
    let message = match error {
        AppError::InvalidCredentials | AppError::Unauthorized => "Credenciales inválidas",
        AppError::Forbidden => "Acceso denegado",
        AppError::NotFound => "Recurso no encontrado",
        AppError::AlreadyExists => "El recurso ya existe",
        AppError::Validation(_) => "Datos inválidos",
        AppError::TooManyRequests => "Demasiados intentos. Intenta más tarde",
        AppError::Internal => "Error interno",
    };

    (
        status,
        Json(ApiError {
            message: message.to_string(),
        }),
    )
}

#[cfg(test)]
mod tests {
    use axum::{body::Body, http::Request};
    use serde_json::{json, Value};
    use std::sync::Arc;
    use tower::ServiceExt;

    use crate::{
        application::{auth::use_cases::AuthUseCases, menu::MenuService, orders::OrderService},
        infrastructure::{
            repositories::{sqlx_orders_repository::SqlxOrderRepository, sqlx_user_repository::SqlxUserRepository},
            seeder::seed_initial_super_admin,
            services::{jwt_service::JwtService, login_rate_limiter::LoginRateLimiter, password_hasher::PasswordHasher, refresh_token_service::RefreshTokenService},
        },
        presentation::auth::router::auth_router,
        AppState,
    };

    /// Requiere una instancia real de PostgreSQL (docker compose up db) con las
    /// migraciones aplicadas. Verifica que, tras hacer logout, el access token
    /// queda revocado y no puede reutilizarse (HTTP 401 en /me).
    #[ignore = "requires a running postgres instance (docker compose up db)"]
    #[tokio::test]
    async fn logout_revokes_access_token_and_blocks_reuse() {
        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgresql://appuser:changeme@localhost:5432/appdb".to_string());
        let pool = sqlx::PgPool::connect(&database_url).await.expect("database connection");
        seed_initial_super_admin(&pool).await.expect("seed super admin");

        let repository = Arc::new(SqlxUserRepository::new(pool.clone()));
        let jwt_service = JwtService::new("tests-secret", 60 * 60);
        let auth_use_cases = Arc::new(AuthUseCases::new(
            repository,
            PasswordHasher::default(),
            jwt_service.clone(),
            RefreshTokenService::new("tests-refresh-secret".to_string(), 60 * 60 * 24),
            5,
            15,
        ));
        let state = AppState {
            pool: pool.clone(),
            auth_use_cases,
            jwt_service,
            login_rate_limiter: LoginRateLimiter::new(1000, 60),
            menu_service: Arc::new(MenuService::new(Arc::new(SqlxUserRepository::new(pool.clone())))),
            order_service: Arc::new(OrderService::new(Arc::new(SqlxOrderRepository::new(pool.clone())))),
        };

        let app = auth_router().with_state(state);

        let login_response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/login")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        json!({
                            "email": "superadmin@bitstitecnologia.com",
                            "password": "BitsTITecnologia!2026",
                        })
                        .to_string(),
                    ))
                    .expect("request"),
            )
            .await
            .expect("response");
        assert_eq!(login_response.status(), axum::http::StatusCode::OK);

        let body = axum::body::to_bytes(login_response.into_body(), usize::MAX).await.expect("body");
        let login_json: Value = serde_json::from_slice(&body).expect("json");
        let access_token = login_json["tokens"]["access_token"].as_str().expect("access token").to_string();

        // El token todavía es válido: /me responde 200.
        let me_before = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/me")
                    .header("Authorization", format!("Bearer {access_token}"))
                    .body(Body::empty())
                    .expect("request"),
            )
            .await
            .expect("response");
        assert_eq!(me_before.status(), axum::http::StatusCode::OK);

        let logout_response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/logout")
                    .header("content-type", "application/json")
                    .header("Authorization", format!("Bearer {access_token}"))
                    .body(Body::from(json!({ "refresh_token": null }).to_string()))
                    .expect("request"),
            )
            .await
            .expect("response");
        assert_eq!(logout_response.status(), axum::http::StatusCode::OK);

        // El mismo access token ya no puede reutilizarse tras el logout.
        let me_after = app
            .oneshot(
                Request::builder()
                    .uri("/me")
                    .header("Authorization", format!("Bearer {access_token}"))
                    .body(Body::empty())
                    .expect("request"),
            )
            .await
            .expect("response");
        assert_eq!(me_after.status(), axum::http::StatusCode::UNAUTHORIZED);
    }
}
