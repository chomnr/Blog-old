use std::error::Error;
use std::{fmt, cmp::min, any::Any};
use std::any::type_name;

use tokio_postgres::error::SqlState;


#[derive(Debug)]
pub enum AccountError {
    UsernameViolation,
    EmailViolation,
    PasswordViolation,
    UsernameTaken,
    EmailTaken,
    RegistrationFailed, //hmm
    LoginFailed,
    UnknownError,
    InvalidSession
}

impl fmt::Display for AccountError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AccountError::UsernameViolation => write!(f, "The specified username violates the format requirements."),
            AccountError::EmailViolation => write!(f, "The specified email address violates the format requirements."),
            AccountError::PasswordViolation => write!(f, "The specified password violates the format requirements."),
            AccountError::UsernameTaken => write!(f, "The specified username is already taken."),
            AccountError::EmailTaken => write!(f, "The specified email is already taken."),
            AccountError::RegistrationFailed => write!(f, "Account registration failed due to an unknown error. Please try again later."),
            AccountError::LoginFailed => write!(f, "Login failed due to an unknown error. Please try again later."),
            AccountError::UnknownError => write!(f, "This error is unknown to the world."),
            AccountError::InvalidSession => write!(f, "The specified session is invalid."),
        }
    }
}


impl AccountError {
    pub fn error_parse(er: tokio_postgres::Error) -> AccountError {
        let code = er.code().unwrap();
        if code.eq(&SqlState::UNIQUE_VIOLATION) {
            let message = er.as_db_error().unwrap().message();
            if message.contains("username") {
                return AccountError::UsernameTaken
            }
            if message.contains("email") {
                return AccountError::EmailTaken
            }
        }
        return AccountError::UnknownError
    }
}

#[derive(Debug)]
pub enum PostError {
    PostViolation,
    InvalidPostId
}

impl fmt::Display for PostError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PostError::PostViolation => write!(f, "The specified post violates the format requirements."),
            PostError::InvalidPostId => write!(f, "The specified post id is invalid."),
        }
    }
}