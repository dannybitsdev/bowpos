use axum::{routing::{get, post}, Router};

use crate::AppState;

use super::handlers::{create_tenant_admin, create_tenant_user, current_user, login, logout, refresh, register_super_admin};

pub fn auth_router() -> Router<AppState> {
    Router::new()
        .route("/login", post(login))
        .route("/refresh", post(refresh))
        .route("/logout", post(logout))
        .route("/me", get(current_user))
        .route("/register-super-admin", post(register_super_admin))
        .route("/tenant-admins", post(create_tenant_admin))
        .route("/users", post(create_tenant_user))
}
