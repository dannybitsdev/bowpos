use std::sync::Arc;

use chrono::Utc;
use uuid::Uuid;

use crate::{
    application::auth::{
        commands::{
            AuthTokens, AuthUserView, CreateTenantAdminCommand, CreateTenantUserCommand,
            LoginCommand, LoginResult, RefreshTokenCommand, RegisterSuperAdminCommand,
        },
        errors::AppError,
    },
    domain::{
        auth::{Permission, Role, User},
        repositories::UserRepository,
        value_objects::{email::Email, password_hash::PasswordHash},
    },
    infrastructure::services::{
        jwt_service::{JwtClaims, JwtService},
        password_hasher::PasswordHasher,
        refresh_token_service::RefreshTokenService,
    },
};

pub struct AuthUseCases {
    repository: Arc<dyn UserRepository>,
    hasher: PasswordHasher,
    jwt: JwtService,
    refresh_tokens: RefreshTokenService,
    max_login_attempts: i32,
    lock_minutes: i32,
}

impl AuthUseCases {
    pub fn new(
        repository: Arc<dyn UserRepository>,
        hasher: PasswordHasher,
        jwt: JwtService,
        refresh_tokens: RefreshTokenService,
        max_login_attempts: i32,
        lock_minutes: i32,
    ) -> Self {
        Self {
            repository,
            hasher,
            jwt,
            refresh_tokens,
            max_login_attempts,
            lock_minutes,
        }
    }

    pub async fn login(&self, command: LoginCommand) -> Result<LoginResult, AppError> {
        let email = Email::parse(&command.email).map_err(|_| AppError::InvalidCredentials)?;

        if self.is_account_locked(email.as_str()).await? {
            return Err(AppError::Unauthorized);
        }

        let user = self
            .repository
            .find_by_email(email.as_str())
            .await
            .map_err(|_| AppError::Internal)?
            .ok_or(AppError::InvalidCredentials);

        let user = match user {
            Ok(value) => value,
            Err(error) => {
                self.record_failed_attempt(email.as_str()).await?;
                return Err(error);
            }
        };

        if let Some(tenant_id) = command.tenant_id {
            if user.tenant_id != tenant_id {
                self.record_failed_attempt(email.as_str()).await?;
                return Err(AppError::InvalidCredentials);
            }
        }

        if self
            .hasher
            .verify(&command.password, user.password_hash.as_str())
            .is_err()
        {
            self.record_failed_attempt(email.as_str()).await?;
            return Err(AppError::InvalidCredentials);
        }

        self.repository
            .reset_login_failures(email.as_str())
            .await
            .map_err(|_| AppError::Internal)?;

        let tokens = self.issue_tokens_for_user(&user).await?;

        Ok(LoginResult {
            tokens,
            user: to_view(user),
        })
    }

    pub async fn refresh_session(
        &self,
        command: RefreshTokenCommand,
    ) -> Result<LoginResult, AppError> {
        if command.refresh_token.trim().is_empty() {
            return Err(AppError::Unauthorized);
        }

        let now = Utc::now();
        let current_hash = self.refresh_tokens.hash_token(&command.refresh_token);
        let current = self
            .repository
            .get_valid_refresh_token(&current_hash, now)
            .await
            .map_err(|_| AppError::Internal)?
            .ok_or(AppError::Unauthorized)?;

        let user = self
            .repository
            .find_by_id(current.user_id)
            .await
            .map_err(|_| AppError::Internal)?
            .ok_or(AppError::Unauthorized)?;

        let tokens = self.issue_tokens_for_user(&user).await?;
        let new_hash = self.refresh_tokens.hash_token(&tokens.refresh_token);
        let replacement = self
            .repository
            .get_valid_refresh_token(&new_hash, now)
            .await
            .map_err(|_| AppError::Internal)?
            .ok_or(AppError::Internal)?;

        self.repository
            .revoke_refresh_token(current.id, Some(replacement.id))
            .await
            .map_err(|_| AppError::Internal)?;

        Ok(LoginResult {
            tokens,
            user: to_view(user),
        })
    }

    async fn issue_tokens_for_user(&self, user: &User) -> Result<AuthTokens, AppError> {
        let claims = JwtClaims::from_user(user);
        let access_token = self
            .jwt
            .issue_access_token(&claims)
            .map_err(|_| AppError::Internal)?;

        let refresh_raw = self.refresh_tokens.generate_raw_token();
        let refresh_hash = self.refresh_tokens.hash_token(&refresh_raw);
        let refresh_id = Uuid::new_v4();
        let refresh_expiration = self.refresh_tokens.expires_at();

        self.repository
            .persist_refresh_token(
                refresh_id,
                user.id,
                user.tenant_id,
                &refresh_hash,
                refresh_expiration,
            )
            .await
            .map_err(|_| AppError::Internal)?;

        Ok(AuthTokens {
            access_token,
            refresh_token: refresh_raw,
            token_type: "Bearer".to_string(),
            expires_in: self.jwt.ttl_seconds(),
            refresh_expires_in: self.refresh_tokens.ttl_seconds() as u64,
        })
    }

    async fn is_account_locked(&self, email: &str) -> Result<bool, AppError> {
        let state = self
            .repository
            .get_login_attempt_state(email)
            .await
            .map_err(|_| AppError::Internal)?;

        if let Some(lock_state) = state {
            if let Some(locked_until) = lock_state.locked_until {
                return Ok(locked_until > Utc::now());
            }
        }

        Ok(false)
    }

    async fn record_failed_attempt(&self, email: &str) -> Result<(), AppError> {
        self.repository
            .register_login_failure(email, self.max_login_attempts, self.lock_minutes)
            .await
            .map_err(|_| AppError::Internal)?;

        Ok(())
    }

    pub async fn register_super_admin(
        &self,
        command: RegisterSuperAdminCommand,
    ) -> Result<AuthUserView, AppError> {
        PasswordHash::validate_plaintext_rules(&command.password)
            .map_err(|_| AppError::Validation("password policy not satisfied".to_string()))?;

        let total = self
            .repository
            .total_users()
            .await
            .map_err(|_| AppError::Internal)?;

        if total > 0 {
            return Err(AppError::Forbidden);
        }

        let email = Email::parse(&command.email)
            .map_err(|_| AppError::Validation("invalid email".to_string()))?;

        let password_hash = self
            .hasher
            .hash(&command.password)
            .map_err(|_| AppError::Internal)?;

        let tenant = self
            .repository
            .create_tenant(&command.tenant_name, &command.tenant_slug)
            .await
            .map_err(|_| AppError::Internal)?;

        let created = self
            .repository
            .create_user(
                tenant.id,
                &command.full_name,
                email.as_str(),
                &password_hash,
                Role::SUPER_ADMIN,
            )
            .await
            .map_err(|_| AppError::Internal)?;

        Ok(to_view(created))
    }

    pub async fn create_tenant_admin(
        &self,
        actor: &User,
        command: CreateTenantAdminCommand,
    ) -> Result<AuthUserView, AppError> {
        if !actor.can_create_role(Role::ADMIN_TENANT, command.tenant_id) {
            return Err(AppError::Forbidden);
        }

        let target_tenant = self
            .repository
            .get_tenant(command.tenant_id)
            .await
            .map_err(|_| AppError::Internal)?;

        if target_tenant.is_none() {
            return Err(AppError::NotFound);
        }

        let email = Email::parse(&command.email)
            .map_err(|_| AppError::Validation("invalid email".to_string()))?;
        PasswordHash::validate_plaintext_rules(&command.password)
            .map_err(|_| AppError::Validation("password policy not satisfied".to_string()))?;

        let password_hash = self
            .hasher
            .hash(&command.password)
            .map_err(|_| AppError::Internal)?;

        let created = self
            .repository
            .create_user(
                command.tenant_id,
                &command.full_name,
                email.as_str(),
                &password_hash,
                Role::ADMIN_TENANT,
            )
            .await
            .map_err(|_| AppError::Internal)?;

        Ok(to_view(created))
    }

    pub async fn create_tenant_user(
        &self,
        actor: &User,
        command: CreateTenantUserCommand,
    ) -> Result<AuthUserView, AppError> {
        if !matches!(command.role, Role::CAJERO | Role::MESERO) {
            return Err(AppError::Validation(
                "tenant admins only can create CAJERO or MESERO".to_string(),
            ));
        }

        if !actor.can_create_role(command.role, command.tenant_id) {
            return Err(AppError::Forbidden);
        }

        let email = Email::parse(&command.email)
            .map_err(|_| AppError::Validation("invalid email".to_string()))?;
        PasswordHash::validate_plaintext_rules(&command.password)
            .map_err(|_| AppError::Validation("password policy not satisfied".to_string()))?;

        let password_hash = self
            .hasher
            .hash(&command.password)
            .map_err(|_| AppError::Internal)?;

        let created = self
            .repository
            .create_user(
                command.tenant_id,
                &command.full_name,
                email.as_str(),
                &password_hash,
                command.role,
            )
            .await
            .map_err(|_| AppError::Internal)?;

        Ok(to_view(created))
    }

    pub fn claims_to_user(&self, claims: &JwtClaims) -> Result<User, AppError> {
        let role = Role::from_db(&claims.role).ok_or(AppError::Unauthorized)?;
        let email = Email::parse(&claims.email).map_err(|_| AppError::Unauthorized)?;
        let password_hash = PasswordHash::new("$argon2id$dummy".to_string())
            .map_err(|_| AppError::Unauthorized)?;

        Ok(User {
            id: claims.user_id,
            tenant_id: claims.tenant_id,
            name: claims.name.clone(),
            email,
            password_hash,
            role,
        })
    }
}

fn to_view(user: User) -> AuthUserView {
    AuthUserView {
        user_id: user.id,
        tenant_id: user.tenant_id,
        role: user.role,
        permissions: user
            .role
            .permissions()
            .into_iter()
            .map(Permission::as_str)
            .map(str::to_string)
            .collect(),
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use async_trait::async_trait;
    use chrono::{Duration, Utc};
    use uuid::Uuid;

    use crate::{
        application::auth::{commands::LoginCommand, errors::AppError, use_cases::AuthUseCases},
        domain::{
            auth::{Role, Tenant, User},
            repositories::{LoginAttemptState, RefreshTokenRecord, UserRepository},
            value_objects::{email::Email, password_hash::PasswordHash},
        },
        infrastructure::services::{
            jwt_service::JwtService,
            password_hasher::PasswordHasher,
            refresh_token_service::RefreshTokenService,
        },
    };

    struct MockRepo {
        user: Option<User>,
    }

    #[async_trait]
    impl UserRepository for MockRepo {
        async fn total_users(&self) -> Result<i64, anyhow::Error> {
            Ok(1)
        }

        async fn find_by_email(&self, _email: &str) -> Result<Option<User>, anyhow::Error> {
            Ok(self.user.clone())
        }

        async fn find_by_id(&self, user_id: Uuid) -> Result<Option<User>, anyhow::Error> {
            Ok(self.user.clone().filter(|value| value.id == user_id))
        }

        async fn create_tenant(&self, _name: &str, _slug: &str) -> Result<Tenant, anyhow::Error> {
            unreachable!()
        }

        async fn get_tenant(&self, _tenant_id: Uuid) -> Result<Option<Tenant>, anyhow::Error> {
            unreachable!()
        }

        async fn create_user(
            &self,
            _tenant_id: Uuid,
            _name: &str,
            _email: &str,
            _password_hash: &str,
            _role: Role,
        ) -> Result<User, anyhow::Error> {
            unreachable!()
        }

        async fn persist_refresh_token(
            &self,
            _id: Uuid,
            _user_id: Uuid,
            _tenant_id: Uuid,
            _token_hash: &str,
            _expires_at: chrono::DateTime<Utc>,
        ) -> Result<(), anyhow::Error> {
            Ok(())
        }

        async fn get_valid_refresh_token(
            &self,
            _token_hash: &str,
            _now: chrono::DateTime<Utc>,
        ) -> Result<Option<RefreshTokenRecord>, anyhow::Error> {
            Ok(Some(RefreshTokenRecord {
                id: Uuid::new_v4(),
                user_id: self.user.as_ref().map(|value| value.id).unwrap_or_else(Uuid::new_v4),
                tenant_id: self.user.as_ref().map(|value| value.tenant_id).unwrap_or_else(Uuid::new_v4),
                expires_at: Utc::now() + Duration::minutes(10),
            }))
        }

        async fn revoke_refresh_token(
            &self,
            _token_id: Uuid,
            _replaced_by: Option<Uuid>,
        ) -> Result<(), anyhow::Error> {
            Ok(())
        }

        async fn get_login_attempt_state(
            &self,
            _email: &str,
        ) -> Result<Option<LoginAttemptState>, anyhow::Error> {
            Ok(None)
        }

        async fn register_login_failure(
            &self,
            _email: &str,
            _max_attempts: i32,
            _lock_minutes: i32,
        ) -> Result<LoginAttemptState, anyhow::Error> {
            Ok(LoginAttemptState {
                failed_attempts: 1,
                locked_until: None,
            })
        }

        async fn reset_login_failures(&self, _email: &str) -> Result<(), anyhow::Error> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn login_fails_with_invalid_password() {
        let tenant_id = Uuid::new_v4();
        let user = User {
            id: Uuid::new_v4(),
            tenant_id,
            name: "Test".to_string(),
            email: Email::parse("test@example.com").expect("valid email"),
            password_hash: PasswordHash::new(
                PasswordHasher::default().hash("StrongP@ssw0rd").expect("hash"),
            )
            .expect("password hash"),
            role: Role::SUPER_ADMIN,
        };

        let repo = Arc::new(MockRepo { user: Some(user) });
        let use_case = AuthUseCases::new(
            repo,
            PasswordHasher::default(),
            JwtService::new("secret", 3600),
            RefreshTokenService::new("refresh-secret".to_string(), 60 * 60 * 24),
            5,
            15,
        );

        let result = use_case
            .login(LoginCommand {
                email: "test@example.com".to_string(),
                password: "wrong-password".to_string(),
                tenant_id: Some(tenant_id),
            })
            .await;

        assert!(matches!(result, Err(AppError::InvalidCredentials)));
    }
}
