use std::{sync::Arc, time::{SystemTime, self}};

use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};
use rand::Rng;
use regex::Regex;
use rocket::{Route, http::{Cookie, SameSite, private::cookie::Expiration}, serde::{self, Deserialize, Serialize}, time::{OffsetDateTime, Duration}};
use tokio_postgres::{types::ToSql, error::SqlState, Row};

use crate::api::{error::AccountError, routes::user_routes};

use super::{Service, ServiceInfo, ServiceStats};

#[derive(Default)]
struct UserPassword {
    hash: String
}

//todo: refractor
impl UserPassword {
    pub fn new(password: &str) -> Self {
        let argon2 = Argon2::default();
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = argon2.hash_password(password.as_bytes(), &salt).unwrap().to_string();
        Self {
            hash: password_hash.to_string(),
        }
    }
}

pub struct User;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserSession {
    sid: String,
    uid: String,
    username: String,
    email: String,
    expires_on: i64
}

impl ServiceInfo for User {
    fn register_service(pool: Arc<deadpool_postgres::Pool>) -> Service<Self> {
        Service {
            service: None,
            stats: ServiceStats::default(),
            routes: Some(user_routes::routes()),
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

    // Session Duration
    const SESSION_DURATION: usize = 604800000; // (7 Days)

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
        let sql = format!("INSERT INTO {} (uid, username, email, password_hash)  VALUES ($1, $2, $3, $4)", Self::USER_TABLE);
        // Executing the query.
        let short_query = self.short_query(sql.as_str(), 
            &[
                &uid, 
                &username, 
                &email, 
                &password.hash, 
                ]).await;
        if short_query.is_err() {
            return Err(
                AccountError::error_parse(short_query.err().unwrap()
            ))
        }
        Ok(())
    }

    /// This function implements an authentication mechanism that determines the validity of 
    /// user credentials, and thus determines the access control decision for allowing or 
    /// denying login into a user account.
    /// 
    /// # Arguments
    /// 
    /// * `login` - Desired username or email.
    /// * `password` - Desired password.
    /// 
    /// # Examples
    /// 
    /// ```
    /// let mut user_service = User::register_service(None, conn);
    /// user_service.login("JohnDoe", "DoeFarmer123");
    /// ```
    pub async fn login(&mut self, login: &str, password: &str) -> Result<UserSession, AccountError> {
        // Getting Postgres object.
        let conn = &self.pool.get().await.unwrap();
        // Deciding whether 'login' is a email or username.
        let method = Self::login_method(login);
        // Preparing query.
        let sql = format!("SELECT * from {} WHERE {} = $1", Self::USER_TABLE, method);
        // Execute query.
        match self.short_query(&sql, &[&login]).await {
            Ok(v) => {
                if v.len() == 0 { return Err(AccountError::LoginFailed); }
                let uid: String = v[0].get(0);
                let username: String = v[0].get(1);
                let email: String = v[0].get(2);
                let target: &str = v[0].get(3);
                Self::password_verify(password, target).unwrap();
                Ok(self.session_make(uid.as_str(), username.as_str(), email.as_str()).await.unwrap())
            },
            Err(_) => {
                return Err(AccountError::UnknownError)
            },
        }
    }

    /// creates session id inside sessions table then creates a cookie...
    async fn session_make(&mut self, uid: &str, username: &str, email: &str) -> Result<UserSession, AccountError> {
        // Simple query insert if session does already 
        // exist if it does update it.
        let sql = "
                INSERT INTO sessions (uid, sid, expires_on)
                VALUES ($1, $2, $3)
                ON CONFLICT (uid) DO UPDATE SET
                    sid = EXCLUDED.sid,
                    expires_on = EXCLUDED.expires_on
            ";
        // Generate random session_id.
        let sid = uuid::Uuid::new_v4().as_simple().to_string();
        // Create the expiration.
        let expires_on = SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64;
        // The UserSession object.
        let user_session = UserSession { 
            sid: sid.to_string(),
            uid: uid.to_string(), 
            username: username.to_string(), 
            email: email.to_string(),
            expires_on,
        };
        // Execute query.
        match self.short_query(sql, &[&uid, &sid, &expires_on]).await {
            Ok(_) => {
               Ok(user_session)
            },
            Err(_) => Err(AccountError::UnknownError),
        }
    }

    /// The function "verify_password" performs a password 
    /// comparison operation to determine whether the provided 
    /// password is equivalent to the target password. If the 
    /// comparison yields a match, the function returns a 
    /// boolean value of true. However, if the comparison 
    /// fails, the function raises an AccountError with the 
    /// specific error LoginFailed, indicating that the 
    /// password verification process was unsuccessful.
    fn password_verify(password: &str, target: &str) -> Result<bool, AccountError> {
        // Default Argon2 configuration.
        let argon2 = Argon2::default();
        // PasswordHash of target password.
        let target_hash = argon2::PasswordHash::new(target).unwrap();
        // Checks if password == Target.
        let check = argon2.verify_password(password.as_bytes(), &target_hash).is_ok();
        // If false return AccountError LoginFailed
        if !check {
            return Err(AccountError::LoginFailed);
        }
        Ok(true)
    }

    /// The function determines the appropriate login method based 
    /// on the input string by comparing it to an email pattern. 
    /// If the input string does not match the email pattern, 
    /// it is considered a username.
    pub fn login_method(login: &str) -> &str {
        let pattern = Regex::new(Self::EMAIL_REGEX).unwrap();
        // Based on the pattern of the given string, 
        // if it does not match the structure of an 
        // email, it is determined to be a username.
        if pattern.is_match(login) {
            return "EMAIL"
        } else {
            return "USERNAME"
        }
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
        // Verifies whether the length of the username exceeds 
        // the permissible maximum value.
        if password.len() > argon2::MAX_PWD_LEN {
            return  Err(AccountError::PasswordViolation);
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
    async fn short_query(&mut self, sql: &str, params: &[&(dyn ToSql + Sync)]) -> Result<Vec<Row>, tokio_postgres::Error> {
        let conn = &self.pool.get().await.unwrap();
        // Prepare the query.
        let statement = conn.prepare(sql).await.unwrap();
        // Execute query.
        match conn.query(&statement, params).await {
            Ok(v) => {
                self.stats.add_usage(1); // Add that the class was used.
                Ok(v)
            },
            Err(er) => {
                return Err(er)
            },
        }
    }
}

/*
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
 */