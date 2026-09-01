use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::auth::Role;

#[derive(Debug, Deserialize)]
pub struct LoginCommand {
    pub email: String,
    pub password: String,
    pub tenant_id: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct RefreshTokenCommand {
    pub refresh_token: String,
}

#[derive(Debug, Deserialize)]
pub struct LogoutCommand {
    pub refresh_token: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RegisterSuperAdminCommand {
    pub tenant_name: String,
    pub tenant_slug: String,
    pub full_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateTenantAdminCommand {
    pub tenant_id: Uuid,
    pub full_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateTenantUserCommand {
    pub tenant_id: Uuid,
    pub full_name: String,
    pub email: String,
    pub password: String,
    pub role: Role,
}

#[derive(Debug, Serialize)]
pub struct AuthTokens {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: u64,
    pub refresh_expires_in: u64,
}

#[derive(Debug, Serialize)]
pub struct AuthUserView {
    pub user_id: Uuid,
    pub tenant_id: Uuid,
    pub tenant_name: String,
    pub name: String,
    pub email: String,
    pub role: Role,
    pub permissions: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct CurrentUserView {
    pub id: Uuid,
    pub nombre: String,
    pub email: String,
    pub rol: Role,
    pub cargo_label: String,
    pub tenant_id: Uuid,
    pub sede_actual_id: Option<Uuid>,
    pub permisos: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct LoginResult {
    pub tokens: AuthTokens,
    pub user: AuthUserView,
}
