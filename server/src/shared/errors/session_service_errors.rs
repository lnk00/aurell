use std::fmt;

#[derive(Debug)]
pub enum SessionServiceError {
    SigninFailed,
    RevokeFailed,
    ServiceUnavailable,
}

impl fmt::Display for SessionServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SessionServiceError::SigninFailed => write!(f, "Failed to sign in."),
            SessionServiceError::RevokeFailed => write!(f, "Failed to revoke session."),
            SessionServiceError::ServiceUnavailable => write!(f, "Service is unavailable"),
        }
    }
}

impl std::error::Error for SessionServiceError {}
