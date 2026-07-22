use crate::domain::errors::DomainError;

#[derive(Debug, Clone)]
pub struct PasswordHash(String);

impl PasswordHash {
    pub fn new(value: String) -> Result<Self, DomainError> {
        if value.starts_with("$argon2id$") || value.starts_with("$2") {
            Ok(Self(value))
        } else {
            Err(DomainError::InvalidPasswordHash)
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn validate_plaintext_rules(password: &str) -> Result<(), DomainError> {
        let has_len = password.len() >= 12;
        let has_upper = password.chars().any(|c| c.is_ascii_uppercase());
        let has_lower = password.chars().any(|c| c.is_ascii_lowercase());
        let has_digit = password.chars().any(|c| c.is_ascii_digit());
        let has_symbol = password.chars().any(|c| !c.is_ascii_alphanumeric());

        if has_len && has_upper && has_lower && has_digit && has_symbol {
            Ok(())
        } else {
            Err(DomainError::WeakPassword)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PasswordHash;

    #[test]
    fn accepts_supported_hash_format() {
        let hash = "$argon2id$v=19$m=19456,t=2,p=1$base64salt$base64hash".to_string();
        assert!(PasswordHash::new(hash).is_ok());
    }

    #[test]
    fn rejects_unknown_hash_format() {
        assert!(PasswordHash::new("plain-text".to_string()).is_err());
    }

    #[test]
    fn validates_password_policy() {
        assert!(PasswordHash::validate_plaintext_rules("StrongP@ssw0rd").is_ok());
        assert!(PasswordHash::validate_plaintext_rules("weakpass").is_err());
    }
}
