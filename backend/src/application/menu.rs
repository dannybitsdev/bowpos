use std::sync::Arc;

use thiserror::Error;
use uuid::Uuid;

use crate::domain::menu::{Category, MenuRepository, Product};

#[derive(Debug, Error)]
pub enum MenuError {
    #[error("menu repository error")]
    Repository(#[source] anyhow::Error),
}

pub struct MenuService {
    repository: Arc<dyn MenuRepository>,
}

impl MenuService {
    pub fn new(repository: Arc<dyn MenuRepository>) -> Self {
        Self { repository }
    }

    pub async fn list_menu(&self, tenant_id: Uuid, branch: Option<Uuid>) -> Result<Vec<Category>, MenuError> {
        self.repository
            .list_menu(tenant_id, branch)
            .await
            .map_err(MenuError::Repository)
    }

    pub async fn list_products(&self, tenant_id: Uuid) -> Result<Vec<Product>, MenuError> {
        self.repository.list_products(tenant_id).await.map_err(MenuError::Repository)
    }

    pub async fn list_categories(&self, tenant_id: Uuid) -> Result<Vec<Category>, MenuError> {
        self.repository.list_categories(tenant_id).await.map_err(MenuError::Repository)
    }

    pub async fn create_product(
        &self, tenant_id: Uuid, category_id: Uuid, name: &str, description: Option<&str>,
        price: f64, stock: i32, image_url: Option<&str>,
    ) -> Result<Product, MenuError> {
        self.repository.create_product(tenant_id, category_id, name, description, price, stock, image_url)
            .await.map_err(MenuError::Repository)
    }

    pub async fn update_product(
        &self, tenant_id: Uuid, product_id: Uuid, category_id: Uuid, name: &str,
        description: Option<&str>, price: f64, stock: i32, image_url: Option<&str>,
    ) -> Result<Option<Product>, MenuError> {
        self.repository.update_product(tenant_id, product_id, category_id, name, description, price, stock, image_url)
            .await.map_err(MenuError::Repository)
    }

    pub async fn delete_product(&self, tenant_id: Uuid, product_id: Uuid) -> Result<bool, MenuError> {
        self.repository.delete_product(tenant_id, product_id).await.map_err(MenuError::Repository)
    }

    pub async fn create_category(&self, tenant_id: Uuid, name: &str, description: Option<&str>, image_url: Option<&str>, display_order: i32) -> Result<Category, MenuError> {
        self.repository.create_category(tenant_id, name, description, image_url, display_order).await.map_err(MenuError::Repository)
    }

    pub async fn update_category(&self, tenant_id: Uuid, category_id: Uuid, name: &str, description: Option<&str>, image_url: Option<&str>, display_order: i32) -> Result<Option<Category>, MenuError> {
        self.repository.update_category(tenant_id, category_id, name, description, image_url, display_order).await.map_err(MenuError::Repository)
    }

    pub async fn deactivate_category(&self, tenant_id: Uuid, category_id: Uuid) -> Result<bool, MenuError> {
        self.repository.deactivate_category(tenant_id, category_id).await.map_err(MenuError::Repository)
    }

    pub async fn upsert_branch_override(
        &self, tenant_id: Uuid, location_id: Uuid, product_id: Uuid,
        price: Option<f64>, stock: Option<i32>, is_available: bool,
    ) -> Result<(), MenuError> {
        if price.is_some_and(|value| value < 0.0) || stock.is_some_and(|value| value < 0) {
            return Err(MenuError::Repository(anyhow::anyhow!("invalid override values")));
        }
        self.repository.upsert_branch_override(tenant_id, location_id, product_id, price, stock, is_available).await.map_err(MenuError::Repository)
    }
}

#[cfg(test)]
mod tests {
    use async_trait::async_trait;

    use super::*;
    use crate::domain::menu::{Category, Product};

    struct MockMenuRepository {
        categories: Vec<Category>,
        tenant_id: Uuid,
    }

    #[async_trait]
    impl MenuRepository for MockMenuRepository {
        async fn list_menu(&self, tenant_id: Uuid, _branch: Option<Uuid>) -> Result<Vec<Category>, anyhow::Error> {
            if tenant_id != self.tenant_id {
                return Ok(Vec::new());
            }
            Ok(self.categories.clone())
        }

        async fn list_products(&self, _tenant_id: Uuid) -> Result<Vec<Product>, anyhow::Error> { Ok(Vec::new()) }

        async fn list_categories(&self, tenant_id: Uuid) -> Result<Vec<Category>, anyhow::Error> {
            if tenant_id == self.tenant_id { Ok(self.categories.clone()) } else { Ok(Vec::new()) }
        }
        async fn create_product(&self, _tenant_id: Uuid, _category_id: Uuid, _name: &str, _description: Option<&str>, _price: f64, _stock: i32, _image_url: Option<&str>) -> Result<Product, anyhow::Error> { unreachable!() }
        async fn update_product(&self, _tenant_id: Uuid, _product_id: Uuid, _category_id: Uuid, _name: &str, _description: Option<&str>, _price: f64, _stock: i32, _image_url: Option<&str>) -> Result<Option<Product>, anyhow::Error> { unreachable!() }
        async fn delete_product(&self, _tenant_id: Uuid, _product_id: Uuid) -> Result<bool, anyhow::Error> { unreachable!() }
        async fn create_category(&self, _tenant_id: Uuid, _name: &str, _description: Option<&str>, _image_url: Option<&str>, _display_order: i32) -> Result<Category, anyhow::Error> { unreachable!() }
        async fn update_category(&self, _tenant_id: Uuid, _category_id: Uuid, _name: &str, _description: Option<&str>, _image_url: Option<&str>, _display_order: i32) -> Result<Option<Category>, anyhow::Error> { unreachable!() }
        async fn deactivate_category(&self, _tenant_id: Uuid, _category_id: Uuid) -> Result<bool, anyhow::Error> { unreachable!() }
        async fn upsert_branch_override(&self, _tenant_id: Uuid, _location_id: Uuid, _product_id: Uuid, _price: Option<f64>, _stock: Option<i32>, _is_available: bool) -> Result<(), anyhow::Error> { unreachable!() }
    }

    #[tokio::test]
    async fn returns_only_the_authenticated_tenant_menu() {
        let tenant_id = Uuid::new_v4();
        let category = Category {
            id: Uuid::new_v4(),
            name: "Bebidas".to_string(),
            description: None,
            image_url: None,
            display_order: 1,
            products: vec![Product {
                id: Uuid::new_v4(),
                category_id: Uuid::new_v4(),
                name: "Cafe".to_string(),
                description: None,
                price: 4.5,
                image_url: None,
                stock: 0,
            }],
        };
        let service = MenuService::new(Arc::new(MockMenuRepository {
            categories: vec![category],
            tenant_id,
        }));

        assert_eq!(service.list_menu(tenant_id, None).await.unwrap().len(), 1);
        assert!(service.list_menu(Uuid::new_v4(), None).await.unwrap().is_empty());
    }

    #[tokio::test]
    async fn preserves_repository_alphabetical_product_order() {
        let tenant_id = Uuid::new_v4();
        let category = Category {
            id: Uuid::new_v4(),
            name: "Platos".to_string(),
            description: None,
            image_url: None,
            display_order: 1,
            products: vec![
                Product { id: Uuid::new_v4(), category_id: Uuid::new_v4(), name: "Arepa".to_string(), description: None, price: 1.0, image_url: None, stock: 0 },
                Product { id: Uuid::new_v4(), category_id: Uuid::new_v4(), name: "Bandeja".to_string(), description: None, price: 2.0, image_url: None, stock: 0 },
            ],
        };
        let service = MenuService::new(Arc::new(MockMenuRepository { categories: vec![category], tenant_id }));
        let menu = service.list_menu(tenant_id, None).await.unwrap();

        assert_eq!(menu[0].products[0].name, "Arepa");
        assert_eq!(menu[0].products[1].name, "Bandeja");
    }

    #[tokio::test]
    async fn categories_are_not_visible_to_another_tenant() {
        let tenant_id = Uuid::new_v4();
        let category = Category { id: Uuid::new_v4(), name: "Entradas".to_string(), description: None, image_url: None, display_order: 0, products: Vec::new() };
        let service = MenuService::new(Arc::new(MockMenuRepository { categories: vec![category], tenant_id }));

        assert_eq!(service.list_categories(tenant_id).await.unwrap().len(), 1);
        assert!(service.list_categories(Uuid::new_v4()).await.unwrap().is_empty());
    }
}