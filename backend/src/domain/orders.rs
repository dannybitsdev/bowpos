use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum OrderStatus {
    #[serde(rename = "CREATED")] CREATED,
    #[serde(rename = "IN_PREPARATION")] IN_PREPARATION,
    #[serde(rename = "READY")] READY,
    #[serde(rename = "DELIVERED")] DELIVERED,
    #[serde(rename = "CANCELLED")] CANCELLED,
}

impl OrderStatus {
    pub fn can_transition_to(self, next: Self) -> bool {
        matches!((self, next),
            (Self::CREATED, Self::IN_PREPARATION | Self::CANCELLED) |
            (Self::IN_PREPARATION, Self::READY | Self::CANCELLED) |
            (Self::READY, Self::DELIVERED) |
            (Self::DELIVERED, Self::DELIVERED) |
            (Self::CANCELLED, Self::CANCELLED))
    }
    pub fn as_str(self) -> &'static str { match self { Self::CREATED => "CREATED", Self::IN_PREPARATION => "IN_PREPARATION", Self::READY => "READY", Self::DELIVERED => "DELIVERED", Self::CANCELLED => "CANCELLED" } }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum ServiceType {
    #[serde(rename = "DINE_IN")] DINE_IN,
    #[serde(rename = "TAKEAWAY")] TAKEAWAY,
    #[serde(rename = "DELIVERY")] DELIVERY,
}
impl ServiceType { pub fn as_str(self) -> &'static str { match self { Self::DINE_IN => "DINE_IN", Self::TAKEAWAY => "TAKEAWAY", Self::DELIVERY => "DELIVERY" } } }

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum PaymentMethod {
    #[serde(rename = "CASH")] CASH,
    #[serde(rename = "CARD")] CARD,
    #[serde(rename = "TRANSFER")] TRANSFER,
}
impl PaymentMethod { pub fn as_str(self) -> &'static str { match self { Self::CASH => "CASH", Self::CARD => "CARD", Self::TRANSFER => "TRANSFER" } } }

#[derive(Debug, Clone, Serialize)]
pub struct Order {
    pub id: Uuid, pub order_number: i64, pub tenant_id: Uuid, pub tenant_name: String, pub location_name: Option<String>, pub service_type: ServiceType, pub table_name: Option<String>,
    pub customer_name: Option<String>, pub notes: Option<String>, pub status: OrderStatus,
    pub payment_method: Option<PaymentMethod>, pub subtotal: f64, pub tax: f64, pub tip: f64,
    pub discount: f64, pub total: f64, pub items: Vec<OrderItem>, pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize)]
pub struct OrderItem {
    pub id: Uuid, pub product_id: Uuid, pub product_name: String, pub quantity: i32,
    pub unit_price: f64, pub notes: Option<String>, pub subtotal: f64,
    pub modifiers: Vec<OrderOption>, pub toppings: Vec<OrderOption>,
}

#[derive(Debug, Clone, Serialize)]
pub struct OrderOption { pub id: Uuid, pub name: String, pub price: f64 }

#[derive(Debug, Clone)]
pub struct NewOrder {
    pub service_type: ServiceType, pub table_name: Option<String>, pub customer_name: Option<String>,
    pub notes: Option<String>, pub payment_method: Option<PaymentMethod>, pub tax_rate: f64,
    pub tip: f64, pub discount: f64, pub items: Vec<NewOrderItem>,
}
#[derive(Debug, Clone)]
pub struct NewOrderItem { pub product_id: Uuid, pub quantity: i32, pub notes: Option<String>, pub modifier_ids: Vec<Uuid>, pub topping_ids: Vec<Uuid> }

#[derive(Debug, Clone, Serialize)]
pub struct OrderCatalogProduct { pub id: Uuid, pub name: String, pub price: f64, pub image_url: Option<String>, pub modifier_groups: Vec<ModifierGroup>, pub toppings: Vec<CatalogOption> }
#[derive(Debug, Clone, Serialize)]
pub struct ModifierGroup { pub id: Uuid, pub name: String, pub required: bool, pub min_selections: i32, pub max_selections: i32, pub modifiers: Vec<CatalogOption> }
#[derive(Debug, Clone, Serialize)]
pub struct CatalogOption { pub id: Uuid, pub name: String, pub price: f64 }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Totals { pub subtotal: i64, pub tax: i64, pub tip: i64, pub discount: i64, pub total: i64 }
impl Totals {
    pub fn calculate(items: &[(i64, i32)], tax_rate: f64, tip: f64, discount: f64) -> Self {
        let subtotal = items.iter().map(|(price, quantity)| price * i64::from(*quantity)).sum::<i64>();
        let tax = ((subtotal as f64) * tax_rate).round() as i64;
        let tip = (tip * 100.0).round() as i64;
        let discount = (discount * 100.0).round() as i64;
        Self { subtotal, tax, tip, discount, total: (subtotal + tax + tip - discount).max(0) }
    }
}

#[async_trait]
pub trait OrderRepository: Send + Sync {
    async fn list_orders(&self, tenant_id: Uuid, status: Option<OrderStatus>) -> Result<Vec<Order>, anyhow::Error>;
    async fn list_catalog(&self, tenant_id: Uuid) -> Result<Vec<OrderCatalogProduct>, anyhow::Error>;
    async fn create_order(&self, tenant_id: Uuid, user_id: Uuid, order: NewOrder) -> Result<Order, anyhow::Error>;
    async fn get_order(&self, tenant_id: Uuid, order_id: Uuid) -> Result<Option<Order>, anyhow::Error>;
    async fn update_status(&self, tenant_id: Uuid, order_id: Uuid, status: OrderStatus) -> Result<Option<Order>, anyhow::Error>;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn calculates_totals_in_cents() {
        let totals = Totals::calculate(&[(1250, 2), (300, 1)], 0.19, 5.0, 2.0);
        assert_eq!(totals, Totals { subtotal: 2800, tax: 532, tip: 500, discount: 200, total: 3632 });
    }
    #[test]
    fn rejects_invalid_state_transitions() {
        assert!(!OrderStatus::DELIVERED.can_transition_to(OrderStatus::CANCELLED));
        assert!(OrderStatus::CREATED.can_transition_to(OrderStatus::IN_PREPARATION));
    }
}
