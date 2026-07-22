use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("invalid email format")]
    InvalidEmail,
    #[error("password hash has invalid format")]
    InvalidPasswordHash,
    #[error("password does not meet security requirements")]
    WeakPassword,
    #[error("role transition is not allowed")]
    RoleTransitionForbidden,
}
