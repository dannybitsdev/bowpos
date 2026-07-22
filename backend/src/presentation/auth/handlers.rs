use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    Json,
};
use serde::Serialize;

use crate::{
    application::auth::{
        commands::{
            CreateTenantAdminCommand, CreateTenantUserCommand, LoginCommand, RefreshTokenCommand,
            RegisterSuperAdminCommand,
        },
        errors::AppError,
    },
    presentation::auth::{
        extractor::AuthUser,
        policy::{SuperAdminOnly, TenantAdminOnly},
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
