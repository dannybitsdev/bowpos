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
    ensure_location(pool, persisted_tenant_id, "Sede Principal").await?;

    let demo_tenant_id = ensure_tenant(pool, "Restaurante Demo", "restaurante-demo").await?;
    seed_catalog(pool, demo_tenant_id, "restaurante-demo").await?;
    ensure_location(pool, demo_tenant_id, "Sede Principal").await?;
    assign_existing_order_locations(pool, persisted_tenant_id).await?;
    assign_existing_order_locations(pool, demo_tenant_id).await?;

    seed_products(pool, persisted_tenant_id).await?;
    seed_products(pool, demo_tenant_id).await?;
    seed_order_catalog(pool, persisted_tenant_id).await?;
    seed_order_catalog(pool, demo_tenant_id).await?;
    seed_demo_orders(pool, persisted_tenant_id).await?;
    seed_demo_orders(pool, demo_tenant_id).await?;

    Ok(())
}

async fn seed_order_catalog(pool: &PgPool, tenant_id: Uuid) -> Result<(), anyhow::Error> {
    let meat_group = ensure_modifier_group(pool, tenant_id, "Término de carne", true, 1, 1).await?;
    for (name, price) in [("3/4", 0.0), ("Bien cocido", 0.0), ("Término medio", 0.0)] { ensure_modifier(pool, tenant_id, meat_group, name, price).await?; }
    let size_group = ensure_modifier_group(pool, tenant_id, "Tamaño de bebida", true, 1, 1).await?;
    for (name, price) in [("Pequeña", 0.0), ("Mediana", 1500.0), ("Grande", 3000.0)] { ensure_modifier(pool, tenant_id, size_group, name, price).await?; }
    let milk_group = ensure_modifier_group(pool, tenant_id, "Tipo de leche", false, 0, 1).await?;
    for (name, price) in [("Entera", 0.0), ("Almendras", 2500.0), ("Avena", 2000.0)] { ensure_modifier(pool, tenant_id, milk_group, name, price).await?; }
    let crust_group = ensure_modifier_group(pool, tenant_id, "Borde de pizza", false, 0, 1).await?;
    for (name, price) in [("Tradicional", 0.0), ("Relleno de queso", 5000.0)] { ensure_modifier(pool, tenant_id, crust_group, name, price).await?; }
    let topping_ids = [
        ensure_topping(pool, tenant_id, "Queso extra", 2500.0).await?,
        ensure_topping(pool, tenant_id, "Tocino", 4000.0).await?,
        ensure_topping(pool, tenant_id, "Salsa especial", 1500.0).await?,
        ensure_topping(pool, tenant_id, "Champiñones", 3000.0).await?,
    ];
    for product_name in ["Hamburguesa clásica", "Hamburguesa BBQ"] { associate_modifier_group(pool, tenant_id, product_name, meat_group).await?; associate_toppings(pool, tenant_id, product_name, &topping_ids[..3]).await?; }
    for product_name in ["Pizza margarita", "Pizza pepperoni"] { associate_modifier_group(pool, tenant_id, product_name, crust_group).await?; associate_toppings(pool, tenant_id, product_name, &topping_ids[0..4]).await?; }
    for product_name in ["Limonada natural", "Limonada de maracuyá", "Cold brew"] { associate_modifier_group(pool, tenant_id, product_name, size_group).await?; }
    associate_modifier_group(pool, tenant_id, "Capuccino de la casa", size_group).await?;
    associate_modifier_group(pool, tenant_id, "Capuccino de la casa", milk_group).await?;
    Ok(())
}

async fn associate_modifier_group(pool: &PgPool, tenant_id: Uuid, product_name: &str, group_id: Uuid) -> Result<(), anyhow::Error> {
    sqlx::query("INSERT INTO product_modifier_groups (tenant_id, product_id, modifier_group_id) SELECT $1, p.id, $3 FROM products p WHERE p.tenant_id = $1 AND p.name = $2 ON CONFLICT DO NOTHING").bind(tenant_id).bind(product_name).bind(group_id).execute(pool).await?;
    Ok(())
}

async fn associate_toppings(pool: &PgPool, tenant_id: Uuid, product_name: &str, topping_ids: &[Uuid]) -> Result<(), anyhow::Error> {
    for topping_id in topping_ids { sqlx::query("INSERT INTO product_toppings (tenant_id, product_id, topping_id) SELECT $1, p.id, $3 FROM products p WHERE p.tenant_id = $1 AND p.name = $2 ON CONFLICT DO NOTHING").bind(tenant_id).bind(product_name).bind(topping_id).execute(pool).await?; }
    Ok(())
}

async fn ensure_modifier_group(pool: &PgPool, tenant_id: Uuid, name: &str, required: bool, min: i32, max: i32) -> Result<Uuid, anyhow::Error> {
    sqlx::query("INSERT INTO modifier_groups (id, tenant_id, name, required, min_selections, max_selections) VALUES ($1,$2,$3,$4,$5,$6) ON CONFLICT (tenant_id, name) DO UPDATE SET required = EXCLUDED.required, min_selections = EXCLUDED.min_selections, max_selections = EXCLUDED.max_selections").bind(Uuid::new_v4()).bind(tenant_id).bind(name).bind(required).bind(min).bind(max).execute(pool).await?;
    Ok(sqlx::query_scalar("SELECT id FROM modifier_groups WHERE tenant_id = $1 AND name = $2").bind(tenant_id).bind(name).fetch_one(pool).await?)
}

async fn ensure_modifier(pool: &PgPool, tenant_id: Uuid, group_id: Uuid, name: &str, price: f64) -> Result<Uuid, anyhow::Error> {
    sqlx::query("INSERT INTO modifiers (id, tenant_id, modifier_group_id, name, price) SELECT $1,$2,$3,$4,$5 WHERE NOT EXISTS (SELECT 1 FROM modifiers WHERE tenant_id = $2 AND modifier_group_id = $3 AND name = $4)").bind(Uuid::new_v4()).bind(tenant_id).bind(group_id).bind(name).bind(price).execute(pool).await?;
    Ok(sqlx::query_scalar("SELECT id FROM modifiers WHERE tenant_id = $1 AND modifier_group_id = $2 AND name = $3").bind(tenant_id).bind(group_id).bind(name).fetch_one(pool).await?)
}

async fn ensure_topping(pool: &PgPool, tenant_id: Uuid, name: &str, price: f64) -> Result<Uuid, anyhow::Error> {
    sqlx::query("INSERT INTO toppings (id, tenant_id, name, price) VALUES ($1,$2,$3,$4) ON CONFLICT (tenant_id, name) DO UPDATE SET price = EXCLUDED.price").bind(Uuid::new_v4()).bind(tenant_id).bind(name).bind(price).execute(pool).await?;
    Ok(sqlx::query_scalar("SELECT id FROM toppings WHERE tenant_id = $1 AND name = $2").bind(tenant_id).bind(name).fetch_one(pool).await?)
}

async fn seed_demo_orders(pool: &PgPool, tenant_id: Uuid) -> Result<(), anyhow::Error> {
    let user_id = sqlx::query_scalar::<_, Uuid>("SELECT id FROM users WHERE tenant_id = $1 LIMIT 1").bind(tenant_id).fetch_optional(pool).await?;
    let product_id = sqlx::query_scalar::<_, Uuid>("SELECT id FROM products WHERE tenant_id = $1 ORDER BY name LIMIT 1").bind(tenant_id).fetch_optional(pool).await?;
    let Some((user_id, product_id)) = user_id.zip(product_id) else { return Ok(()); };
    for status in ["IN_PREPARATION", "DELIVERED", "CANCELLED"] {
        let exists = sqlx::query_scalar::<_, bool>("SELECT EXISTS (SELECT 1 FROM orders WHERE tenant_id = $1 AND notes = $2)").bind(tenant_id).bind(format!("Demo {status}")).fetch_one(pool).await?;
        if exists { continue; }
        let order_id = Uuid::new_v4();
        let item_id = Uuid::new_v4();
        let order_number: i64 = sqlx::query_scalar("SELECT COALESCE(MAX(order_number), 0) + 1 FROM orders WHERE tenant_id = $1").bind(tenant_id).fetch_one(pool).await?;
        sqlx::query("INSERT INTO orders (id, order_number, tenant_id, user_id, service_type, table_name, customer_name, notes, status, subtotal, tax, total) VALUES ($1,$2,$3,$4,'DINE_IN','Mesa 4','Cliente demo',$5,$6,28000,5320,33320)").bind(order_id).bind(order_number).bind(tenant_id).bind(user_id).bind(format!("Demo {status}")).bind(status).execute(pool).await?;
        sqlx::query("INSERT INTO order_items (id, tenant_id, order_id, product_id, product_name, quantity, unit_price, subtotal) SELECT $1,$2,$3,id,name,1,price,price FROM products WHERE id = $4 AND tenant_id = $2").bind(item_id).bind(tenant_id).bind(order_id).bind(product_id).execute(pool).await?;
    }
    Ok(())
}

async fn ensure_tenant(pool: &PgPool, name: &str, slug: &str) -> Result<Uuid, anyhow::Error> {
    sqlx::query("INSERT INTO tenants (id, name, slug, created_at) VALUES ($1, $2, $3, NOW()) ON CONFLICT (slug) DO NOTHING")
        .bind(Uuid::new_v4()).bind(name).bind(slug).execute(pool).await?;
    Ok(sqlx::query_scalar("SELECT id FROM tenants WHERE slug = $1").bind(slug).fetch_one(pool).await?)
}

async fn ensure_location(pool: &PgPool, tenant_id: Uuid, name: &str) -> Result<Uuid, anyhow::Error> {
    sqlx::query("INSERT INTO locations (id, tenant_id, name, address, city) SELECT $1, $2, $3, 'Calle Principal 100', 'Bogotá' WHERE NOT EXISTS (SELECT 1 FROM locations WHERE tenant_id = $2 AND name = $3)")
        .bind(Uuid::new_v4()).bind(tenant_id).bind(name).execute(pool).await?;
    Ok(sqlx::query_scalar("SELECT id FROM locations WHERE tenant_id = $1 AND name = $2").bind(tenant_id).bind(name).fetch_one(pool).await?)
}

async fn assign_existing_order_locations(pool: &PgPool, tenant_id: Uuid) -> Result<(), anyhow::Error> {
    sqlx::query("UPDATE orders SET location_id = (SELECT id FROM locations WHERE tenant_id = $1 ORDER BY name LIMIT 1) WHERE tenant_id = $1 AND location_id IS NULL")
        .bind(tenant_id).execute(pool).await?;
    Ok(())
}

async fn seed_catalog(pool: &PgPool, tenant_id: Uuid, _tenant_slug: &str) -> Result<(), anyhow::Error> {
    let categories = [
        ("Entradas", "Para compartir", "https://images.unsplash.com/photo-1601050690597-df0568f70950?auto=format&fit=crop&w=800&q=80"),
        ("Platos principales", "Preparaciones de la casa", "https://images.unsplash.com/photo-1547592180-85f173990554?auto=format&fit=crop&w=800&q=80"),
        ("Hamburguesas", "Pan artesanal y carnes a la parrilla", "https://images.unsplash.com/photo-1568901346375-23c9450c58cd?auto=format&fit=crop&w=800&q=80"),
        ("Pizzas", "Pizzas horneadas al momento", "https://images.unsplash.com/photo-1574071318508-1cdbab80d002?auto=format&fit=crop&w=800&q=80"),
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
        ("Arepa de chocolo", "Arepa dulce con queso campesino y mantequilla", 12000.0, 20, "Entradas", "https://images.unsplash.com/photo-1601050690597-df0568f70950?auto=format&fit=crop&w=800&q=80"),
        ("Empanadas de la casa", "Tres empanadas de carne con ají de la casa", 11000.0, 24, "Entradas", "https://images.unsplash.com/photo-1628840042765-356cda07504e?auto=format&fit=crop&w=800&q=80"),
        ("Patacones con hogao", "Patacones crocantes, hogao y queso costeño", 14500.0, 18, "Entradas", "https://images.unsplash.com/photo-1547592180-85f173990554?auto=format&fit=crop&w=800&q=80"),
        ("Bandeja paisa", "Frijoles, arroz, carne, chicharrón, huevo y aguacate", 28000.0, 15, "Platos principales", "https://images.unsplash.com/photo-1547592180-85f173990554?auto=format&fit=crop&w=800&q=80"),
        ("Pollo a la parrilla", "Pechuga marinada, papas rústicas y ensalada fresca", 26000.0, 12, "Platos principales", "https://images.unsplash.com/photo-1532550907401-a500c9a57435?auto=format&fit=crop&w=800&q=80"),
        ("Hamburguesa clásica", "Carne Angus, queso, lechuga, tomate y salsa de la casa", 24000.0, 16, "Hamburguesas", "https://images.unsplash.com/photo-1568901346375-23c9450c58cd?auto=format&fit=crop&w=800&q=80"),
        ("Hamburguesa BBQ", "Carne Angus, tocino, cebolla caramelizada y BBQ ahumada", 29000.0, 14, "Hamburguesas", "https://images.unsplash.com/photo-1550547660-d9450f859349?auto=format&fit=crop&w=800&q=80"),
        ("Pizza margarita", "Mozzarella, tomate San Marzano, albahaca y aceite de oliva", 26000.0, 10, "Pizzas", "https://images.unsplash.com/photo-1574071318508-1cdbab80d002?auto=format&fit=crop&w=800&q=80"),
        ("Pizza pepperoni", "Mozzarella, pepperoni artesanal y orégano", 30000.0, 10, "Pizzas", "https://images.unsplash.com/photo-1628840042765-356cda07504e?auto=format&fit=crop&w=800&q=80"),
        ("Cheesecake de frutos rojos", "Cheesecake horneado con coulis de frutos rojos", 12500.0, 12, "Postres", "https://images.unsplash.com/photo-1565958011703-44f9829ba187?auto=format&fit=crop&w=800&q=80"),
        ("Limonada natural", "Limonada preparada al momento con hierbabuena", 7000.0, 30, "Bebidas", "https://images.unsplash.com/photo-1513558161293-cdaf765ed2fd?auto=format&fit=crop&w=800&q=80"),
        ("Limonada de maracuyá", "Maracuyá natural, limón y un toque de hierbabuena", 8500.0, 25, "Bebidas", "https://images.unsplash.com/photo-1544145945-f90425340c7e?auto=format&fit=crop&w=800&q=80"),
        ("Cold brew", "Café de origen extraído en frío durante 16 horas", 9000.0, 20, "Bebidas", "https://images.unsplash.com/photo-1517701604599-bb29b565090c?auto=format&fit=crop&w=800&q=80"),
        ("Capuccino de la casa", "Espresso doble, leche vaporizada y espuma cremosa", 9500.0, 20, "Bebidas", "https://images.unsplash.com/photo-1534778101976-62847782c213?auto=format&fit=crop&w=800&q=80"),
        ("Soda de frutos rojos", "Soda artesanal con frutos rojos, limón y romero", 10000.0, 18, "Bebidas", "https://images.unsplash.com/photo-1513558161293-cdaf765ed2fd?auto=format&fit=crop&w=800&q=80"),
    ];
    for (name, description, price, stock, category_name, image_url) in products {
        sqlx::query("INSERT INTO products (id, tenant_id, category_id, name, description, price, stock, image_url) SELECT $1, $2, id, $3, $4, $5, $6, $7 FROM categories WHERE tenant_id = $2 AND name = $8 AND NOT EXISTS (SELECT 1 FROM products WHERE tenant_id = $2 AND name = $3)")
            .bind(Uuid::new_v4()).bind(tenant_id).bind(name).bind(description).bind(price).bind(stock).bind(image_url).bind(category_name).execute(pool).await?;
    }
    Ok(())
}
