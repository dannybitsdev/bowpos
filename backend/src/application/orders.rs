use std::sync::Arc;
use thiserror::Error;
use uuid::Uuid;
use crate::domain::orders::{BranchSalesSummary, NewOrder, Order, OrderCatalogProduct, OrderRepository, OrderStatus};

#[derive(Debug, Error)]
pub enum OrderError {
    #[error("order repository error")]
    Repository(#[source] anyhow::Error),
    #[error("invalid order: {0}")]
    Invalid(String),
    #[error("order not found")]
    NotFound,
    #[error("invalid status transition")]
    InvalidTransition,
}

pub struct OrderService { repository: Arc<dyn OrderRepository> }
impl OrderService {
    pub fn new(repository: Arc<dyn OrderRepository>) -> Self { Self { repository } }
    pub async fn list_orders(&self, tenant_id: Uuid, branch: Option<Uuid>, status: Option<OrderStatus>) -> Result<Vec<Order>, OrderError> { self.repository.list_orders(tenant_id, branch, status).await.map_err(OrderError::Repository) }
    pub async fn list_catalog(&self, tenant_id: Uuid) -> Result<Vec<OrderCatalogProduct>, OrderError> { self.repository.list_catalog(tenant_id).await.map_err(OrderError::Repository) }
    pub async fn get_order(&self, tenant_id: Uuid, branch: Option<Uuid>, order_id: Uuid) -> Result<Order, OrderError> { self.repository.get_order(tenant_id, branch, order_id).await.map_err(OrderError::Repository)?.ok_or(OrderError::NotFound) }
    pub async fn create_order(&self, tenant_id: Uuid, branch_id: Uuid, user_id: Uuid, order: NewOrder) -> Result<Order, OrderError> {
        if order.items.is_empty() { return Err(OrderError::Invalid("la orden debe incluir al menos un producto".into())); }
        if order.items.iter().any(|item| item.quantity <= 0) { return Err(OrderError::Invalid("la cantidad debe ser mayor a cero".into())); }
        if !(0.0..=1.0).contains(&order.tax_rate) || order.tip < 0.0 || order.discount < 0.0 { return Err(OrderError::Invalid("desglose financiero inválido".into())); }
        self.repository.create_order(tenant_id, branch_id, user_id, order).await.map_err(OrderError::Repository)
    }
    pub async fn update_status(&self, tenant_id: Uuid, branch: Option<Uuid>, order_id: Uuid, status: OrderStatus) -> Result<Order, OrderError> {
        let current = self.get_order(tenant_id, branch, order_id).await?;
        if !current.status.can_transition_to(status) { return Err(OrderError::InvalidTransition); }
        self.repository.update_status(tenant_id, branch, order_id, status).await.map_err(OrderError::Repository)?.ok_or(OrderError::NotFound)
    }
    pub async fn sales_summary(&self, tenant_id: Uuid, branch: Option<Uuid>) -> Result<Vec<BranchSalesSummary>, OrderError> {
        self.repository.sales_summary(tenant_id, branch).await.map_err(OrderError::Repository)
    }
}
