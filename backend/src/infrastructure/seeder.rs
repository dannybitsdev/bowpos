use sqlx::PgPool;
use uuid::Uuid;

use crate::infrastructure::services::password_hasher::PasswordHasher;

pub async fn seed_initial_super_admin(pool: &PgPool) -> Result<(), anyhow::Error> {
    let tenant_id = Uuid::new_v4();
    let hashed = PasswordHasher::default().hash("BitsTITecnologia!2026")?;

    sqlx::query(
        r#"
        INSERT INTO tenants (id, name, slug, created_at)
        VALUES ($1, $2, $3, NOW())
        ON CONFLICT (slug) DO NOTHING
        "#,
    )
    .bind(tenant_id)
    .bind("Bits TI Tecnología")
    .bind("bits-ti-tecnologia")
    .execute(pool)
    .await?;

    let persisted_tenant_id = sqlx::query_scalar::<_, Uuid>(
        "SELECT id FROM tenants WHERE slug = 'bits-ti-tecnologia' LIMIT 1",
    )
    .fetch_one(pool)
    .await?;

    sqlx::query(
        r#"
        INSERT INTO users (id, tenant_id, location_id, name, email, password_hash, role)
        VALUES ($1, $2, NULL, $3, $4, $5, 'SUPER_ADMIN')
        ON CONFLICT (email) DO UPDATE SET
            tenant_id = EXCLUDED.tenant_id,
            name = EXCLUDED.name,
            password_hash = EXCLUDED.password_hash,
            role = EXCLUDED.role
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(persisted_tenant_id)
    .bind("Bits TI Tecnología")
    .bind("superadmin@bitstitecnologia.com")
    .bind(hashed)
    .execute(pool)
    .await?;

    let categories = [("Entradas", "Para compartir"), ("Platos principales", "Preparaciones de la casa"), ("Bebidas", "Bebidas frías y calientes")];
    for (index, (name, description)) in categories.iter().enumerate() {
        sqlx::query("INSERT INTO categories (id, tenant_id, name, description, display_order) VALUES ($1, $2, $3, $4, $5) ON CONFLICT (tenant_id, name) DO NOTHING")
            .bind(Uuid::new_v4()).bind(persisted_tenant_id).bind(name).bind(description).bind(index as i32).execute(pool).await?;
    }

    let products = [
        ("Arepa de chocolo", "Arepa dulce con queso", 12000.0, 20, "Entradas", "https://images.unsplash.com/photo-1601050690597-df0568f70950?auto=format&fit=crop&w=800&q=80"),
        ("Bandeja paisa", "Frijoles, arroz, carne y acompañamientos", 28000.0, 15, "Platos principales", "https://images.unsplash.com/photo-1547592180-85f173990554?auto=format&fit=crop&w=800&q=80"),
        ("Limonada natural", "Limonada preparada al momento", 7000.0, 30, "Bebidas", "https://images.unsplash.com/photo-1513558161293-cdaf765ed2fd?auto=format&fit=crop&w=800&q=80"),
    ];
    for (name, description, price, stock, category_name, image_url) in products {
        sqlx::query("UPDATE products SET description = $1, price = $2, stock = $3, image_url = $4, updated_at = NOW() WHERE tenant_id = $5 AND name = $6")
            .bind(description).bind(price).bind(stock).bind(image_url).bind(persisted_tenant_id).bind(name).execute(pool).await?;
        sqlx::query("INSERT INTO products (id, tenant_id, category_id, name, description, price, stock, image_url) SELECT $1, $2, id, $3, $4, $5, $6, $7 FROM categories WHERE tenant_id = $2 AND name = $8 AND NOT EXISTS (SELECT 1 FROM products WHERE tenant_id = $2 AND name = $3)")
            .bind(Uuid::new_v4()).bind(persisted_tenant_id).bind(name).bind(description).bind(price).bind(stock).bind(image_url).bind(category_name).execute(pool).await?;
    }

    Ok(())
}
