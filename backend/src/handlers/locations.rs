use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use uuid::Uuid;

use crate::AppState;
use crate::presentation::auth::{extractor::AuthUser, policy::{BranchAssignmentAccess, LocationWriteAccess, MenuReadAccess}};

#[derive(Deserialize)]
pub struct CreateLocationPayload {
    pub name: String,
    pub address: String,
    pub city: String,
}

#[derive(Deserialize)]
pub struct AssignBranchPayload {
    pub location_id: Uuid,
    #[serde(default)]
    pub is_primary: bool,
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
    Router::new()
        .route("/locations", get(list_locations).post(create_location))
        .route("/users/:user_id/branches", post(assign_branch))
}

pub async fn list_locations(
    State(state): State<AppState>,
    auth: AuthUser<MenuReadAccess>,
) -> Result<Json<Vec<LocationResponse>>, StatusCode> {
    // Roles restringidos a sedes espec\u00edficas solo ven las suyas; roles admin ven todas las del tenant.
    let rows = sqlx::query("SELECT id, tenant_id, name, address, city FROM locations WHERE tenant_id = $1 AND ($2::uuid[] IS NULL OR id = ANY($2)) ORDER BY name")
        .bind(auth.user.tenant_id)
        .bind(if auth.user.branch_ids.is_empty() { None } else { Some(auth.user.branch_ids.clone()) })
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
    auth: AuthUser<LocationWriteAccess>,
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

pub async fn assign_branch(
    State(state): State<AppState>,
    auth: AuthUser<BranchAssignmentAccess>,
    Path(user_id): Path<Uuid>,
    Json(payload): Json<AssignBranchPayload>,
) -> Result<StatusCode, StatusCode> {
    // Ambos, el usuario objetivo y la sede, deben pertenecer al tenant del actor.
    let target_belongs_to_tenant = sqlx::query_scalar::<_, bool>("SELECT EXISTS (SELECT 1 FROM users WHERE id = $1 AND tenant_id = $2)")
        .bind(user_id).bind(auth.user.tenant_id).fetch_one(&state.pool).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    if !target_belongs_to_tenant { return Err(StatusCode::NOT_FOUND); }

    let location_belongs_to_tenant = sqlx::query_scalar::<_, bool>("SELECT EXISTS (SELECT 1 FROM locations WHERE id = $1 AND tenant_id = $2)")
        .bind(payload.location_id).bind(auth.user.tenant_id).fetch_one(&state.pool).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    if !location_belongs_to_tenant { return Err(StatusCode::NOT_FOUND); }

    sqlx::query(
        "INSERT INTO user_branch_access (tenant_id, user_id, location_id, is_primary) VALUES ($1, $2, $3, $4)
         ON CONFLICT (tenant_id, user_id, location_id) DO UPDATE SET is_primary = EXCLUDED.is_primary",
    )
    .bind(auth.user.tenant_id)
    .bind(user_id)
    .bind(payload.location_id)
    .bind(payload.is_primary)
    .execute(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if payload.is_primary {
        sqlx::query("UPDATE user_branch_access SET is_primary = FALSE WHERE tenant_id = $1 AND user_id = $2 AND location_id <> $3")
            .bind(auth.user.tenant_id).bind(user_id).bind(payload.location_id).execute(&state.pool).await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    Ok(StatusCode::NO_CONTENT)
}
