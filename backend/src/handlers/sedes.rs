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

#[derive(Deserialize)]
pub struct CreateSedePayload {
    pub tenant_id: Uuid,
    pub nombre: String,
    pub direccion: String,
    pub ciudad: String,
}

#[derive(Serialize)]
pub struct SedeResponse {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub nombre: String,
    pub direccion: String,
    pub ciudad: String,
}

pub fn sedes_router() -> Router<AppState> {
    Router::new()
        .route("/sedes", get(list_sedes).post(create_sede))
}

pub async fn list_sedes(
    State(state): State<AppState>,
) -> Result<Json<Vec<SedeResponse>>, StatusCode> {
    let rows = sqlx::query("SELECT id, tenant_id, nombre, direccion, ciudad FROM sedes ORDER BY nombre")
        .fetch_all(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut sedes = Vec::new();
    for row in rows {
        let id: Uuid = row.try_get("id").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        let tenant_id: Uuid = row.try_get("tenant_id").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        let nombre: String = row.try_get("nombre").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        let direccion: String = row.try_get("direccion").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        let ciudad: String = row.try_get("ciudad").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        sedes.push(SedeResponse { id, tenant_id, nombre, direccion, ciudad });
    }

    Ok(Json(sedes))
}

pub async fn create_sede(
    State(state): State<AppState>,
    Json(payload): Json<CreateSedePayload>,
) -> Result<(StatusCode, Json<SedeResponse>), StatusCode> {
    let id = Uuid::new_v4();
    let row = sqlx::query("INSERT INTO sedes (id, tenant_id, nombre, direccion, ciudad, configuracion_impresora) VALUES ($1, $2, $3, $4, $5, '{}'::jsonb) RETURNING id, tenant_id, nombre, direccion, ciudad")
        .bind(id)
        .bind(payload.tenant_id)
        .bind(payload.nombre)
        .bind(payload.direccion)
        .bind(payload.ciudad)
        .fetch_one(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let sede = SedeResponse {
        id: row.try_get("id").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
        tenant_id: row.try_get("tenant_id").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
        nombre: row.try_get("nombre").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
        direccion: row.try_get("direccion").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
        ciudad: row.try_get("ciudad").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
    };

    Ok((StatusCode::CREATED, Json(sede)))
}
