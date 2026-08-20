use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::auth::{Permission, Role, User};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtClaims {
    pub sub: String,
    pub user_id: Uuid,
    pub tenant_id: Uuid,
    pub tenant_name: String,
    pub role: String,
    pub permissions: Vec<String>,
    pub email: String,
    pub name: String,
    pub branch_ids: Vec<Uuid>,
    pub iat: i64,
    pub exp: i64,
}

impl JwtClaims {
    pub fn from_user(user: &User) -> Self {
        let now = Utc::now();
        let expires = now + Duration::hours(4);

        Self {
            sub: user.id.to_string(),
            user_id: user.id,
            tenant_id: user.tenant_id,
            tenant_name: user.tenant_name.clone(),
            role: user.role.as_str().to_string(),
            permissions: user
                .role
                .permissions()
                .into_iter()
                .map(Permission::as_str)
                .map(str::to_string)
                .collect(),
            email: user.email.as_str().to_string(),
            name: user.name.clone(),
            branch_ids: user.branch_ids.clone(),
            iat: now.timestamp(),
            exp: expires.timestamp(),
        }
    }
}

#[derive(Clone)]
pub struct JwtService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    ttl_seconds: u64,
}

impl JwtService {
    pub fn new(secret: &str, ttl_seconds: u64) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret.as_bytes()),
            decoding_key: DecodingKey::from_secret(secret.as_bytes()),
            ttl_seconds,
        }
    }

    pub fn ttl_seconds(&self) -> u64 {
        self.ttl_seconds
    }

    pub fn issue_access_token(&self, claims: &JwtClaims) -> Result<String, anyhow::Error> {
        let mut adjusted = claims.clone();
        adjusted.exp = (Utc::now() + Duration::seconds(self.ttl_seconds as i64)).timestamp();
        adjusted.iat = Utc::now().timestamp();
        encode(&Header::default(), &adjusted, &self.encoding_key)
            .map_err(|error| anyhow::anyhow!("failed to sign token: {error}"))
    }

    pub fn validate_access_token(&self, token: &str) -> Result<JwtClaims, anyhow::Error> {
        let mut validation = Validation::default();
        validation.validate_exp = true;
        let data = decode::<JwtClaims>(token, &self.decoding_key, &validation)
            .map_err(|error| anyhow::anyhow!("invalid token: {error}"))?;
        Ok(data.claims)
    }

    pub fn has_role(claims: &JwtClaims, required_roles: &[Role]) -> bool {
        required_roles
            .iter()
            .any(|role| claims.role.as_str() == role.as_str())
    }

    pub fn has_permissions(claims: &JwtClaims, required_permissions: &[Permission]) -> bool {
        required_permissions
            .iter()
            .all(|permission| claims.permissions.iter().any(|value| value == permission.as_str()))
    }
}
