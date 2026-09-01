use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Product {
    pub id: Uuid,
    pub category_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub price: f64,
    pub image_url: Option<String>,
    pub stock: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Category {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub display_order: i32,
    pub products: Vec<Product>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ModifierGroup {
    pub id: Uuid,
    pub name: String,
    pub required: bool,
    pub min_selections: i32,
    pub max_selections: i32,
    pub is_active: bool,
    pub modifiers: Vec<Modifier>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Modifier {
    pub id: Uuid,
    pub modifier_group_id: Uuid,
    pub name: String,
    pub price_delta: f64,
    pub is_active: bool,
}

#[async_trait]
pub trait MenuRepository: Send + Sync {
    /// `branch` fusiona los overrides de precio/stock/disponibilidad de esa sede sobre el cat\u00e1logo global.
    async fn list_menu(&self, tenant_id: Uuid, branch: Option<Uuid>) -> Result<Vec<Category>, anyhow::Error>;
    async fn list_products(&self, tenant_id: Uuid) -> Result<Vec<Product>, anyhow::Error>;
    async fn list_categories(&self, tenant_id: Uuid) -> Result<Vec<Category>, anyhow::Error>;
    async fn create_product(
        &self,
        tenant_id: Uuid,
        category_id: Uuid,
        name: &str,
        description: Option<&str>,
        price: f64,
        stock: i32,
        image_url: Option<&str>,
    ) -> Result<Product, anyhow::Error>;
    async fn update_product(
        &self,
        tenant_id: Uuid,
        product_id: Uuid,
        category_id: Uuid,
        name: &str,
        description: Option<&str>,
        price: f64,
        stock: i32,
        image_url: Option<&str>,
    ) -> Result<Option<Product>, anyhow::Error>;
    async fn delete_product(&self, tenant_id: Uuid, product_id: Uuid) -> Result<bool, anyhow::Error>;
    async fn create_category(&self, tenant_id: Uuid, name: &str, description: Option<&str>, image_url: Option<&str>, display_order: i32) -> Result<Category, anyhow::Error>;
    async fn update_category(&self, tenant_id: Uuid, category_id: Uuid, name: &str, description: Option<&str>, image_url: Option<&str>, display_order: i32) -> Result<Option<Category>, anyhow::Error>;
    async fn deactivate_category(&self, tenant_id: Uuid, category_id: Uuid) -> Result<bool, anyhow::Error>;
    async fn upsert_branch_override(
        &self,
        tenant_id: Uuid,
        location_id: Uuid,
        product_id: Uuid,
        price: Option<f64>,
        stock: Option<i32>,
        is_available: bool,
    ) -> Result<(), anyhow::Error>;

    // --- CRUD de Modificadores / Add-ons ---
    async fn list_modifier_groups(&self, tenant_id: Uuid) -> Result<Vec<ModifierGroup>, anyhow::Error>;
    async fn create_modifier_group(&self, tenant_id: Uuid, name: &str, required: bool, min_selections: i32, max_selections: i32) -> Result<ModifierGroup, anyhow::Error>;
    async fn update_modifier_group(&self, tenant_id: Uuid, group_id: Uuid, name: &str, required: bool, min_selections: i32, max_selections: i32) -> Result<Option<ModifierGroup>, anyhow::Error>;
    async fn deactivate_modifier_group(&self, tenant_id: Uuid, group_id: Uuid) -> Result<bool, anyhow::Error>;
    async fn create_modifier(&self, tenant_id: Uuid, group_id: Uuid, name: &str, price_delta: f64) -> Result<Modifier, anyhow::Error>;
    async fn update_modifier(&self, tenant_id: Uuid, modifier_id: Uuid, name: &str, price_delta: f64, is_active: bool) -> Result<Option<Modifier>, anyhow::Error>;
    async fn delete_modifier(&self, tenant_id: Uuid, modifier_id: Uuid) -> Result<bool, anyhow::Error>;
    /// Reemplaza por completo el conjunto de grupos de modificadores asociados a un producto.
    async fn set_product_modifier_groups(&self, tenant_id: Uuid, product_id: Uuid, group_ids: &[Uuid]) -> Result<(), anyhow::Error>;
    async fn list_product_modifier_group_ids(&self, tenant_id: Uuid, product_id: Uuid) -> Result<Vec<Uuid>, anyhow::Error>;
}
