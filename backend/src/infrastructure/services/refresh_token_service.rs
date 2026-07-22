use chrono::{Duration, Utc};
use sha2::{Digest, Sha256};
use uuid::Uuid;

#[derive(Clone)]
pub struct RefreshTokenService {
    secret: String,
    ttl_seconds: i64,
}

impl RefreshTokenService {
    pub fn new(secret: String, ttl_seconds: i64) -> Self {
        Self { secret, ttl_seconds }
    }

    pub fn generate_raw_token(&self) -> String {
        let random = Uuid::new_v4();
        format!("rt_{}_{}", random, Utc::now().timestamp_nanos_opt().unwrap_or_default())
    }

    pub fn hash_token(&self, raw_token: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(self.secret.as_bytes());
        hasher.update(raw_token.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    pub fn expires_at(&self) -> chrono::DateTime<Utc> {
        Utc::now() + Duration::seconds(self.ttl_seconds)
    }

    pub fn ttl_seconds(&self) -> i64 {
        self.ttl_seconds
    }
}
