use axum::{
    extract::{Json, State},
    http::StatusCode,
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use uuid::Uuid;

use crate::AppState;
use crate::presentation::auth::{extractor::AuthUser, policy::MenuReadAccess};

#[derive(Deserialize)]
pub struct CreateLocationPayload {
    pub name: String,
    pub address: String,
    pub city: String,
}

#[derive(Serialize)]
pub struct LocationResponse {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub address: String,
    pub city: String,
}

pub fn locations_router() -> Router<AppState> {
    Router::new().route("/locations", get(list_locations).post(create_location))
}

pub async fn list_locations(
    State(state): State<AppState>,
    auth: AuthUser<MenuReadAccess>,
) -> Result<Json<Vec<LocationResponse>>, StatusCode> {
    let rows = sqlx::query("SELECT id, tenant_id, name, address, city FROM locations WHERE tenant_id = $1 ORDER BY name")
        .bind(auth.user.tenant_id)
        .fetch_all(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    rows.into_iter()
        .map(|row| {
            Ok(LocationResponse {
                id: row.try_get("id").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
                tenant_id: row.try_get("tenant_id").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
                name: row.try_get("name").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
                address: row.try_get("address").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
                city: row.try_get("city").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
            })
        })
        .collect::<Result<Vec<_>, StatusCode>>()
        .map(Json)
}

pub async fn create_location(
    State(state): State<AppState>,
    auth: AuthUser<MenuReadAccess>,
    Json(payload): Json<CreateLocationPayload>,
) -> Result<(StatusCode, Json<LocationResponse>), StatusCode> {
    let row = sqlx::query(
        "INSERT INTO locations (id, tenant_id, name, address, city, printer_config) VALUES ($1, $2, $3, $4, $5, '{}'::jsonb) RETURNING id, tenant_id, name, address, city",
    )
    .bind(Uuid::new_v4())
    .bind(auth.user.tenant_id)
    .bind(payload.name)
    .bind(payload.address)
    .bind(payload.city)
    .fetch_one(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((
        StatusCode::CREATED,
        Json(LocationResponse {
            id: row.try_get("id").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
            tenant_id: row.try_get("tenant_id").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
            name: row.try_get("name").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
            address: row.try_get("address").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
            city: row.try_get("city").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
        }),
    ))
}