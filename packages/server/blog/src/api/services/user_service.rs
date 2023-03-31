use std::default;

use super::{Service, ServiceStats};


pub struct SecurePassword {
    hash: String,
    salt: String
}

impl Default for SecurePassword {
    fn default() -> Self {
        Self { 
            hash: Default::default(), 
            salt: Default::default() 
        }
    }
}

pub struct User {
    uid: String,
    username: String,
    password: SecurePassword,
    email: String,
    avatar: String,
    bio: String
}

impl Default for User {
    fn default() -> Self {
        Self { 
            uid: String::default(), 
            username: String::default(), 
            password: SecurePassword::default(), 
            email: String::default(), 
            avatar: String::default(),
            bio: String::default()  
        }
    }
}


impl Service<User> {
    pub fn new() -> Service<User> {
        let statistics: Vec<ServiceStats> = Vec::new();
        Service { 
            name: "User".to_string(), 
            category: file!().to_string(), 
            status: true, 
            service: User::default(), 
            statistics
        }
    }
}