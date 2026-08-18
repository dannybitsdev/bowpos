use async_trait::async_trait;
use chrono::{DateTime, Duration, Utc};
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::domain::{
    auth::{Role, Tenant, User},
    menu::{Category, MenuRepository, Product},
    repositories::{LoginAttemptState, RefreshTokenRecord, UserRepository},
    value_objects::{email::Email, password_hash::PasswordHash},
};

#[derive(Clone)]
pub struct SqlxUserRepository {
    pool: PgPool,
}

impl SqlxUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    async fn get_tenant_name(&self, tenant_id: Uuid) -> Result<String, anyhow::Error> {
        let row = sqlx::query_scalar::<_, String>(
            r#"
            SELECT name
            FROM tenants
            WHERE id = $1
            LIMIT 1
            "#,
        )
        .bind(tenant_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.unwrap_or_else(|| "Bits TI Tecnología".to_string()))
    }
}

#[async_trait]
impl UserRepository for SqlxUserRepository {
    async fn total_users(&self) -> Result<i64, anyhow::Error> {
        let count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM users")
            .fetch_one(&self.pool)
            .await?;
        Ok(count)
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, anyhow::Error> {
        let row = sqlx::query(
            r#"
            SELECT id, tenant_id, name, email, password_hash, role
            FROM users
            WHERE email = $1
            LIMIT 1
            "#,
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await?;

        let Some(record) = row else {
            return Ok(None);
        };

        let tenant_id: Uuid = record.try_get("tenant_id")?;
        let tenant_name = self.get_tenant_name(tenant_id).await?;
        let role = Role::from_db(record.try_get::<&str, _>("role")?)
            .ok_or_else(|| anyhow::anyhow!("invalid role in database"))?;
        let email = Email::parse(record.try_get::<&str, _>("email")?)
            .map_err(|_| anyhow::anyhow!("invalid email in database"))?;
        let password_hash = PasswordHash::new(record.try_get::<String, _>("password_hash")?)
            .map_err(|_| anyhow::anyhow!("invalid hash in database"))?;

        Ok(Some(User {
            id: record.try_get("id")?,
            tenant_id,
            tenant_name,
            name: record.try_get("name")?,
            email,
            password_hash,
            role,
        }))
    }

    async fn find_by_id(&self, user_id: Uuid) -> Result<Option<User>, anyhow::Error> {
        let row = sqlx::query(
            r#"
            SELECT id, tenant_id, name, email, password_hash, role
            FROM users
            WHERE id = $1
            LIMIT 1
            "#,
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        let Some(record) = row else {
            return Ok(None);
        };

        let tenant_id: Uuid = record.try_get("tenant_id")?;
        let tenant_name = self.get_tenant_name(tenant_id).await?;
        let role = Role::from_db(record.try_get::<&str, _>("role")?)
            .ok_or_else(|| anyhow::anyhow!("invalid role in database"))?;
        let email = Email::parse(record.try_get::<&str, _>("email")?)
            .map_err(|_| anyhow::anyhow!("invalid email in database"))?;
        let password_hash = PasswordHash::new(record.try_get::<String, _>("password_hash")?)
            .map_err(|_| anyhow::anyhow!("invalid hash in database"))?;

        Ok(Some(User {
            id: record.try_get("id")?,
            tenant_id,
            tenant_name,
            name: record.try_get("name")?,
            email,
            password_hash,
            role,
        }))
    }

    async fn create_tenant(&self, name: &str, slug: &str) -> Result<Tenant, anyhow::Error> {
        let id = Uuid::new_v4();
        sqlx::query(
            r#"
            INSERT INTO tenants (id, name, slug, created_at)
            VALUES ($1, $2, $3, NOW())
            "#,
        )
        .bind(id)
        .bind(name)
        .bind(slug)
        .execute(&self.pool)
        .await?;

        Ok(Tenant {
            id,
            name: name.to_string(),
            slug: slug.to_string(),
        })
    }

    async fn get_tenant(&self, tenant_id: Uuid) -> Result<Option<Tenant>, anyhow::Error> {
        let row = sqlx::query(
            r#"
            SELECT id, name, slug
            FROM tenants
            WHERE id = $1
            LIMIT 1
            "#,
        )
        .bind(tenant_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|record| {
            let id: Uuid = record.get("id");
            let name: String = record.get("name");
            let slug: String = record.get("slug");
            Tenant { id, name, slug }
        }))
    }

    async fn create_user(
        &self,
        tenant_id: Uuid,
        name: &str,
        email: &str,
        password_hash: &str,
        role: Role,
    ) -> Result<User, anyhow::Error> {
        let id = Uuid::new_v4();
        let row = sqlx::query(
            r#"
            INSERT INTO users (id, tenant_id, location_id, name, email, password_hash, role)
            VALUES ($1, $2, NULL, $3, $4, $5, $6)
            RETURNING id, tenant_id, name, email, password_hash, role
            "#,
        )
        .bind(id)
        .bind(tenant_id)
        .bind(name)
        .bind(email)
        .bind(password_hash)
        .bind(role.as_str())
        .fetch_one(&self.pool)
        .await?;

        let tenant_id: Uuid = row.try_get("tenant_id")?;
        let tenant_name = self.get_tenant_name(tenant_id).await?;

        Ok(User {
            id: row.try_get("id")?,
            tenant_id,
            tenant_name,
            name: row.try_get("name")?,
            email: Email::parse(row.try_get::<&str, _>("email")?)
                .map_err(|_| anyhow::anyhow!("invalid email"))?,
            password_hash: PasswordHash::new(row.try_get::<String, _>("password_hash")?)
                .map_err(|_| anyhow::anyhow!("invalid hash"))?,
            role: Role::from_db(row.try_get::<&str, _>("role")?)
                .ok_or_else(|| anyhow::anyhow!("invalid role"))?,
        })
    }

    async fn persist_refresh_token(
        &self,
        id: Uuid,
        user_id: Uuid,
        tenant_id: Uuid,
        token_hash: &str,
        expires_at: DateTime<Utc>,
    ) -> Result<(), anyhow::Error> {
        sqlx::query(
            r#"
            INSERT INTO auth_refresh_tokens (id, user_id, tenant_id, token_hash, expires_at)
            VALUES ($1, $2, $3, $4, $5)
            "#,
        )
        .bind(id)
        .bind(user_id)
        .bind(tenant_id)
        .bind(token_hash)
        .bind(expires_at)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn get_valid_refresh_token(
        &self,
        token_hash: &str,
        now: DateTime<Utc>,
    ) -> Result<Option<RefreshTokenRecord>, anyhow::Error> {
        let row = sqlx::query(
            r#"
            SELECT id, user_id, tenant_id, expires_at
            FROM auth_refresh_tokens
            WHERE token_hash = $1
              AND revoked_at IS NULL
              AND expires_at > $2
            LIMIT 1
            "#,
        )
        .bind(token_hash)
        .bind(now)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|record| RefreshTokenRecord {
            id: record.get("id"),
            user_id: record.get("user_id"),
            tenant_id: record.get("tenant_id"),
            expires_at: record.get("expires_at"),
        }))
    }

    async fn revoke_refresh_token(
        &self,
        token_id: Uuid,
        replaced_by: Option<Uuid>,
    ) -> Result<(), anyhow::Error> {
        sqlx::query(
            r#"
            UPDATE auth_refresh_tokens
            SET revoked_at = NOW(), replaced_by = $2
            WHERE id = $1
            "#,
        )
        .bind(token_id)
        .bind(replaced_by)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn get_login_attempt_state(
        &self,
        email: &str,
    ) -> Result<Option<LoginAttemptState>, anyhow::Error> {
        let row = sqlx::query(
            r#"
            SELECT failed_attempts, locked_until
            FROM auth_login_attempts
            WHERE email = $1
            LIMIT 1
            "#,
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|record| LoginAttemptState {
            failed_attempts: record.get("failed_attempts"),
            locked_until: record.get("locked_until"),
        }))
    }

    async fn register_login_failure(
        &self,
        email: &str,
        max_attempts: i32,
        lock_minutes: i32,
    ) -> Result<LoginAttemptState, anyhow::Error> {
        let current = self.get_login_attempt_state(email).await?;
        let next_failed_attempts = current
            .as_ref()
            .map(|value| value.failed_attempts + 1)
            .unwrap_or(1);

        let locked_until = if next_failed_attempts >= max_attempts {
            Some(Utc::now() + Duration::minutes(lock_minutes as i64))
        } else {
            None
        };

        sqlx::query(
            r#"
            INSERT INTO auth_login_attempts (email, failed_attempts, locked_until, updated_at)
            VALUES ($1, $2, $3, NOW())
            ON CONFLICT (email)
            DO UPDATE SET
                failed_attempts = EXCLUDED.failed_attempts,
                locked_until = EXCLUDED.locked_until,
                updated_at = NOW()
            "#,
        )
        .bind(email)
        .bind(next_failed_attempts)
        .bind(locked_until)
        .execute(&self.pool)
        .await?;

        Ok(LoginAttemptState {
            failed_attempts: next_failed_attempts,
            locked_until,
        })
    }

    async fn reset_login_failures(&self, email: &str) -> Result<(), anyhow::Error> {
        sqlx::query(
            r#"
            DELETE FROM auth_login_attempts
            WHERE email = $1
            "#,
        )
        .bind(email)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}

#[async_trait]
impl MenuRepository for SqlxUserRepository {
    async fn list_menu(&self, tenant_id: Uuid) -> Result<Vec<Category>, anyhow::Error> {
        let rows = sqlx::query(
            r#"
            SELECT
                c.id AS category_id,
                c.name AS category_name,
                c.description AS category_description,
                c.image_url AS category_image_url,
                c.display_order,
                p.id AS product_id,
                p.name AS product_name,
                p.description AS product_description,
                p.price::text AS product_price,
                p.image_url AS product_image_url,
                p.stock AS product_stock
            FROM categories c
            LEFT JOIN products p
                ON p.category_id = c.id
                AND p.tenant_id = c.tenant_id
                AND p.is_active = true
            WHERE c.tenant_id = $1
                AND c.is_active = true
            ORDER BY c.display_order ASC, c.name ASC, p.name ASC
            "#,
        )
        .bind(tenant_id)
        .fetch_all(&self.pool)
        .await?;

        let mut categories: Vec<Category> = Vec::new();
        for row in rows {
            let category_id: Uuid = row.try_get("category_id")?;
            let category_index = categories.iter().position(|category| category.id == category_id);

            let category_index = match category_index {
                Some(index) => index,
                None => {
                    categories.push(Category {
                        id: category_id,
                        name: row.try_get("category_name")?,
                        description: row.try_get("category_description")?,
                        image_url: row.try_get("category_image_url")?,
                        display_order: row.try_get("display_order")?,
                        products: Vec::new(),
                    });
                    categories.len() - 1
                }
            };

            let product_id: Option<Uuid> = row.try_get("product_id")?;
            if let Some(product_id) = product_id {
                let price_text: String = row.try_get("product_price")?;
                categories[category_index].products.push(Product {
                    id: product_id,
                    category_id,
                    name: row.try_get("product_name")?,
                    description: row.try_get("product_description")?,
                    price: price_text.parse::<f64>()?,
                    image_url: row.try_get("product_image_url")?,
                    stock: row.try_get("product_stock")?,
                });
            }
        }

        Ok(categories)
    }

    async fn list_products(&self, tenant_id: Uuid) -> Result<Vec<Product>, anyhow::Error> {
        let rows = sqlx::query(
            "SELECT id, category_id, name, description, price::text AS price, stock, image_url FROM products WHERE tenant_id = $1 AND is_active = true ORDER BY name",
        )
        .bind(tenant_id)
        .fetch_all(&self.pool)
        .await?;

        rows.iter().map(product_from_row).collect()
    }

    async fn list_categories(&self, tenant_id: Uuid) -> Result<Vec<Category>, anyhow::Error> {
        let rows = sqlx::query(
            "SELECT id, name, description, image_url, display_order FROM categories WHERE tenant_id = $1 AND is_active = true ORDER BY display_order, name",
        )
        .bind(tenant_id)
        .fetch_all(&self.pool)
        .await?;

        rows.into_iter().map(|row| Ok(Category {
            id: row.try_get("id")?,
            name: row.try_get("name")?,
            description: row.try_get("description")?,
            image_url: row.try_get("image_url")?,
            display_order: row.try_get("display_order")?,
            products: Vec::new(),
        })).collect()
    }

    async fn create_product(
        &self, tenant_id: Uuid, category_id: Uuid, name: &str, description: Option<&str>,
        price: f64, stock: i32, image_url: Option<&str>,
    ) -> Result<Product, anyhow::Error> {
        let row = sqlx::query(
            "INSERT INTO products (id, tenant_id, category_id, name, description, price, stock, image_url) SELECT $1, $2, id, $3, $4, $5, $6, $7 FROM categories WHERE id = $8 AND tenant_id = $2 AND is_active = true RETURNING id, category_id, name, description, price::text AS price, stock, image_url",
        )
        .bind(Uuid::new_v4()).bind(tenant_id).bind(name).bind(description)
        .bind(price).bind(stock).bind(image_url).bind(category_id)
        .fetch_optional(&self.pool).await?
        .ok_or_else(|| anyhow::anyhow!("category not found for tenant"))?;
        product_from_row(&row)
    }

    async fn update_product(
        &self, tenant_id: Uuid, product_id: Uuid, category_id: Uuid, name: &str,
        description: Option<&str>, price: f64, stock: i32, image_url: Option<&str>,
    ) -> Result<Option<Product>, anyhow::Error> {
        let row = sqlx::query(
            "UPDATE products SET category_id = $1, name = $2, description = $3, price = $4, stock = $5, image_url = $6, updated_at = NOW() WHERE id = $7 AND tenant_id = $8 AND EXISTS (SELECT 1 FROM categories WHERE id = $1 AND tenant_id = $8 AND is_active = true) RETURNING id, category_id, name, description, price::text AS price, stock, image_url",
        )
        .bind(category_id).bind(name).bind(description).bind(price).bind(stock).bind(image_url)
        .bind(product_id).bind(tenant_id).fetch_optional(&self.pool).await?;
        row.as_ref().map(product_from_row).transpose()
    }

    async fn delete_product(&self, tenant_id: Uuid, product_id: Uuid) -> Result<bool, anyhow::Error> {
        let result = sqlx::query("DELETE FROM products WHERE id = $1 AND tenant_id = $2")
            .bind(product_id)
            .bind(tenant_id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected() == 1)
    }

    async fn create_category(&self, tenant_id: Uuid, name: &str, description: Option<&str>, image_url: Option<&str>, display_order: i32) -> Result<Category, anyhow::Error> {
        let row = sqlx::query("INSERT INTO categories (id, tenant_id, name, description, image_url, display_order) VALUES ($1, $2, $3, $4, $5, $6) RETURNING id, name, description, image_url, display_order")
            .bind(Uuid::new_v4()).bind(tenant_id).bind(name).bind(description).bind(image_url).bind(display_order)
            .fetch_one(&self.pool).await?;
        Ok(Category { id: row.try_get("id")?, name: row.try_get("name")?, description: row.try_get("description")?, image_url: row.try_get("image_url")?, display_order: row.try_get("display_order")?, products: Vec::new() })
    }

    async fn update_category(&self, tenant_id: Uuid, category_id: Uuid, name: &str, description: Option<&str>, image_url: Option<&str>, display_order: i32) -> Result<Option<Category>, anyhow::Error> {
        let row = sqlx::query("UPDATE categories SET name = $1, description = $2, image_url = $3, display_order = $4, updated_at = NOW() WHERE id = $5 AND tenant_id = $6 AND is_active = true RETURNING id, name, description, image_url, display_order")
            .bind(name).bind(description).bind(image_url).bind(display_order).bind(category_id).bind(tenant_id)
            .fetch_optional(&self.pool).await?;
        row.map(|row| Ok(Category { id: row.try_get("id")?, name: row.try_get("name")?, description: row.try_get("description")?, image_url: row.try_get("image_url")?, display_order: row.try_get("display_order")?, products: Vec::new() })).transpose()
    }

    async fn deactivate_category(&self, tenant_id: Uuid, category_id: Uuid) -> Result<bool, anyhow::Error> {
        let result = sqlx::query("UPDATE categories SET is_active = false, updated_at = NOW() WHERE id = $1 AND tenant_id = $2 AND is_active = true")
            .bind(category_id).bind(tenant_id).execute(&self.pool).await?;
        Ok(result.rows_affected() == 1)
    }
}

fn product_from_row(row: &sqlx::postgres::PgRow) -> Result<Product, anyhow::Error> {
    Ok(Product {
        id: row.try_get("id")?, category_id: row.try_get("category_id")?, name: row.try_get("name")?,
        description: row.try_get("description")?, price: row.try_get::<String, _>("price")?.parse()?,
        stock: row.try_get("stock")?, image_url: row.try_get("image_url")?,
    })
}
