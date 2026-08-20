use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::domain::auth::{Role, Tenant, User};

#[derive(Debug, Clone)]
pub struct RefreshTokenRecord {
    pub id: Uuid,
    pub user_id: Uuid,
    pub tenant_id: Uuid,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct LoginAttemptState {
    pub failed_attempts: i32,
    pub locked_until: Option<DateTime<Utc>>,
}

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn total_users(&self) -> Result<i64, anyhow::Error>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, anyhow::Error>;
    async fn find_by_id(&self, user_id: Uuid) -> Result<Option<User>, anyhow::Error>;
    async fn create_tenant(&self, name: &str, slug: &str) -> Result<Tenant, anyhow::Error>;
    async fn get_tenant(&self, tenant_id: Uuid) -> Result<Option<Tenant>, anyhow::Error>;
    async fn create_user(
        &self,
        tenant_id: Uuid,
        name: &str,
        email: &str,
        password_hash: &str,
        role: Role,
    ) -> Result<User, anyhow::Error>;
    async fn persist_refresh_token(
        &self,
        id: Uuid,
        user_id: Uuid,
        tenant_id: Uuid,
        token_hash: &str,
        expires_at: DateTime<Utc>,
    ) -> Result<(), anyhow::Error>;
    async fn get_valid_refresh_token(
        &self,
        token_hash: &str,
        now: DateTime<Utc>,
    ) -> Result<Option<RefreshTokenRecord>, anyhow::Error>;
    async fn revoke_refresh_token(
        &self,
        token_id: Uuid,
        replaced_by: Option<Uuid>,
    ) -> Result<(), anyhow::Error>;
    async fn get_login_attempt_state(
        &self,
        email: &str,
    ) -> Result<Option<LoginAttemptState>, anyhow::Error>;
    async fn register_login_failure(
        &self,
        email: &str,
        max_attempts: i32,
        lock_minutes: i32,
    ) -> Result<LoginAttemptState, anyhow::Error>;
    async fn reset_login_failures(&self, email: &str) -> Result<(), anyhow::Error>;
    async fn assign_branch(
        &self,
        tenant_id: Uuid,
        user_id: Uuid,
        location_id: Uuid,
        is_primary: bool,
    ) -> Result<(), anyhow::Error>;
}
