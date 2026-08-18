use std::marker::PhantomData;

use axum::{
    extract::FromRequestParts,
    http::{header::AUTHORIZATION, request::Parts, StatusCode},
};
use async_trait::async_trait;

use crate::{
    domain::auth::User,
    infrastructure::services::jwt_service::JwtService,
    AppState,
};

use super::policy::{AccessPolicy, DenyByDefault};

#[derive(Clone)]
pub struct AuthUser<P = DenyByDefault>
where
    P: AccessPolicy,
{
    pub user: User,
    _policy: PhantomData<P>,
}

impl<P> AuthUser<P>
where
    P: AccessPolicy,
{
    pub fn into_inner(self) -> User {
        self.user
    }
}

#[async_trait]
impl<S, P> FromRequestParts<S> for AuthUser<P>
where
    S: Send + Sync,
    AppState: FromRef<S>,
    P: AccessPolicy + Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let app_state = AppState::from_ref(state);
        let auth_header = match parts
            .headers
            .get(AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
        {
            Some(value) => value,
            None => return Err((StatusCode::UNAUTHORIZED, "missing authorization header")),
        };

        let token = match auth_header.strip_prefix("Bearer ") {
            Some(value) => value,
            None => return Err((StatusCode::UNAUTHORIZED, "invalid authorization scheme")),
        };

        let claims = match app_state.jwt_service.validate_access_token(token) {
            Ok(value) => value,
            Err(_) => return Err((StatusCode::UNAUTHORIZED, "invalid or expired token")),
        };

        let no_role_guard = P::required_roles().is_empty();
        let no_permission_guard = P::required_permissions().is_empty();
        if no_role_guard && no_permission_guard {
            return Err((
                StatusCode::FORBIDDEN,
                "endpoint guard is required and not configured",
            ));
        }

        let has_roles = JwtService::has_role(&claims, P::required_roles());
        let has_permissions = JwtService::has_permissions(&claims, P::required_permissions());

        if !has_roles || !has_permissions {
            return Err((StatusCode::FORBIDDEN, "insufficient privileges"));
        }

        let user = match app_state.auth_use_cases.claims_to_user(&claims) {
            Ok(value) => value,
            Err(_) => return Err((StatusCode::UNAUTHORIZED, "invalid token claims")),
        };

        Ok(Self {
            user,
            _policy: PhantomData,
        })
    }
}

use axum::extract::FromRef;

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use axum::{body::Body, http::Request, routing::get, Router};
    use chrono::Utc;
    use jsonwebtoken::{encode, EncodingKey, Header};
    use tower::ServiceExt;
    use uuid::Uuid;

    use crate::{
        application::auth::use_cases::AuthUseCases,
        application::menu::MenuService,
        domain::{
            auth::Role,
            menu::MenuRepository,
            repositories::{LoginAttemptState, RefreshTokenRecord, UserRepository},
            value_objects::{email::Email, password_hash::PasswordHash},
        },
        infrastructure::{
            services::{jwt_service::JwtService, password_hasher::PasswordHasher, refresh_token_service::RefreshTokenService},
        },
        presentation::auth::policy::{SuperAdminOnly, TenantAdminOnly},
        AppState,
    };

    use super::AuthUser;

    struct EmptyRepo;

    struct EmptyMenuRepo;

    #[async_trait::async_trait]
    impl MenuRepository for EmptyMenuRepo {
        async fn list_menu(&self, _tenant_id: Uuid) -> Result<Vec<crate::domain::menu::Category>, anyhow::Error> {
            Ok(Vec::new())
        }
        async fn list_products(&self, _tenant_id: Uuid) -> Result<Vec<crate::domain::menu::Product>, anyhow::Error> { Ok(Vec::new()) }
        async fn list_categories(&self, _tenant_id: Uuid) -> Result<Vec<crate::domain::menu::Category>, anyhow::Error> { Ok(Vec::new()) }
        async fn create_product(&self, _tenant_id: Uuid, _category_id: Uuid, _name: &str, _description: Option<&str>, _price: f64, _stock: i32, _image_url: Option<&str>) -> Result<crate::domain::menu::Product, anyhow::Error> { unreachable!() }
        async fn update_product(&self, _tenant_id: Uuid, _product_id: Uuid, _category_id: Uuid, _name: &str, _description: Option<&str>, _price: f64, _stock: i32, _image_url: Option<&str>) -> Result<Option<crate::domain::menu::Product>, anyhow::Error> { unreachable!() }
        async fn delete_product(&self, _tenant_id: Uuid, _product_id: Uuid) -> Result<bool, anyhow::Error> { unreachable!() }
        async fn create_category(&self, _tenant_id: Uuid, _name: &str, _description: Option<&str>, _image_url: Option<&str>, _display_order: i32) -> Result<crate::domain::menu::Category, anyhow::Error> { unreachable!() }
        async fn update_category(&self, _tenant_id: Uuid, _category_id: Uuid, _name: &str, _description: Option<&str>, _image_url: Option<&str>, _display_order: i32) -> Result<Option<crate::domain::menu::Category>, anyhow::Error> { unreachable!() }
        async fn deactivate_category(&self, _tenant_id: Uuid, _category_id: Uuid) -> Result<bool, anyhow::Error> { unreachable!() }
    }

    #[async_trait::async_trait]
    impl UserRepository for EmptyRepo {
        async fn total_users(&self) -> Result<i64, anyhow::Error> { Ok(0) }
        async fn find_by_email(&self, _email: &str) -> Result<Option<crate::domain::auth::User>, anyhow::Error> { Ok(None) }
        async fn find_by_id(&self, _user_id: Uuid) -> Result<Option<crate::domain::auth::User>, anyhow::Error> { Ok(None) }
        async fn create_tenant(&self, _name: &str, _slug: &str) -> Result<crate::domain::auth::Tenant, anyhow::Error> { unreachable!() }
        async fn get_tenant(&self, _tenant_id: Uuid) -> Result<Option<crate::domain::auth::Tenant>, anyhow::Error> { Ok(None) }
        async fn create_user(&self, _tenant_id: Uuid, _name: &str, _email: &str, _password_hash: &str, _role: Role) -> Result<crate::domain::auth::User, anyhow::Error> { unreachable!() }
        async fn persist_refresh_token(&self, _id: Uuid, _user_id: Uuid, _tenant_id: Uuid, _token_hash: &str, _expires_at: chrono::DateTime<chrono::Utc>) -> Result<(), anyhow::Error> { Ok(()) }
        async fn get_valid_refresh_token(&self, _token_hash: &str, _now: chrono::DateTime<chrono::Utc>) -> Result<Option<RefreshTokenRecord>, anyhow::Error> { Ok(None) }
        async fn revoke_refresh_token(&self, _token_id: Uuid, _replaced_by: Option<Uuid>) -> Result<(), anyhow::Error> { Ok(()) }
        async fn get_login_attempt_state(&self, _email: &str) -> Result<Option<LoginAttemptState>, anyhow::Error> { Ok(None) }
        async fn register_login_failure(&self, _email: &str, _max_attempts: i32, _lock_minutes: i32) -> Result<LoginAttemptState, anyhow::Error> {
            Ok(LoginAttemptState { failed_attempts: 1, locked_until: None })
        }
        async fn reset_login_failures(&self, _email: &str) -> Result<(), anyhow::Error> { Ok(()) }
    }

    async fn super_admin_endpoint(_auth: AuthUser<SuperAdminOnly>) -> &'static str {
        "ok"
    }

    async fn tenant_admin_endpoint(_auth: AuthUser<TenantAdminOnly>) -> &'static str {
        "ok"
    }

    fn app() -> Router {
        let repo = Arc::new(EmptyRepo);
        let jwt = JwtService::new("tests-secret", 1);
        let use_cases = AuthUseCases::new(
            repo,
            PasswordHasher::default(),
            jwt.clone(),
            RefreshTokenService::new("tests-refresh-secret".to_string(), 60 * 60 * 24),
            5,
            15,
        );
        let state = AppState {
            pool: sqlx::PgPool::connect_lazy("postgres://ignored").expect("pool"),
            auth_use_cases: Arc::new(use_cases),
            jwt_service: jwt,
            login_rate_limiter: crate::infrastructure::services::login_rate_limiter::LoginRateLimiter::new(20, 60),
            menu_service: Arc::new(MenuService::new(Arc::new(EmptyMenuRepo))),
            order_service: Arc::new(crate::application::orders::OrderService::new(Arc::new(crate::infrastructure::repositories::sqlx_orders_repository::SqlxOrderRepository::new(sqlx::PgPool::connect_lazy("postgres://ignored").expect("pool"))))),
        };

        Router::new()
            .route("/super", get(super_admin_endpoint))
            .route("/tenant", get(tenant_admin_endpoint))
            .with_state(state)
    }

    #[tokio::test]
    async fn returns_401_for_corrupted_token() {
        let response = app()
            .oneshot(
                Request::builder()
                    .uri("/super")
                    .header("Authorization", "Bearer invalid.token.here")
                    .body(Body::empty())
                    .expect("request"),
            )
            .await
            .expect("response");

        assert_eq!(response.status(), axum::http::StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn returns_401_for_expired_token() {
        let now = Utc::now().timestamp();
        let claims = crate::infrastructure::services::jwt_service::JwtClaims {
            sub: Uuid::new_v4().to_string(),
            user_id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            tenant_name: "Bits TI Tecnología".to_string(),
            role: "SUPER_ADMIN".to_string(),
            permissions: vec!["manage:tenant_admins".to_string()],
            email: "admin@example.com".to_string(),
            name: "Admin".to_string(),
            iat: now - 7200,
            exp: now - 3600,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret("tests-secret".as_bytes()),
        )
        .expect("token");

        let response = app()
            .oneshot(
                Request::builder()
                    .uri("/super")
                    .header("Authorization", format!("Bearer {token}"))
                    .body(Body::empty())
                    .expect("request"),
            )
            .await
            .expect("response");

        assert_eq!(response.status(), axum::http::StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn returns_403_when_role_is_insufficient() {
        let jwt = JwtService::new("tests-secret", 3600);
        let user = crate::domain::auth::User {
            id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            tenant_name: "Bits TI Tecnología".to_string(),
            name: "Cashier".to_string(),
            email: Email::parse("cashier@example.com").expect("email"),
            password_hash: PasswordHash::new("$argon2id$v=19$m=19456,t=2,p=1$a2V5$YWJj".to_string()).expect("hash"),
            role: Role::CAJERO,
        };

        let token = jwt
            .issue_access_token(&crate::infrastructure::services::jwt_service::JwtClaims::from_user(&user))
            .expect("token");

        let response = app()
            .oneshot(
                Request::builder()
                    .uri("/tenant")
                    .header("Authorization", format!("Bearer {token}"))
                    .body(Body::empty())
                    .expect("request"),
            )
            .await
            .expect("response");

        assert_eq!(response.status(), axum::http::StatusCode::FORBIDDEN);
    }
}
