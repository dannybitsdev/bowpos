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

    seed_catalog(pool, persisted_tenant_id, "bits-ti-tecnologia").await?;

    let demo_tenant_id = ensure_tenant(pool, "Restaurante Demo", "restaurante-demo").await?;
    seed_catalog(pool, demo_tenant_id, "restaurante-demo").await?;

    seed_products(pool, persisted_tenant_id).await?;
    seed_products(pool, demo_tenant_id).await?;

    Ok(())
}

async fn ensure_tenant(pool: &PgPool, name: &str, slug: &str) -> Result<Uuid, anyhow::Error> {
    sqlx::query("INSERT INTO tenants (id, name, slug, created_at) VALUES ($1, $2, $3, NOW()) ON CONFLICT (slug) DO NOTHING")
        .bind(Uuid::new_v4()).bind(name).bind(slug).execute(pool).await?;
    Ok(sqlx::query_scalar("SELECT id FROM tenants WHERE slug = $1").bind(slug).fetch_one(pool).await?)
}

async fn seed_catalog(pool: &PgPool, tenant_id: Uuid, _tenant_slug: &str) -> Result<(), anyhow::Error> {
    let categories = [
        ("Entradas", "Para compartir", "https://images.unsplash.com/photo-1601050690597-df0568f70950?auto=format&fit=crop&w=800&q=80"),
        ("Platos principales", "Preparaciones de la casa", "https://images.unsplash.com/photo-1547592180-85f173990554?auto=format&fit=crop&w=800&q=80"),
        ("Bebidas", "Bebidas frías y calientes", "https://images.unsplash.com/photo-1513558161293-cdaf765ed2fd?auto=format&fit=crop&w=800&q=80"),
        ("Postres", "Dulces para cerrar la experiencia", "https://images.unsplash.com/photo-1551024506-0bccd828d307?auto=format&fit=crop&w=800&q=80"),
        ("Promociones", "Opciones especiales del día", "https://images.unsplash.com/photo-1550547660-d9450f859349?auto=format&fit=crop&w=800&q=80"),
    ];
    for (index, (name, description, image_url)) in categories.iter().enumerate() {
        sqlx::query("INSERT INTO categories (id, tenant_id, name, description, image_url, display_order) VALUES ($1, $2, $3, $4, $5, $6) ON CONFLICT (tenant_id, name) DO UPDATE SET description = EXCLUDED.description, image_url = EXCLUDED.image_url, display_order = EXCLUDED.display_order")
            .bind(Uuid::new_v4()).bind(tenant_id).bind(name).bind(description).bind(image_url).bind(index as i32).execute(pool).await?;
    }
    Ok(())
}

async fn seed_products(pool: &PgPool, tenant_id: Uuid) -> Result<(), anyhow::Error> {
    let products = [
        ("Arepa de chocolo", "Arepa dulce con queso", 12000.0, 20, "Entradas", "https://images.unsplash.com/photo-1601050690597-df0568f70950?auto=format&fit=crop&w=800&q=80"),
        ("Bandeja paisa", "Frijoles, arroz, carne y acompañamientos", 28000.0, 15, "Platos principales", "https://images.unsplash.com/photo-1547592180-85f173990554?auto=format&fit=crop&w=800&q=80"),
        ("Limonada natural", "Limonada preparada al momento", 7000.0, 30, "Bebidas", "https://images.unsplash.com/photo-1513558161293-cdaf765ed2fd?auto=format&fit=crop&w=800&q=80"),
    ];
    for (name, description, price, stock, category_name, image_url) in products {
        sqlx::query("INSERT INTO products (id, tenant_id, category_id, name, description, price, stock, image_url) SELECT $1, $2, id, $3, $4, $5, $6, $7 FROM categories WHERE tenant_id = $2 AND name = $8 AND NOT EXISTS (SELECT 1 FROM products WHERE tenant_id = $2 AND name = $3)")
            .bind(Uuid::new_v4()).bind(tenant_id).bind(name).bind(description).bind(price).bind(stock).bind(image_url).bind(category_name).execute(pool).await?;
    }
    Ok(())
}
