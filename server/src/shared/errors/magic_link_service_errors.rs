use std::fmt;

#[derive(Debug)]
pub enum MagicLinkServiceError {
    SendFailed,
    VerifyFailed,
    ServiceUnavailable,
}

impl fmt::Display for MagicLinkServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MagicLinkServiceError::SendFailed => write!(f, "Failed to send magic link."),
            MagicLinkServiceError::VerifyFailed => write!(f, "Failed to verify magic link token."),
            MagicLinkServiceError::ServiceUnavailable => write!(f, "Service is unavailable"),
        }
    }
}

impl std::error::Error for MagicLinkServiceError {}
