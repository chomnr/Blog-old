use regex::Regex;
use rocket::Route;
use tokio_postgres::{types::ToSql, error::SqlState};

use crate::api::{error::AccountError, routes::user_routes};

use super::{Service, ServiceInfo, ServiceStats};

use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, SaltString
    },
    Argon2
};

#[derive(Default)]
struct UserPassword {
    hash: Box<[u8]>,
    salt: Box<[u8]>,
}

//todo: refractor
impl UserPassword {
    pub fn new(password: &str) -> Self {
        let salt = Self::gen_salt();
        let argon2 = Argon2::default();
        let hash = argon2.hash_password(password.as_bytes(), salt.as_salt()).unwrap();
        Self {
            hash: hash.hash.unwrap().as_bytes().into(),
            salt: salt.as_ref().as_bytes().into(),
        }
    }
    fn gen_salt() -> SaltString {
        SaltString::generate(&mut OsRng)
    }
}

pub struct User {
    uid: String,
    username: String,
    password: UserPassword, 
    email: String
}

impl ServiceInfo for User {
    fn register_service(pool: deadpool_postgres::Pool) -> Service<Self> {
        Service {
            service: Self {
                uid: String::default(),
                username: String::default(),
                password: UserPassword::default(),
                email: String::default(),
            },
            stats: ServiceStats::default(),
            routes: user_routes::routes(),
            pool,
        }
    }
}

impl Service<User> {

    // Username Constraints
    const USERNAME_MAX: usize = 16;
    const USERNAME_MIN: usize = 3;
    const USERNAME_REGEX: &str = r"^[a-zA-Z0-9_]+$";

    // Email Constraints
    const EMAIL_REGEX: &str = r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$";

    // Password Constrant | 1 Capital Letter, 1 Digit, 1 Special Character ( At least ).
    const PASSWORD_MIN: usize = 7;
    const PASSWORD_REGEX: &str = r#"^.*[!@#$%^&*(),.?":{}|<>].*[0-9].*[A-Z].{5,}$"#; // look-around, including look-ahead and look-behind, is not supported.

    // The designated identifier for the PostgreSQL table where all user information is stored.
    const USER_TABLE: &str = "users";

    /// This function inserts a new user record directly into the PostgreSQL database.
    /// 
    /// # Arguments
    /// 
    /// * `username` - Desired username.
    /// * `password` - Desired password.
    /// * `email` - Desired email.
    /// 
    /// # Examples
    /// 
    /// ```
    /// let mut user_service = User::register_service(None, conn);
    /// user_service.create("JohnDoe", "DoeFarmer123", "JohnDoe@gmail.com");
    /// ```
    pub async fn create(&mut self, username: &str, password: &str, email: &str) -> Result<(), AccountError> {
        // Calling the procedures and or constraints.
        Self::username_proc(username).unwrap();
        Self::password_proc(password).unwrap();
        Self::email_proc(email).unwrap();
        // Converting the given password string of type 
        // &str to an instance of UserPassword.
        let password = UserPassword::new(password);
        // Since a new account is being created, a unique identifier (UID) is required.
        let uid = uuid::Uuid::new_v4().as_simple().to_string();
        // Specifies the SQL statement that will be executed to perform the desired action.
        let sql = format!("INSERT INTO {} (uid, username, email, password_hash, password_salt)  VALUES ($1, $2, $3, $4, $5)", Self::USER_TABLE);
        // Executing the query.
        self.short_query(sql.as_str(), 
            &[
                &uid, 
                &username, 
                &email, 
                &password.hash.as_ref(), 
                &password.salt.as_ref()
                ]).await.unwrap();
        Ok(())
    }

    /// Validates the input string as a username by performing 
    /// a comprehensive check that includes length requirements 
    /// and regular expression pattern matching. If the input 
    /// string does not satisfy these criteria, the function 
    /// returns an AccountError object with the specific error 
    /// UsernameViolation.
    fn username_proc(username: &str) -> Result<(), AccountError> {
        // Verifies whether the length of the username is 
        // below the prescribed minimum.
        if username.len() < Self::USERNAME_MIN {
            return Err(AccountError::UsernameViolation);
        }
        // Verifies whether the length of the username exceeds 
        // the permissible maximum value.
        if username.len() > Self::USERNAME_MAX {
            return Err(AccountError::UsernameViolation);
        }
        // Validates the username against the defined USERNAME_REGEX 
        // pattern to ensure its compliance.
        let pattern = Regex::new(Self::USERNAME_REGEX).unwrap();
        if !pattern.is_match(username) {
            return Err(AccountError::UsernameViolation);
        }
        Ok(())
    }

    /// This function is responsible for validating the user's 
    /// password, ensuring that it adheres to the specific pattern 
    /// defined by a regular expression, and meets the required 
    /// length. If the password does not meet these criteria, it 
    /// will return an AccountError with the specific error type 
    /// of PasswordViolation. (Does not support 1 special character currently)
    fn password_proc(password: &str) -> Result<(), AccountError> {
        // Verifies whether the length of the password is 
        // below the prescribed minimum.
        if password.len() < Self::PASSWORD_MIN {
            return Err(AccountError::PasswordViolation)
        }
        // Verifies whether password contains at least
        // one UPPERCASE letter.
        if !password.chars().any(|c| c.is_uppercase()) {
            return Err(AccountError::PasswordViolation)
        }
        // Verifies whether password contains at least
        // one DIGIT.
        if !password.chars().any(|c| c.is_digit(10)) {
            return Err(AccountError::PasswordViolation)
        }
        Ok(())
    }

    /// This function verifies whether the given email string 
    /// conforms to the required regular expression pattern. 
    /// If the email does not comply with the pattern, the 
    /// function returns an AccountError with the specific 
    /// error type of EmailViolation.
    fn email_proc(email: &str) -> Result<(), AccountError> {
        // Validates the email against the defined EMAIL_REGEX 
        // pattern to ensure its compliance.
        let pattern = Regex::new(Self::EMAIL_REGEX).unwrap();
        if !pattern.is_match(email) {
            return Err(AccountError::EmailViolation);
        }
        Ok(())
    }

    /// This function encapsulates the existing postgres query 
    /// to streamline the requisite procedures for executing 
    /// a query. 
    async fn short_query(&mut self, sql: &str, params: &[&(dyn ToSql + Sync)]) -> Result<(), AccountError> where {
        let conn = &self.pool.get().await.unwrap();
        // Prepare the query.
        let statement = conn.prepare(sql).await.unwrap();
        // Execute query.
        match conn.query(&statement, params).await {
            Ok(_) => {
                self.stats.add_usage(1); // Add that the class was used.
                Ok(())
            },
            Err(er) => {
                let code = er.code().unwrap();
                if code.eq(&SqlState::UNIQUE_VIOLATION) {
                    let message = er.as_db_error().unwrap().message();
                    if message.contains("username") {
                        return Err(AccountError::UsernameTaken)
                    }
                    if message.contains("email") {
                        return Err(AccountError::EmailTaken)
                    }
                }
                return Err(AccountError::UnknownError)
            },
        }
    }
}
