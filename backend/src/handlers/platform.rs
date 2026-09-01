use axum::{extract::State, http::StatusCode, routing::get, Json, Router};
use serde::Serialize;
use sqlx::Row;
use uuid::Uuid;

use crate::presentation::auth::{extractor::AuthUser, policy::SuperAdminOnly};
use crate::AppState;

#[derive(Serialize)]
pub struct TenantSummary {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
}

pub fn platform_router() -> Router<AppState> {
    Router::new().route("/platform/tenants", get(list_tenants))
}

/// Solo SUPER_ADMIN puede listar todos los tenants: es la base para el "modo plataforma"
/// (cambiar de tenant vía header `X-Tenant-Override`).
async fn list_tenants(
    State(state): State<AppState>,
    _auth: AuthUser<SuperAdminOnly>,
) -> Result<Json<Vec<TenantSummary>>, StatusCode> {
    let rows = sqlx::query("SELECT id, name, slug FROM tenants ORDER BY name")
        .fetch_all(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    rows.into_iter()
        .map(|row| {
            Ok(TenantSummary {
                id: row.try_get("id").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
                name: row.try_get("name").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
                slug: row.try_get("slug").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
            })
        })
        .collect::<Result<Vec<_>, StatusCode>>()
        .map(Json)
}
