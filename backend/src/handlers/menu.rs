use axum::{
    extract::{Path, State, Json},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Router,
};
use serde::Serialize;

use crate::{
    application::menu::MenuError,
    presentation::auth::{extractor::AuthUser, policy::{BranchCatalogWriteAccess, MenuReadAccess, MenuWriteAccess}},
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

#[derive(serde::Deserialize)]
pub struct CategoryPayload {
    pub name: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub display_order: i32,
}

#[derive(serde::Deserialize)]
pub struct BranchOverridePayload {
    pub price: Option<f64>,
    pub stock: Option<i32>,
    #[serde(default = "default_true")]
    pub is_available: bool,
}
fn default_true() -> bool { true }

pub fn menu_router() -> Router<AppState> {
    Router::new()
        .route("/menu", get(list_menu))
        .route("/menu/products", get(list_products))
        .route("/menu/categories", get(list_categories))
        .route("/menu/categories", post(create_category))
        .route("/menu/categories/:category_id", put(update_category))
        .route("/menu/categories/:category_id", delete(deactivate_category))
        .route("/menu/products", post(create_product))
        .route("/menu/products/:product_id", put(update_product))
        .route("/menu/products/:product_id", delete(delete_product))
        .route("/menu/branch-overrides/:product_id", put(upsert_branch_override))
}

pub async fn list_menu(
    State(state): State<AppState>,
    auth: AuthUser<MenuReadAccess>,
) -> Result<Json<MenuResponse>, MenuError> {
    let menu = state.menu_service.list_menu(auth.user.tenant_id, auth.branch).await?;
    Ok(Json(MenuResponse { data: menu }))
}

pub async fn list_products(
    State(state): State<AppState>,
    auth: AuthUser<MenuReadAccess>,
) -> Result<Json<Vec<crate::domain::menu::Product>>, MenuError> {
    Ok(Json(state.menu_service.list_products(auth.user.tenant_id).await?))
}

pub async fn list_categories(
    State(state): State<AppState>,
    auth: AuthUser<MenuReadAccess>,
) -> Result<Json<CategoriesResponse>, MenuError> {
    Ok(Json(CategoriesResponse { data: state.menu_service.list_categories(auth.user.tenant_id).await? }))
}

pub async fn create_category(
    State(state): State<AppState>,
    auth: AuthUser<MenuWriteAccess>,
    Json(payload): Json<CategoryPayload>,
) -> Result<(StatusCode, Json<crate::domain::menu::Category>), MenuError> {
    let category = state.menu_service.create_category(auth.user.tenant_id, &payload.name, payload.description.as_deref(), payload.image_url.as_deref(), payload.display_order).await?;
    Ok((StatusCode::CREATED, Json(category)))
}

pub async fn update_category(
    State(state): State<AppState>,
    auth: AuthUser<MenuWriteAccess>,
    Path(category_id): Path<uuid::Uuid>,
    Json(payload): Json<CategoryPayload>,
) -> Result<Json<crate::domain::menu::Category>, MenuError> {
    let category = state.menu_service.update_category(auth.user.tenant_id, category_id, &payload.name, payload.description.as_deref(), payload.image_url.as_deref(), payload.display_order).await?;
    category.map(Json).ok_or_else(|| MenuError::Repository(anyhow::anyhow!("category not found")))
}

pub async fn deactivate_category(
    State(state): State<AppState>,
    auth: AuthUser<MenuWriteAccess>,
    Path(category_id): Path<uuid::Uuid>,
) -> Result<StatusCode, MenuError> {
    if state.menu_service.deactivate_category(auth.user.tenant_id, category_id).await? {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(MenuError::Repository(anyhow::anyhow!("category not found")))
    }
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

pub async fn delete_product(
    State(state): State<AppState>,
    auth: AuthUser<MenuWriteAccess>,
    Path(product_id): Path<uuid::Uuid>,
) -> Result<StatusCode, MenuError> {
    if state.menu_service.delete_product(auth.user.tenant_id, product_id).await? {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(MenuError::Repository(anyhow::anyhow!("product not found")))
    }
}

pub async fn upsert_branch_override(
    State(state): State<AppState>,
    auth: AuthUser<BranchCatalogWriteAccess>,
    Path(product_id): Path<uuid::Uuid>,
    Json(payload): Json<BranchOverridePayload>,
) -> Result<StatusCode, MenuError> {
    let location_id = auth.branch.ok_or_else(|| MenuError::Repository(anyhow::anyhow!("se requiere el header x-branch-id")))?;
    state.menu_service.upsert_branch_override(auth.user.tenant_id, location_id, product_id, payload.price, payload.stock, payload.is_available).await?;
    Ok(StatusCode::NO_CONTENT)
}

impl IntoResponse for MenuError {
    fn into_response(self) -> axum::response::Response {
        let body = Json(serde_json::json!({
            "message": self.to_string(),
        }));
        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}