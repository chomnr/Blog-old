pub mod account;

// Postgres
use std::{fmt, cmp::min};

#[derive(Debug)]
pub enum AccountError {
    USERNAME_TOO_SHORT(String),
    USERNAME_TOO_LONG(String),
    ACCOUNT_CREATION_FAILED
}

impl fmt::Display for AccountError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AccountError::USERNAME_TOO_SHORT(username) => write!(f, "The username '{}' is too short.", username),
            AccountError::USERNAME_TOO_LONG(username) => write!(f, "The username '{}' is too long. ", username),
            AccountError::ACCOUNT_CREATION_FAILED => write!(f, "The account failed to create."),
        }
    }
}

//Self::InvalidLength(input) => write!(f, "Incorrect length for '{}' ", input),

/* SCHEMAS
    CREATE TABLE IF NOT EXISTS accounts (
        uid VARCHAR(255) PRIMARY KEY,
        username VARCHAR(16) UNIQUE NOT NULL,
        email VARCHAR(320) NOT NULL,
        password_hash BYTEA NOT NULL,
        password_salt BYTEA NOT NULL
    );
*/
