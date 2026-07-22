use sqlx::PgPool;
use uuid::Uuid;

use crate::infrastructure::services::password_hasher::PasswordHasher;

pub async fn seed_initial_super_admin(pool: &PgPool) -> Result<(), anyhow::Error> {
    let users_count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM usuarios")
        .fetch_one(pool)
        .await?;

    if users_count > 0 {
        return Ok(());
    }

    let tenant_id = Uuid::new_v4();
    let hashed = PasswordHasher::default().hash("ChangeMe!12345")?;

    sqlx::query(
        r#"
        INSERT INTO tenants (id, name, slug, created_at)
        VALUES ($1, $2, $3, NOW())
        ON CONFLICT (slug) DO NOTHING
        "#,
    )
    .bind(tenant_id)
    .bind("Tenant Global")
    .bind("tenant-global")
    .execute(pool)
    .await?;

    let persisted_tenant_id = sqlx::query_scalar::<_, Uuid>(
        "SELECT id FROM tenants WHERE slug = 'tenant-global' LIMIT 1",
    )
    .fetch_one(pool)
    .await?;

    sqlx::query(
        r#"
        INSERT INTO usuarios (id, tenant_id, sede_id, nombre, email, password_hash, rol)
        VALUES ($1, $2, NULL, $3, $4, $5, 'SUPER_ADMIN')
        ON CONFLICT (email) DO NOTHING
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(persisted_tenant_id)
    .bind("Global Super Admin")
    .bind("superadmin@bowpos.local")
    .bind(hashed)
    .execute(pool)
    .await?;

    Ok(())
}
