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

#[derive(serde::Deserialize)]
pub struct ModifierGroupPayload {
    pub name: String,
    #[serde(default)]
    pub required: bool,
    #[serde(default)]
    pub min_selections: i32,
    #[serde(default = "default_max_selections")]
    pub max_selections: i32,
}
fn default_max_selections() -> i32 { 1 }

#[derive(serde::Deserialize)]
pub struct ModifierPayload {
    pub name: String,
    pub price_delta: f64,
    #[serde(default = "default_true")]
    pub is_active: bool,
}

#[derive(serde::Serialize)]
pub struct ModifierGroupsResponse {
    pub data: Vec<crate::domain::menu::ModifierGroup>,
}

#[derive(serde::Deserialize)]
pub struct ProductModifierGroupsPayload {
    pub modifier_group_ids: Vec<uuid::Uuid>,
}

#[derive(serde::Serialize)]
pub struct ProductModifierGroupIdsResponse {
    pub data: Vec<uuid::Uuid>,
}

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
        .route("/menu/modifier-groups", get(list_modifier_groups))
        .route("/menu/modifier-groups", post(create_modifier_group))
        .route("/menu/modifier-groups/:group_id", put(update_modifier_group))
        .route("/menu/modifier-groups/:group_id", delete(deactivate_modifier_group))
        .route("/menu/modifier-groups/:group_id/modifiers", post(create_modifier))
        .route("/menu/modifiers/:modifier_id", put(update_modifier))
        .route("/menu/modifiers/:modifier_id", delete(delete_modifier))
        .route("/menu/products/:product_id/modifier-groups", get(list_product_modifier_groups))
        .route("/menu/products/:product_id/modifier-groups", put(set_product_modifier_groups))
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

pub async fn list_modifier_groups(
    State(state): State<AppState>,
    auth: AuthUser<MenuReadAccess>,
) -> Result<Json<ModifierGroupsResponse>, MenuError> {
    Ok(Json(ModifierGroupsResponse { data: state.menu_service.list_modifier_groups(auth.user.tenant_id).await? }))
}

pub async fn create_modifier_group(
    State(state): State<AppState>,
    auth: AuthUser<MenuWriteAccess>,
    Json(payload): Json<ModifierGroupPayload>,
) -> Result<(StatusCode, Json<crate::domain::menu::ModifierGroup>), MenuError> {
    let group = state.menu_service.create_modifier_group(auth.user.tenant_id, &payload.name, payload.required, payload.min_selections, payload.max_selections).await?;
    Ok((StatusCode::CREATED, Json(group)))
}

pub async fn update_modifier_group(
    State(state): State<AppState>,
    auth: AuthUser<MenuWriteAccess>,
    Path(group_id): Path<uuid::Uuid>,
    Json(payload): Json<ModifierGroupPayload>,
) -> Result<Json<crate::domain::menu::ModifierGroup>, MenuError> {
    let group = state.menu_service.update_modifier_group(auth.user.tenant_id, group_id, &payload.name, payload.required, payload.min_selections, payload.max_selections).await?;
    group.map(Json).ok_or_else(|| MenuError::Repository(anyhow::anyhow!("modifier group not found")))
}

pub async fn deactivate_modifier_group(
    State(state): State<AppState>,
    auth: AuthUser<MenuWriteAccess>,
    Path(group_id): Path<uuid::Uuid>,
) -> Result<StatusCode, MenuError> {
    if state.menu_service.deactivate_modifier_group(auth.user.tenant_id, group_id).await? {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(MenuError::Repository(anyhow::anyhow!("modifier group not found")))
    }
}

pub async fn create_modifier(
    State(state): State<AppState>,
    auth: AuthUser<MenuWriteAccess>,
    Path(group_id): Path<uuid::Uuid>,
    Json(payload): Json<ModifierPayload>,
) -> Result<(StatusCode, Json<crate::domain::menu::Modifier>), MenuError> {
    let modifier = state.menu_service.create_modifier(auth.user.tenant_id, group_id, &payload.name, payload.price_delta).await?;
    Ok((StatusCode::CREATED, Json(modifier)))
}

pub async fn update_modifier(
    State(state): State<AppState>,
    auth: AuthUser<MenuWriteAccess>,
    Path(modifier_id): Path<uuid::Uuid>,
    Json(payload): Json<ModifierPayload>,
) -> Result<Json<crate::domain::menu::Modifier>, MenuError> {
    let modifier = state.menu_service.update_modifier(auth.user.tenant_id, modifier_id, &payload.name, payload.price_delta, payload.is_active).await?;
    modifier.map(Json).ok_or_else(|| MenuError::Repository(anyhow::anyhow!("modifier not found")))
}

pub async fn delete_modifier(
    State(state): State<AppState>,
    auth: AuthUser<MenuWriteAccess>,
    Path(modifier_id): Path<uuid::Uuid>,
) -> Result<StatusCode, MenuError> {
    if state.menu_service.delete_modifier(auth.user.tenant_id, modifier_id).await? {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(MenuError::Repository(anyhow::anyhow!("modifier not found")))
    }
}

pub async fn list_product_modifier_groups(
    State(state): State<AppState>,
    auth: AuthUser<MenuReadAccess>,
    Path(product_id): Path<uuid::Uuid>,
) -> Result<Json<ProductModifierGroupIdsResponse>, MenuError> {
    let ids = state.menu_service.list_product_modifier_group_ids(auth.user.tenant_id, product_id).await?;
    Ok(Json(ProductModifierGroupIdsResponse { data: ids }))
}

pub async fn set_product_modifier_groups(
    State(state): State<AppState>,
    auth: AuthUser<MenuWriteAccess>,
    Path(product_id): Path<uuid::Uuid>,
    Json(payload): Json<ProductModifierGroupsPayload>,
) -> Result<StatusCode, MenuError> {
    state.menu_service.set_product_modifier_groups(auth.user.tenant_id, product_id, &payload.modifier_group_ids).await?;
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