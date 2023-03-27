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
    pub const USERNAME_REGEX: &str = "^[A-Za-z0-9_]+$";

    /// This function creates a user row inside Postgres.
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
    pub async fn create(&self, username: &str, password: &str, email: &str) {
        
    }

    /// Validates the input string as a username by performing 
    /// a comprehensive check that includes length requirements 
    /// and regular expression pattern matching. If the input 
    /// string does not satisfy these criteria, the function 
    /// returns an AccountError object with the specific error 
    /// message UsernameViolation.
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
        if pattern.is_match(username) {
            return Err(AccountError::UsernameViolation);
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