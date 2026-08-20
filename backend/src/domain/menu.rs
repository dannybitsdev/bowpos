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
}