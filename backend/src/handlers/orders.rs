use axum::{extract::{Path, Query, State}, http::StatusCode, response::IntoResponse, routing::{get, put}, Json, Router};
use serde::Deserialize;
use uuid::Uuid;
use crate::{application::orders::OrderError, domain::orders::{NewOrder, NewOrderItem, OrderStatus, PaymentMethod, ServiceType}, presentation::auth::{extractor::AuthUser, policy::{OrderReadAccess, OrderWriteAccess}}, AppState};

#[derive(Debug, Deserialize)]
pub struct CreateOrderPayload { pub service_type: ServiceType, pub table_name: Option<String>, pub customer_name: Option<String>, pub notes: Option<String>, pub payment_method: Option<PaymentMethod>, #[serde(default = "default_tax_rate")] pub tax_rate: f64, #[serde(default)] pub tip: f64, #[serde(default)] pub discount: f64, pub items: Vec<CreateOrderItemPayload> }
#[derive(Debug, Deserialize)]
pub struct CreateOrderItemPayload { pub product_id: Uuid, pub quantity: i32, pub notes: Option<String>, #[serde(default)] pub modifier_ids: Vec<Uuid>, #[serde(default)] pub topping_ids: Vec<Uuid> }
#[derive(Debug, Deserialize)] pub struct OrdersQuery { pub status: Option<OrderStatus> }
#[derive(Debug, Deserialize)] pub struct UpdateStatusPayload { pub status: OrderStatus }
fn default_tax_rate() -> f64 { 0.19 }

pub fn orders_router() -> Router<AppState> {
    Router::new().route("/orders", get(list_orders).post(create_order)).route("/orders/catalog", get(list_catalog)).route("/orders/summary", get(sales_summary)).route("/orders/:order_id", get(get_order)).route("/orders/:order_id/status", put(update_status))
}

async fn list_orders(State(state): State<AppState>, auth: AuthUser<OrderReadAccess>, Query(query): Query<OrdersQuery>) -> Result<impl IntoResponse, OrderError> { Ok(Json(state.order_service.list_orders(auth.user.tenant_id, auth.branch, query.status).await?)) }
async fn list_catalog(State(state): State<AppState>, auth: AuthUser<OrderReadAccess>) -> Result<impl IntoResponse, OrderError> { Ok(Json(state.order_service.list_catalog(auth.user.tenant_id).await?)) }
async fn sales_summary(State(state): State<AppState>, auth: AuthUser<OrderReadAccess>) -> Result<impl IntoResponse, OrderError> { Ok(Json(state.order_service.sales_summary(auth.user.tenant_id, auth.branch).await?)) }
async fn get_order(State(state): State<AppState>, auth: AuthUser<OrderReadAccess>, Path(order_id): Path<Uuid>) -> Result<impl IntoResponse, OrderError> { Ok(Json(state.order_service.get_order(auth.user.tenant_id, auth.branch, order_id).await?)) }
async fn create_order(State(state): State<AppState>, auth: AuthUser<OrderWriteAccess>, Json(payload): Json<CreateOrderPayload>) -> Result<impl IntoResponse, OrderError> {
    let branch_id = auth.branch.ok_or_else(|| OrderError::Invalid("se requiere el header x-branch-id para crear una orden".into()))?;
    let order = NewOrder { service_type: payload.service_type, table_name: payload.table_name, customer_name: payload.customer_name, notes: payload.notes, payment_method: payload.payment_method, tax_rate: payload.tax_rate, tip: payload.tip, discount: payload.discount, items: payload.items.into_iter().map(|item| NewOrderItem { product_id: item.product_id, quantity: item.quantity, notes: item.notes, modifier_ids: item.modifier_ids, topping_ids: item.topping_ids }).collect() };
    Ok((StatusCode::CREATED, Json(state.order_service.create_order(auth.user.tenant_id, branch_id, auth.user.id, order).await?)))
}
async fn update_status(State(state): State<AppState>, auth: AuthUser<OrderWriteAccess>, Path(order_id): Path<Uuid>, Json(payload): Json<UpdateStatusPayload>) -> Result<impl IntoResponse, OrderError> { Ok(Json(state.order_service.update_status(auth.user.tenant_id, auth.branch, order_id, payload.status).await?)) }

impl IntoResponse for OrderError {
    fn into_response(self) -> axum::response::Response { let (status, message) = match self { OrderError::NotFound => (StatusCode::NOT_FOUND, self.to_string()), OrderError::Invalid(_) | OrderError::InvalidTransition => (StatusCode::BAD_REQUEST, self.to_string()), OrderError::Repository(_) => (StatusCode::INTERNAL_SERVER_ERROR, "order repository error".to_string()) }; (status, Json(serde_json::json!({"message": message}))).into_response() }
}
