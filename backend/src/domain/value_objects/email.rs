use crate::domain::errors::DomainError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Email(String);

impl Email {
    pub fn parse(raw: &str) -> Result<Self, DomainError> {
        let trimmed = raw.trim().to_lowercase();
        let parts: Vec<&str> = trimmed.split('@').collect();
        if parts.len() != 2 || parts[0].is_empty() || parts[1].is_empty() {
            return Err(DomainError::InvalidEmail);
        }
        if !parts[1].contains('.') || parts[1].starts_with('.') || parts[1].ends_with('.') {
            return Err(DomainError::InvalidEmail);
        }
        Ok(Self(trimmed))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::Email;

    #[test]
    fn accepts_valid_email() {
        let email = Email::parse(" Admin@Example.com ").expect("valid email");
        assert_eq!(email.as_str(), "admin@example.com");
    }

    #[test]
    fn rejects_invalid_email() {
        assert!(Email::parse("invalid").is_err());
        assert!(Email::parse("john@").is_err());
        assert!(Email::parse("@example.com").is_err());
    }
}
