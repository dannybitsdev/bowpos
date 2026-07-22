use async_trait::async_trait;
use chrono::{DateTime, Duration, Utc};
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::domain::{
    auth::{Role, Tenant, User},
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
}

#[async_trait]
impl UserRepository for SqlxUserRepository {
    async fn total_users(&self) -> Result<i64, anyhow::Error> {
        let count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM usuarios")
            .fetch_one(&self.pool)
            .await?;
        Ok(count)
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, anyhow::Error> {
        let row = sqlx::query(
            r#"
            SELECT id, tenant_id, nombre, email, password_hash, rol
            FROM usuarios
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

        let role = Role::from_db(record.try_get::<&str, _>("rol")?)
            .ok_or_else(|| anyhow::anyhow!("invalid role in database"))?;
        let email = Email::parse(record.try_get::<&str, _>("email")?)
            .map_err(|_| anyhow::anyhow!("invalid email in database"))?;
        let password_hash = PasswordHash::new(record.try_get::<String, _>("password_hash")?)
            .map_err(|_| anyhow::anyhow!("invalid hash in database"))?;

        Ok(Some(User {
            id: record.try_get("id")?,
            tenant_id: record.try_get("tenant_id")?,
            name: record.try_get("nombre")?,
            email,
            password_hash,
            role,
        }))
    }

    async fn find_by_id(&self, user_id: Uuid) -> Result<Option<User>, anyhow::Error> {
        let row = sqlx::query(
            r#"
            SELECT id, tenant_id, nombre, email, password_hash, rol
            FROM usuarios
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

        let role = Role::from_db(record.try_get::<&str, _>("rol")?)
            .ok_or_else(|| anyhow::anyhow!("invalid role in database"))?;
        let email = Email::parse(record.try_get::<&str, _>("email")?)
            .map_err(|_| anyhow::anyhow!("invalid email in database"))?;
        let password_hash = PasswordHash::new(record.try_get::<String, _>("password_hash")?)
            .map_err(|_| anyhow::anyhow!("invalid hash in database"))?;

        Ok(Some(User {
            id: record.try_get("id")?,
            tenant_id: record.try_get("tenant_id")?,
            name: record.try_get("nombre")?,
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
            INSERT INTO usuarios (id, tenant_id, sede_id, nombre, email, password_hash, rol)
            VALUES ($1, $2, NULL, $3, $4, $5, $6)
            RETURNING id, tenant_id, nombre, email, password_hash, rol
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

        Ok(User {
            id: row.try_get("id")?,
            tenant_id: row.try_get("tenant_id")?,
            name: row.try_get("nombre")?,
            email: Email::parse(row.try_get::<&str, _>("email")?)
                .map_err(|_| anyhow::anyhow!("invalid email"))?,
            password_hash: PasswordHash::new(row.try_get::<String, _>("password_hash")?)
                .map_err(|_| anyhow::anyhow!("invalid hash"))?,
            role: Role::from_db(row.try_get::<&str, _>("rol")?)
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
