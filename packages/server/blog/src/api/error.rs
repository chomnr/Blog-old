use std::{fmt, cmp::min};

#[derive(Debug)]
pub enum AccountError {
    UsernameViolation,
    EmailViolation,
    RegistrationFailed,
}

impl fmt::Display for AccountError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AccountError::UsernameViolation => write!(f, "The specified username violates the format requirements"),
            AccountError::EmailViolation => write!(f, "The specified email address violates the format requirements."),
            AccountError::RegistrationFailed => write!(f, "Account registration failed due to an unknown error. Please try again later."),
        }
    }
}