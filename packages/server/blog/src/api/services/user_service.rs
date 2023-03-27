use regex::Regex;
use rocket::Route;

use crate::{quik_utils::{quik_id, quik_hash}, api::error::AccountError};

use super::{Service, ServiceInfo, ServiceStats};

#[derive(Default)]
struct UserPassword {
    hash: Box<[u8]>,
    salt: Box<[u8]>,
}

pub struct User {
    uid: String,
    username: String,
    password: UserPassword, 
    email: String
}

impl ServiceInfo for User {
    fn register_service(routes: Option<Vec<Route>>, conn: deadpool_postgres::Object) -> Service<Self> {
        Service {
            service: Self {
                uid: String::default(),
                username: String::default(),
                password: UserPassword::default(),
                email: String::default(),
            },
            stats: ServiceStats::default(),
            routers: routes,
            conn,
        }
    }
}

impl Service<User> {

    // Username Constraints
    pub const USERNAME_MAX: usize = 16;
    pub const USERNAME_MIN: usize = 3;
    pub const USERNAME_REGEX: &str = r"^[a-zA-Z0-9_]+$";

    // Email Constraints
    pub const EMAIL_REGEX: &str = r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$";

    // Password Constrant | 1 Capital Letter, 1 Digit, 1 Special Character ( At least ).
    pub const PASSWORD_MIN: usize = 7;
    pub const PASSWORD_REGEX: &str = r#"^.*[!@#$%^&*(),.?":{}|<>].*[0-9].*[A-Z].{5,}$"#; // look-around, including look-ahead and look-behind, is not supported.

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
    pub async fn create(&self, username: &str, password: &str, email: &str) -> Result<(), AccountError> {
        // Calling the procedures and or constraints.
        Self::username_proc(username).unwrap();
        Self::password_proc(password).unwrap();
        Self::email_proc(email).unwrap();

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
    pub fn password_proc(password: &str) -> Result<(), AccountError> {
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

    /*
    pub fn read(){}
    pub fn update(){}
    pub fn delete(){}
    */
}

/*
let password = quik_hash(password);
        
        let to_make = User {
            uid: uuid::Uuid::new_v4().as_simple().to_string(),
            username: username.to_string(),
            password: UserPassword { hash: password.0, salt: password.1 },
            email: email.to_string(),
        };

 */