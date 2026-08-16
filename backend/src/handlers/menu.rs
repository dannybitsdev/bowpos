use axum::{
    extract::{Path, State, Json},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, put},
    Router,
};
use serde::Serialize;

use crate::{
    application::menu::MenuError,
    presentation::auth::{extractor::AuthUser, policy::{MenuReadAccess, MenuWriteAccess}},
    AppState,
};

#[derive(Serialize)]
pub struct MenuResponse {
    pub data: Vec<crate::domain::menu::Category>,
}

#[derive(serde::Deserialize)]
pub struct ProductPayload {
    pub category_id: uuid::Uuid,
    pub name: String,
    pub description: Option<String>,
    pub price: f64,
    pub stock: i32,
    pub image_url: Option<String>,
}

#[derive(serde::Serialize)]
pub struct CategoriesResponse {
    pub data: Vec<crate::domain::menu::Category>,
}

pub fn menu_router() -> Router<AppState> {
    Router::new()
        .route("/menu", get(list_menu))
        .route("/menu/categories", get(list_categories))
        .route("/menu/products", post(create_product))
        .route("/menu/products/:product_id", put(update_product))
}

pub async fn list_menu(
    State(state): State<AppState>,
    auth: AuthUser<MenuReadAccess>,
) -> Result<Json<MenuResponse>, MenuError> {
    let tenant_id = auth.user.tenant_id;
    let menu = state.menu_service.list_menu(tenant_id).await?;
    Ok(Json(MenuResponse { data: menu }))
}

pub async fn list_categories(
    State(state): State<AppState>,
    auth: AuthUser<MenuReadAccess>,
) -> Result<Json<CategoriesResponse>, MenuError> {
    Ok(Json(CategoriesResponse { data: state.menu_service.list_categories(auth.user.tenant_id).await? }))
}

pub async fn create_product(
    State(state): State<AppState>,
    auth: AuthUser<MenuWriteAccess>,
    Json(payload): Json<ProductPayload>,
) -> Result<(StatusCode, Json<crate::domain::menu::Product>), MenuError> {
    let product = state.menu_service.create_product(auth.user.tenant_id, payload.category_id, &payload.name, payload.description.as_deref(), payload.price, payload.stock, payload.image_url.as_deref()).await?;
    Ok((StatusCode::CREATED, Json(product)))
}

pub async fn update_product(
    State(state): State<AppState>,
    auth: AuthUser<MenuWriteAccess>,
    Path(product_id): Path<uuid::Uuid>,
    Json(payload): Json<ProductPayload>,
) -> Result<Json<crate::domain::menu::Product>, MenuError> {
    let product = state.menu_service.update_product(auth.user.tenant_id, product_id, payload.category_id, &payload.name, payload.description.as_deref(), payload.price, payload.stock, payload.image_url.as_deref()).await?;
    product.map(Json).ok_or_else(|| MenuError::Repository(anyhow::anyhow!("product not found")))
}

impl IntoResponse for MenuError {
    fn into_response(self) -> axum::response::Response {
        let body = Json(serde_json::json!({
            "message": self.to_string(),
        }));
        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}