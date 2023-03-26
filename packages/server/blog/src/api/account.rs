use std::time::{SystemTime, UNIX_EPOCH};

use crate::quik_utils;

#[derive(Default)]
struct Password {
    hash: Box<[u8]>,
    salt: Box<[u8]>
}

impl Password {
    pub fn new(password: &str) -> Password {
        let quik_hash = quik_utils::quik_hash(password);
        Password { hash: quik_hash.0, salt: quik_hash.1 }
    }
}

pub struct Account {
    uid: u128,
    username: String,
    email: String,
    password: Password
}

impl Account {
    pub fn new(
        username: &str, 
        email: &str, 
        password: &str 
    ) -> Account {
        Account { 
            uid: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_nanos(), 
            username: String::from(username), 
            email: String::from(email), 
            password: Password::new(password) 
        }
    }
}

/* 
#[derive(Default)]
struct Password {
    hash: Box<[u8]>,
    salt: Box<[u8]>
}

impl Password {
    pub fn new(password: &str) -> Password {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2.hash_password(password.as_bytes(), &salt).unwrap();
        Password { 
            hash: password_hash.hash.unwrap().as_bytes().into(), 
            salt: salt.as_ref().as_bytes().into()
        }
    }
}

pub struct Account {
    uid: u128,
    username: String,
    email: String,
    password: Password
}

impl Default for Account {
    fn default() -> Self {
        let uid = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        Self {
            uid,
            username: String::default(),
            email: String::default(),
            password: Password::default(),
        }
    }
}
impl Account {
    pub fn new(username: & str, email: & str, password: & str) -> Account {
        let mut account = Account::default();
        account.username = String::from(username);
        account.email = String::from(email);
        account.password = Password::new(password);
        account
    }
}

pub struct Manager {
    account: Account
}

*/
