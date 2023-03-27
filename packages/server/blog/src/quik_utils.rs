use std::time::{SystemTime, UNIX_EPOCH};

/// List of quick utilities
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, SaltString
    },
    Argon2
};

/* MAKING ARGON2 HIGHER LEVEL. */
pub fn quik_hash(password: &str) -> (Box<[u8]>, Box<[u8]>) {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2.hash_password(password.as_bytes(), &salt).unwrap();
        (password_hash.hash.unwrap().as_bytes().into(), salt.as_ref().as_bytes().into())
}

pub fn quik_id() -> u128 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()
}