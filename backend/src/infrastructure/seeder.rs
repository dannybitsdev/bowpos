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

    // El SUPER_ADMIN no se asigna a ninguna sede (branch_ids vacío = control total sobre todas
    // las sedes de su tenant: cat\u00e1logo, \u00f3rdenes, reportes y gesti\u00f3n de usuarios).
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

    seed_tenant_full(pool, persisted_tenant_id).await?;

    let demo_tenant_id = ensure_tenant(pool, "Restaurante Demo", "restaurante-demo").await?;
    seed_tenant_full(pool, demo_tenant_id).await?;

    Ok(())
}

/// Aprovisiona un tenant completo: cat\u00e1logo (7 categor\u00edas, 50+ productos), 2 sedes, personal
/// operativo asignado a cada sede, modificadores/toppings y \u00f3rdenes demo en ambas sedes.
async fn seed_tenant_full(pool: &PgPool, tenant_id: Uuid) -> Result<(), anyhow::Error> {
    seed_catalog(pool, tenant_id).await?;
    seed_products(pool, tenant_id).await?;
    seed_order_catalog(pool, tenant_id).await?;

    let main_branch = ensure_location(pool, tenant_id, "Sede Principal", "Calle Principal 100", "Bogotá").await?;
    let north_branch = ensure_location(pool, tenant_id, "Sede Norte", "Carrera 45 # 102-30", "Bogotá").await?;
    assign_existing_order_locations(pool, tenant_id, main_branch).await?;

    let tenant_slug = sqlx::query_scalar::<_, String>("SELECT slug FROM tenants WHERE id = $1").bind(tenant_id).fetch_one(pool).await?;
    let manager_id = ensure_staff_user(pool, tenant_id, &format!("gerente.principal@{tenant_slug}.com"), "Gerente Sede Principal", "BRANCH_MANAGER").await?;
    let cashier_main_id = ensure_staff_user(pool, tenant_id, &format!("cajero.principal@{tenant_slug}.com"), "Cajero Sede Principal", "CAJERO").await?;
    let waiter_main_id = ensure_staff_user(pool, tenant_id, &format!("mesero.principal@{tenant_slug}.com"), "Mesero Sede Principal", "MESERO").await?;
    let manager_north_id = ensure_staff_user(pool, tenant_id, &format!("gerente.norte@{tenant_slug}.com"), "Gerente Sede Norte", "BRANCH_MANAGER").await?;
    let cashier_north_id = ensure_staff_user(pool, tenant_id, &format!("cajero.norte@{tenant_slug}.com"), "Cajero Sede Norte", "CAJERO").await?;

    assign_branch(pool, tenant_id, manager_id, main_branch, true).await?;
    assign_branch(pool, tenant_id, cashier_main_id, main_branch, true).await?;
    assign_branch(pool, tenant_id, waiter_main_id, main_branch, true).await?;
    assign_branch(pool, tenant_id, manager_north_id, north_branch, true).await?;
    assign_branch(pool, tenant_id, cashier_north_id, north_branch, true).await?;

    seed_demo_orders(pool, tenant_id, main_branch, cashier_main_id).await?;
    seed_demo_orders(pool, tenant_id, north_branch, cashier_north_id).await?;

    Ok(())
}

async fn ensure_staff_user(pool: &PgPool, tenant_id: Uuid, email: &str, name: &str, role: &str) -> Result<Uuid, anyhow::Error> {
    let hashed = PasswordHasher::default().hash("BowposStaff!2026")?;
    sqlx::query(
        r#"
        INSERT INTO users (id, tenant_id, location_id, name, email, password_hash, role)
        VALUES ($1, $2, NULL, $3, $4, $5, $6)
        ON CONFLICT (email) DO UPDATE SET
            tenant_id = EXCLUDED.tenant_id,
            name = EXCLUDED.name,
            password_hash = EXCLUDED.password_hash,
            role = EXCLUDED.role
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(tenant_id)
    .bind(name)
    .bind(email)
    .bind(hashed)
    .bind(role)
    .execute(pool)
    .await?;

    Ok(sqlx::query_scalar("SELECT id FROM users WHERE email = $1").bind(email).fetch_one(pool).await?)
}

async fn assign_branch(pool: &PgPool, tenant_id: Uuid, user_id: Uuid, location_id: Uuid, is_primary: bool) -> Result<(), anyhow::Error> {
    sqlx::query(
        "INSERT INTO user_branch_access (tenant_id, user_id, location_id, is_primary) VALUES ($1, $2, $3, $4)
         ON CONFLICT (tenant_id, user_id, location_id) DO UPDATE SET is_primary = EXCLUDED.is_primary",
    )
    .bind(tenant_id).bind(user_id).bind(location_id).bind(is_primary)
    .execute(pool)
    .await?;
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
    for product_name in ["Hamburguesa clásica", "Hamburguesa BBQ", "Hamburguesa doble queso", "Hamburguesa vegetariana", "Hamburguesa hawaiana", "Hamburguesa picante"] {
        associate_modifier_group(pool, tenant_id, product_name, meat_group).await?;
        associate_toppings(pool, tenant_id, product_name, &topping_ids[..3]).await?;
    }
    for product_name in ["Pizza margarita", "Pizza pepperoni", "Pizza hawaiana", "Pizza cuatro quesos", "Pizza vegetariana", "Pizza BBQ de pollo"] {
        associate_modifier_group(pool, tenant_id, product_name, crust_group).await?;
        associate_toppings(pool, tenant_id, product_name, &topping_ids[0..4]).await?;
    }
    for product_name in ["Limonada natural", "Limonada de maracuyá", "Cold brew", "Jugo de mango", "Té helado", "Malteada de vainilla"] {
        associate_modifier_group(pool, tenant_id, product_name, size_group).await?;
    }
    associate_modifier_group(pool, tenant_id, "Capuccino de la casa", size_group).await?;
    associate_modifier_group(pool, tenant_id, "Capuccino de la casa", milk_group).await?;
    associate_modifier_group(pool, tenant_id, "Chocolate caliente", milk_group).await?;
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

async fn seed_demo_orders(pool: &PgPool, tenant_id: Uuid, location_id: Uuid, user_id: Uuid) -> Result<(), anyhow::Error> {
    let product_id = sqlx::query_scalar::<_, Uuid>("SELECT id FROM products WHERE tenant_id = $1 ORDER BY name LIMIT 1").bind(tenant_id).fetch_optional(pool).await?;
    let Some(product_id) = product_id else { return Ok(()); };
    let location_tag = sqlx::query_scalar::<_, String>("SELECT name FROM locations WHERE id = $1").bind(location_id).fetch_one(pool).await?;
    for status in ["IN_PREPARATION", "DELIVERED", "CANCELLED"] {
        let notes = format!("Demo {location_tag} {status}");
        let exists = sqlx::query_scalar::<_, bool>("SELECT EXISTS (SELECT 1 FROM orders WHERE tenant_id = $1 AND notes = $2)").bind(tenant_id).bind(&notes).fetch_one(pool).await?;
        if exists { continue; }
        let order_id = Uuid::new_v4();
        let item_id = Uuid::new_v4();
        let order_number: i64 = sqlx::query_scalar("SELECT COALESCE(MAX(order_number), 0) + 1 FROM orders WHERE tenant_id = $1").bind(tenant_id).fetch_one(pool).await?;
        sqlx::query("INSERT INTO orders (id, order_number, tenant_id, location_id, user_id, service_type, table_name, customer_name, notes, status, subtotal, tax, total) VALUES ($1,$2,$3,$4,$5,'DINE_IN','Mesa 4','Cliente demo',$6,$7,28000,5320,33320)")
            .bind(order_id).bind(order_number).bind(tenant_id).bind(location_id).bind(user_id).bind(&notes).bind(status).execute(pool).await?;
        sqlx::query("INSERT INTO order_items (id, tenant_id, order_id, product_id, product_name, quantity, unit_price, subtotal) SELECT $1,$2,$3,id,name,1,price,price FROM products WHERE id = $4 AND tenant_id = $2").bind(item_id).bind(tenant_id).bind(order_id).bind(product_id).execute(pool).await?;
    }
    Ok(())
}

async fn ensure_tenant(pool: &PgPool, name: &str, slug: &str) -> Result<Uuid, anyhow::Error> {
    sqlx::query("INSERT INTO tenants (id, name, slug, created_at) VALUES ($1, $2, $3, NOW()) ON CONFLICT (slug) DO NOTHING")
        .bind(Uuid::new_v4()).bind(name).bind(slug).execute(pool).await?;
    Ok(sqlx::query_scalar("SELECT id FROM tenants WHERE slug = $1").bind(slug).fetch_one(pool).await?)
}

async fn ensure_location(pool: &PgPool, tenant_id: Uuid, name: &str, address: &str, city: &str) -> Result<Uuid, anyhow::Error> {
    sqlx::query("INSERT INTO locations (id, tenant_id, name, address, city) SELECT $1, $2, $3, $4, $5 WHERE NOT EXISTS (SELECT 1 FROM locations WHERE tenant_id = $2 AND name = $3)")
        .bind(Uuid::new_v4()).bind(tenant_id).bind(name).bind(address).bind(city).execute(pool).await?;
    Ok(sqlx::query_scalar("SELECT id FROM locations WHERE tenant_id = $1 AND name = $2").bind(tenant_id).bind(name).fetch_one(pool).await?)
}

async fn assign_existing_order_locations(pool: &PgPool, tenant_id: Uuid, default_location_id: Uuid) -> Result<(), anyhow::Error> {
    sqlx::query("UPDATE orders SET location_id = $2 WHERE tenant_id = $1 AND location_id IS NULL")
        .bind(tenant_id).bind(default_location_id).execute(pool).await?;
    Ok(())
}

async fn seed_catalog(pool: &PgPool, tenant_id: Uuid) -> Result<(), anyhow::Error> {
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
    const FOOD_IMG: &str = "https://images.unsplash.com/photo-1546069901-ba9599a7e63c?auto=format&fit=crop&w=800&q=80";
    const BURGER_IMG: &str = "https://images.unsplash.com/photo-1568901346375-23c9450c58cd?auto=format&fit=crop&w=800&q=80";
    const PIZZA_IMG: &str = "https://images.unsplash.com/photo-1574071318508-1cdbab80d002?auto=format&fit=crop&w=800&q=80";
    const DRINK_IMG: &str = "https://images.unsplash.com/photo-1513558161293-cdaf765ed2fd?auto=format&fit=crop&w=800&q=80";
    const DESSERT_IMG: &str = "https://images.unsplash.com/photo-1551024506-0bccd828d307?auto=format&fit=crop&w=800&q=80";
    const PROMO_IMG: &str = "https://images.unsplash.com/photo-1550547660-d9450f859349?auto=format&fit=crop&w=800&q=80";

    let products = [
        // Entradas (8)
        ("Arepa de chocolo", "Arepa dulce con queso campesino y mantequilla", 12000.0, 20, "Entradas", FOOD_IMG),
        ("Empanadas de la casa", "Tres empanadas de carne con ají de la casa", 11000.0, 24, "Entradas", FOOD_IMG),
        ("Patacones con hogao", "Patacones crocantes, hogao y queso costeño", 14500.0, 18, "Entradas", FOOD_IMG),
        ("Ceviche de camarón", "Camarones marinados en limón con leche de tigre", 22000.0, 15, "Entradas", FOOD_IMG),
        ("Croquetas de queso", "Croquetas artesanales con costra crocante", 13000.0, 20, "Entradas", FOOD_IMG),
        ("Alitas BBQ", "Alitas de pollo bañadas en salsa BBQ ahumada", 18000.0, 20, "Entradas", FOOD_IMG),
        ("Tequeños venezolanos", "Palitos de queso envueltos en masa crocante", 15000.0, 22, "Entradas", FOOD_IMG),
        ("Ensalada César", "Lechuga romana, pollo, crotones y aderezo césar", 16000.0, 18, "Entradas", FOOD_IMG),
        // Platos principales (8)
        ("Bandeja paisa", "Frijoles, arroz, carne, chicharrón, huevo y aguacate", 28000.0, 15, "Platos principales", FOOD_IMG),
        ("Pollo a la parrilla", "Pechuga marinada, papas rústicas y ensalada fresca", 26000.0, 12, "Platos principales", FOOD_IMG),
        ("Lomo al vino tinto", "Lomo de res reducido en vino tinto con puré", 34000.0, 10, "Platos principales", FOOD_IMG),
        ("Trucha a la plancha", "Trucha fresca con patacón y ensalada", 29000.0, 10, "Platos principales", FOOD_IMG),
        ("Costillas BBQ", "Costillas de cerdo glaseadas en BBQ de la casa", 32000.0, 10, "Platos principales", FOOD_IMG),
        ("Arroz con pollo", "Arroz meloso con pollo desmechado y verduras", 21000.0, 16, "Platos principales", FOOD_IMG),
        ("Pasta alfredo", "Pasta fresca en salsa alfredo con pollo", 24000.0, 14, "Platos principales", FOOD_IMG),
        ("Salmón a la parrilla", "Salmón con costra de hierbas y vegetales asados", 36000.0, 8, "Platos principales", FOOD_IMG),
        // Hamburguesas (6)
        ("Hamburguesa clásica", "Carne Angus, queso, lechuga, tomate y salsa de la casa", 24000.0, 16, "Hamburguesas", BURGER_IMG),
        ("Hamburguesa BBQ", "Carne Angus, tocino, cebolla caramelizada y BBQ ahumada", 29000.0, 14, "Hamburguesas", BURGER_IMG),
        ("Hamburguesa doble queso", "Doble carne, doble queso cheddar fundido", 31000.0, 12, "Hamburguesas", BURGER_IMG),
        ("Hamburguesa vegetariana", "Base de garbanzo y vegetales asados", 23000.0, 12, "Hamburguesas", BURGER_IMG),
        ("Hamburguesa hawaiana", "Carne, piña asada, tocino y salsa teriyaki", 27000.0, 12, "Hamburguesas", BURGER_IMG),
        ("Hamburguesa picante", "Carne, jalapeños, pepper jack y salsa picante", 28000.0, 12, "Hamburguesas", BURGER_IMG),
        // Pizzas (6)
        ("Pizza margarita", "Mozzarella, tomate San Marzano, albahaca y aceite de oliva", 26000.0, 10, "Pizzas", PIZZA_IMG),
        ("Pizza pepperoni", "Mozzarella, pepperoni artesanal y orégano", 30000.0, 10, "Pizzas", PIZZA_IMG),
        ("Pizza hawaiana", "Jamón, piña y mozzarella", 28000.0, 10, "Pizzas", PIZZA_IMG),
        ("Pizza cuatro quesos", "Mozzarella, parmesano, gorgonzola y provolone", 32000.0, 8, "Pizzas", PIZZA_IMG),
        ("Pizza vegetariana", "Vegetales asados de temporada y mozzarella", 27000.0, 10, "Pizzas", PIZZA_IMG),
        ("Pizza BBQ de pollo", "Pollo, cebolla morada y salsa BBQ", 31000.0, 10, "Pizzas", PIZZA_IMG),
        // Bebidas (10)
        ("Limonada natural", "Limonada preparada al momento con hierbabuena", 7000.0, 30, "Bebidas", DRINK_IMG),
        ("Limonada de maracuyá", "Maracuyá natural, limón y un toque de hierbabuena", 8500.0, 25, "Bebidas", DRINK_IMG),
        ("Cold brew", "Café de origen extraído en frío durante 16 horas", 9000.0, 20, "Bebidas", DRINK_IMG),
        ("Capuccino de la casa", "Espresso doble, leche vaporizada y espuma cremosa", 9500.0, 20, "Bebidas", DRINK_IMG),
        ("Soda de frutos rojos", "Soda artesanal con frutos rojos, limón y romero", 10000.0, 18, "Bebidas", DRINK_IMG),
        ("Jugo de mango", "Jugo natural de mango con agua o leche", 8000.0, 22, "Bebidas", DRINK_IMG),
        ("Té helado", "Té negro helado con limón y menta", 7500.0, 24, "Bebidas", DRINK_IMG),
        ("Chocolate caliente", "Chocolate espeso con canela y queso", 8500.0, 18, "Bebidas", DRINK_IMG),
        ("Malteada de vainilla", "Malteada cremosa de vainilla con crema batida", 11000.0, 16, "Bebidas", DRINK_IMG),
        ("Agua con gas", "Agua mineral con gas 500ml", 4500.0, 40, "Bebidas", DRINK_IMG),
        // Postres (7)
        ("Cheesecake de frutos rojos", "Cheesecake horneado con coulis de frutos rojos", 12500.0, 12, "Postres", DESSERT_IMG),
        ("Brownie con helado", "Brownie tibio con helado de vainilla", 13000.0, 14, "Postres", DESSERT_IMG),
        ("Flan de caramelo", "Flan casero bañado en caramelo", 9500.0, 16, "Postres", DESSERT_IMG),
        ("Tiramisú", "Clásico tiramisú italiano con café y cacao", 14000.0, 12, "Postres", DESSERT_IMG),
        ("Torta de chocolate", "Porción de torta húmeda de chocolate", 12000.0, 14, "Postres", DESSERT_IMG),
        ("Postre de tres leches", "Bizcocho bañado en tres leches con canela", 11500.0, 14, "Postres", DESSERT_IMG),
        ("Helado artesanal", "Dos bolas de helado artesanal a elección", 9000.0, 20, "Postres", DESSERT_IMG),
        // Promociones (7)
        ("Combo hamburguesa + bebida", "Hamburguesa clásica con bebida incluida", 28000.0, 20, "Promociones", PROMO_IMG),
        ("Combo pizza familiar", "Pizza grande a elección más gaseosa 1.5L", 42000.0, 12, "Promociones", PROMO_IMG),
        ("Combo desayuno ejecutivo", "Huevos, arepa, café y jugo natural", 16000.0, 18, "Promociones", PROMO_IMG),
        ("Combo 2x1 entradas", "Dos entradas a elección por el precio de una", 18000.0, 16, "Promociones", PROMO_IMG),
        ("Combo infantil", "Mini hamburguesa, papas y jugo", 15000.0, 20, "Promociones", PROMO_IMG),
        ("Combo pareja", "Dos platos principales y una botella de vino de la casa", 68000.0, 8, "Promociones", PROMO_IMG),
        ("Combo oficina", "Cinco almuerzos ejecutivos para el equipo", 95000.0, 6, "Promociones", PROMO_IMG),
    ];

    for (name, description, price, stock, category_name, image_url) in products {
        sqlx::query("INSERT INTO products (id, tenant_id, category_id, name, description, price, stock, image_url) SELECT $1, $2, id, $3, $4, $5, $6, $7 FROM categories WHERE tenant_id = $2 AND name = $8 AND NOT EXISTS (SELECT 1 FROM products WHERE tenant_id = $2 AND name = $3)")
            .bind(Uuid::new_v4()).bind(tenant_id).bind(name).bind(description).bind(price).bind(stock).bind(image_url).bind(category_name).execute(pool).await?;
    }
    Ok(())
}

