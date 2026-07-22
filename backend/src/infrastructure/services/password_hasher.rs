use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher as _, PasswordVerifier, SaltString},
    Argon2,
};

#[derive(Clone)]
#[derive(Default)]
pub struct PasswordHasher {
    algorithm: Argon2<'static>,
}

impl PasswordHasher {
    pub fn hash(&self, password: &str) -> Result<String, anyhow::Error> {
        let salt = SaltString::generate(&mut OsRng);
        let hash = self
            .algorithm
            .hash_password(password.as_bytes(), &salt)
            .map_err(|error| anyhow::anyhow!("failed to hash password: {error}"))?
            .to_string();

        Ok(hash)
    }

    pub fn verify(&self, password: &str, password_hash: &str) -> Result<(), anyhow::Error> {
        if password_hash.starts_with("$2a$")
            || password_hash.starts_with("$2b$")
            || password_hash.starts_with("$2y$")
        {
            let is_valid = bcrypt::verify(password, password_hash)
                .map_err(|error| anyhow::anyhow!("invalid bcrypt hash: {error}"))?;

            if is_valid {
                return Ok(());
            }

            return Err(anyhow::anyhow!("invalid credentials"));
        }

        let hash = PasswordHash::new(password_hash)
            .map_err(|error| anyhow::anyhow!("invalid password hash: {error}"))?;

        self.algorithm
            .verify_password(password.as_bytes(), &hash)
            .map_err(|_| anyhow::anyhow!("invalid credentials"))
    }
}
