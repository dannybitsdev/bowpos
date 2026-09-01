use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{PgPool, Row};
use uuid::Uuid;
use crate::domain::orders::{BranchSalesSummary, CatalogOption, ModifierGroup, NewOrder, Order, OrderCatalogProduct, OrderItem, OrderOption, OrderRepository, OrderStatus, PaymentMethod, ServiceType, Totals};

pub struct SqlxOrderRepository { pool: PgPool }
impl SqlxOrderRepository { pub fn new(pool: PgPool) -> Self { Self { pool } } }

fn parse_service(value: &str) -> ServiceType { match value { "TAKEAWAY" => ServiceType::TAKEAWAY, "DELIVERY" => ServiceType::DELIVERY, _ => ServiceType::DINE_IN } }
fn parse_status(value: &str) -> OrderStatus { match value { "IN_PREPARATION" => OrderStatus::IN_PREPARATION, "READY" => OrderStatus::READY, "DELIVERED" => OrderStatus::DELIVERED, "CANCELLED" => OrderStatus::CANCELLED, _ => OrderStatus::CREATED } }
fn parse_payment(value: Option<String>) -> Option<PaymentMethod> { value.map(|item| match item.as_str() { "CARD" => PaymentMethod::CARD, "TRANSFER" => PaymentMethod::TRANSFER, _ => PaymentMethod::CASH }) }
fn money(value: String) -> f64 { value.parse().unwrap_or(0.0) }
fn branch_scope_value(branch: Option<Uuid>) -> String { branch.map(|value| value.to_string()).unwrap_or_else(|| "ALL".to_string()) }

#[async_trait]
impl OrderRepository for SqlxOrderRepository {
    async fn list_orders(&self, tenant_id: Uuid, branch: Option<Uuid>, status: Option<OrderStatus>) -> Result<Vec<Order>, anyhow::Error> {
        let mut transaction = self.pool.begin().await?;
        self.apply_rls_scope(&mut transaction, tenant_id, branch).await?;
        let rows = sqlx::query("SELECT id FROM orders WHERE tenant_id = $1 AND ($2::uuid IS NULL OR location_id = $2) AND ($3::text IS NULL OR status = $3) ORDER BY created_at DESC")
            .bind(tenant_id).bind(branch).bind(status.map(OrderStatus::as_str))
            .fetch_all(&mut *transaction).await?;
        transaction.commit().await?;

        let mut orders = Vec::with_capacity(rows.len());
        for row in rows { if let Some(order) = self.load_order(tenant_id, branch, row.try_get("id")?).await? { orders.push(order); } }
        Ok(orders)
    }

    async fn get_order(&self, tenant_id: Uuid, branch: Option<Uuid>, order_id: Uuid) -> Result<Option<Order>, anyhow::Error> { self.load_order(tenant_id, branch, order_id).await }

    async fn list_catalog(&self, tenant_id: Uuid) -> Result<Vec<OrderCatalogProduct>, anyhow::Error> {
        let products = sqlx::query("SELECT id, name, price::text AS price, image_url FROM products WHERE tenant_id = $1 AND is_active = true ORDER BY name").bind(tenant_id).fetch_all(&self.pool).await?;
        let mut catalog = Vec::with_capacity(products.len());
        for product in products {
            let product_id: Uuid = product.try_get("id")?;
            let group_rows = sqlx::query("SELECT mg.id, mg.name, mg.required, mg.min_selections, mg.max_selections FROM modifier_groups mg JOIN product_modifier_groups pmg ON pmg.modifier_group_id = mg.id AND pmg.tenant_id = mg.tenant_id WHERE pmg.product_id = $1 AND pmg.tenant_id = $2 AND mg.is_active = true ORDER BY mg.name").bind(product_id).bind(tenant_id).fetch_all(&self.pool).await?;
            let mut groups = Vec::with_capacity(group_rows.len());
            for group in group_rows {
                let group_id: Uuid = group.try_get("id")?;
                let modifier_rows = sqlx::query("SELECT id, name, price::text AS price FROM modifiers WHERE modifier_group_id = $1 AND tenant_id = $2 AND is_active = true ORDER BY name").bind(group_id).bind(tenant_id).fetch_all(&self.pool).await?;
                groups.push(ModifierGroup { id: group_id, name: group.try_get("name")?, required: group.try_get("required")?, min_selections: group.try_get("min_selections")?, max_selections: group.try_get("max_selections")?, modifiers: modifier_rows.into_iter().map(|row| Ok(CatalogOption { id: row.try_get("id")?, name: row.try_get("name")?, price: money(row.try_get("price")?) })).collect::<Result<_, anyhow::Error>>()? });
            }
            let topping_rows = sqlx::query("SELECT t.id, t.name, t.price::text AS price FROM toppings t JOIN product_toppings pt ON pt.topping_id = t.id AND pt.tenant_id = t.tenant_id WHERE pt.product_id = $1 AND pt.tenant_id = $2 AND t.is_active = true ORDER BY t.name").bind(product_id).bind(tenant_id).fetch_all(&self.pool).await?;
            catalog.push(OrderCatalogProduct { id: product_id, name: product.try_get("name")?, price: money(product.try_get("price")?), image_url: product.try_get("image_url")?, modifier_groups: groups, toppings: topping_rows.into_iter().map(|row| Ok(CatalogOption { id: row.try_get("id")?, name: row.try_get("name")?, price: money(row.try_get("price")?) })).collect::<Result<_, anyhow::Error>>()? });
        }
        Ok(catalog)
    }

    async fn create_order(&self, tenant_id: Uuid, branch_id: Uuid, user_id: Uuid, input: NewOrder) -> Result<Order, anyhow::Error> {
        let mut transaction = self.pool.begin().await?;
        self.apply_rls_scope(&mut transaction, tenant_id, Some(branch_id)).await?;
        sqlx::query("SELECT id FROM locations WHERE id = $1 AND tenant_id = $2").bind(branch_id).bind(tenant_id).fetch_optional(&mut *transaction).await?.ok_or_else(|| anyhow::anyhow!("branch not found for tenant"))?;
        let order_number: i64 = sqlx::query_scalar("SELECT COALESCE(MAX(order_number), 0) + 1 FROM orders WHERE tenant_id = $1").bind(tenant_id).fetch_one(&mut *transaction).await?;
        let mut priced_items = Vec::with_capacity(input.items.len());
        for item in &input.items {
            let product = sqlx::query("SELECT name, price::text AS price FROM products WHERE id = $1 AND tenant_id = $2 AND is_active = true").bind(item.product_id).bind(tenant_id).fetch_optional(&mut *transaction).await?.ok_or_else(|| anyhow::anyhow!("product not found"))?;
            let product_name: String = product.try_get("name")?;
            let unit_price = money(product.try_get("price")?);
            let modifiers = self.load_options_for_transaction(&mut transaction, tenant_id, item.product_id, &item.modifier_ids, true).await?;
            let toppings = self.load_options_for_transaction(&mut transaction, tenant_id, item.product_id, &item.topping_ids, false).await?;
            let option_total = modifiers.iter().chain(toppings.iter()).map(|option| option.2).sum::<f64>();
            priced_items.push((item, product_name, unit_price + option_total, modifiers, toppings));
        }
        let cents = priced_items.iter().map(|(item, _, price, _, _)| ((price * 100.0).round() as i64, item.quantity)).collect::<Vec<_>>();
        let totals = Totals::calculate(&cents, input.tax_rate, input.tip, input.discount);
        let order_id = Uuid::new_v4();
        sqlx::query("INSERT INTO orders (id, order_number, tenant_id, location_id, user_id, service_type, table_name, customer_name, notes, payment_method, subtotal, tax, tip, discount, total) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15)")
            .bind(order_id).bind(order_number).bind(tenant_id).bind(branch_id).bind(user_id).bind(input.service_type.as_str()).bind(&input.table_name).bind(&input.customer_name).bind(&input.notes).bind(input.payment_method.map(|item| item.as_str()))
            .bind(totals.subtotal as f64 / 100.0).bind(totals.tax as f64 / 100.0).bind(totals.tip as f64 / 100.0).bind(totals.discount as f64 / 100.0).bind(totals.total as f64 / 100.0).execute(&mut *transaction).await?;
        for (item, name, unit_price, modifiers, toppings) in priced_items {
            let item_id = Uuid::new_v4();
            sqlx::query("INSERT INTO order_items (id, tenant_id, order_id, product_id, product_name, quantity, unit_price, notes, subtotal) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9)")
                .bind(item_id).bind(tenant_id).bind(order_id).bind(item.product_id).bind(name).bind(item.quantity).bind(unit_price).bind(&item.notes).bind(unit_price * f64::from(item.quantity)).execute(&mut *transaction).await?;
            for (option_id, option_name, option_price) in modifiers.into_iter().chain(toppings.into_iter()) {
                let table = if item.modifier_ids.contains(&option_id) { "order_item_modifiers" } else { "order_item_toppings" };
                let query = format!("INSERT INTO {table} (id, tenant_id, order_item_id, {}_id, name, price) VALUES ($1,$2,$3,$4,$5,$6)", if table == "order_item_modifiers" { "modifier" } else { "topping" });
                sqlx::query(&query).bind(Uuid::new_v4()).bind(tenant_id).bind(item_id).bind(option_id).bind(option_name).bind(option_price).execute(&mut *transaction).await?;
            }
        }
        transaction.commit().await?;
        self.load_order(tenant_id, Some(branch_id), order_id).await?.ok_or_else(|| anyhow::anyhow!("created order not found"))
    }

    async fn update_status(&self, tenant_id: Uuid, branch: Option<Uuid>, order_id: Uuid, status: OrderStatus) -> Result<Option<Order>, anyhow::Error> {
        let mut transaction = self.pool.begin().await?;
        self.apply_rls_scope(&mut transaction, tenant_id, branch).await?;
        sqlx::query("UPDATE orders SET status = $1, updated_at = NOW() WHERE id = $2 AND tenant_id = $3 AND ($4::uuid IS NULL OR location_id = $4)")
            .bind(status.as_str()).bind(order_id).bind(tenant_id).bind(branch).execute(&mut *transaction).await?;
        transaction.commit().await?;
        self.load_order(tenant_id, branch, order_id).await
    }

    async fn sales_summary(&self, tenant_id: Uuid, branch: Option<Uuid>) -> Result<Vec<BranchSalesSummary>, anyhow::Error> {
        let mut transaction = self.pool.begin().await?;
        self.apply_rls_scope(&mut transaction, tenant_id, branch).await?;
        let rows = sqlx::query(
            "SELECT l.id AS location_id, l.name AS location_name, COUNT(o.id) AS order_count, COALESCE(SUM(o.total), 0)::text AS total
             FROM locations l
             LEFT JOIN orders o ON o.location_id = l.id AND o.tenant_id = l.tenant_id AND o.status <> 'CANCELLED'
             WHERE l.tenant_id = $1 AND ($2::uuid IS NULL OR l.id = $2)
             GROUP BY l.id, l.name
             ORDER BY l.name",
        )
        .bind(tenant_id).bind(branch).fetch_all(&mut *transaction).await?;
        transaction.commit().await?;

        rows.into_iter().map(|row| Ok(BranchSalesSummary {
            location_id: row.try_get("location_id")?,
            location_name: row.try_get("location_name")?,
            order_count: row.try_get("order_count")?,
            total: money(row.try_get("total")?),
        })).collect()
    }
}

impl SqlxOrderRepository {
    /// Fija las variables de sesión que respaldan las políticas RLS de `orders` (defensa en profundidad).
    /// El filtrado real de seguridad ocurre en las cláusulas WHERE de cada consulta.
    async fn apply_rls_scope(&self, transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>, tenant_id: Uuid, branch: Option<Uuid>) -> Result<(), anyhow::Error> {
        sqlx::query("SELECT set_config('app.tenant_id', $1, true)").bind(tenant_id.to_string()).execute(&mut **transaction).await?;
        sqlx::query("SELECT set_config('app.branch_scope', $1, true)").bind(branch_scope_value(branch)).execute(&mut **transaction).await?;
        Ok(())
    }

    async fn load_options_for_transaction(&self, transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>, tenant_id: Uuid, product_id: Uuid, ids: &[Uuid], modifiers: bool) -> Result<Vec<(Uuid, String, f64)>, anyhow::Error> {
        let mut result = Vec::new();
        for id in ids {
            let query = if modifiers {
                "SELECT m.id, m.name, m.price::text AS price FROM modifiers m JOIN product_modifier_groups pmg ON pmg.modifier_group_id = m.modifier_group_id AND pmg.tenant_id = m.tenant_id WHERE m.id = $1 AND m.tenant_id = $2 AND pmg.product_id = $3 AND m.is_active = true"
            } else {
                "SELECT t.id, t.name, t.price::text AS price FROM toppings t JOIN product_toppings pt ON pt.topping_id = t.id AND pt.tenant_id = t.tenant_id WHERE t.id = $1 AND t.tenant_id = $2 AND pt.product_id = $3 AND t.is_active = true"
            };
            let row = sqlx::query(query).bind(id).bind(tenant_id).bind(product_id).fetch_optional(&mut **transaction).await?.ok_or_else(|| anyhow::anyhow!("option not available for product"))?;
            result.push((*id, row.try_get("name")?, money(row.try_get("price")?)));
        }
        Ok(result)
    }

    async fn load_order(&self, tenant_id: Uuid, branch: Option<Uuid>, order_id: Uuid) -> Result<Option<Order>, anyhow::Error> {
        let mut transaction = self.pool.begin().await?;
        self.apply_rls_scope(&mut transaction, tenant_id, branch).await?;
        let row = sqlx::query("SELECT o.id, o.order_number, o.location_id, o.service_type, o.table_name, o.customer_name, o.notes, o.status, o.payment_method, o.subtotal::text AS subtotal, o.tax::text AS tax, o.tip::text AS tip, o.discount::text AS discount, o.total::text AS total, o.created_at, t.name AS tenant_name, l.name AS location_name FROM orders o JOIN tenants t ON t.id = o.tenant_id LEFT JOIN locations l ON l.id = o.location_id AND l.tenant_id = o.tenant_id WHERE o.id = $1 AND o.tenant_id = $2 AND ($3::uuid IS NULL OR o.location_id = $3)")
            .bind(order_id).bind(tenant_id).bind(branch).fetch_optional(&mut *transaction).await?;
        transaction.commit().await?;
        let Some(row) = row else { return Ok(None); };

        let item_rows = sqlx::query("SELECT id, product_id, product_name, quantity, unit_price::text AS unit_price, notes, subtotal::text AS subtotal FROM order_items WHERE order_id = $1 AND tenant_id = $2 ORDER BY product_name").bind(order_id).bind(tenant_id).fetch_all(&self.pool).await?;
        let mut items = Vec::new();
        for item in item_rows {
            let item_id: Uuid = item.try_get("id")?;
            let modifiers = self.load_item_options(item_id, tenant_id, "order_item_modifiers").await?;
            let toppings = self.load_item_options(item_id, tenant_id, "order_item_toppings").await?;
            items.push(OrderItem { id: item_id, product_id: item.try_get("product_id")?, product_name: item.try_get("product_name")?, quantity: item.try_get("quantity")?, unit_price: money(item.try_get("unit_price")?), notes: item.try_get("notes")?, subtotal: money(item.try_get("subtotal")?), modifiers, toppings });
        }
        Ok(Some(Order { id: order_id, order_number: row.try_get("order_number")?, tenant_id, tenant_name: row.try_get("tenant_name")?, location_id: row.try_get("location_id")?, location_name: row.try_get("location_name")?, service_type: parse_service(row.try_get("service_type")?), table_name: row.try_get("table_name")?, customer_name: row.try_get("customer_name")?, notes: row.try_get("notes")?, status: parse_status(row.try_get("status")?), payment_method: parse_payment(row.try_get("payment_method")?), subtotal: money(row.try_get("subtotal")?), tax: money(row.try_get("tax")?), tip: money(row.try_get("tip")?), discount: money(row.try_get("discount")?), total: money(row.try_get("total")?), items, created_at: row.try_get::<DateTime<Utc>, _>("created_at")? }))
    }
    async fn load_item_options(&self, item_id: Uuid, tenant_id: Uuid, table: &str) -> Result<Vec<OrderOption>, anyhow::Error> {
        let query = format!("SELECT id, name, price::text AS price FROM {table} WHERE order_item_id = $1 AND tenant_id = $2");
        Ok(sqlx::query(&query).bind(item_id).bind(tenant_id).fetch_all(&self.pool).await?.into_iter().map(|row| Ok(OrderOption { id: row.try_get("id")?, name: row.try_get("name")?, price: money(row.try_get("price")?) })).collect::<Result<_, anyhow::Error>>()?)
    }
}
