use std::{default, time::{SystemTime, UNIX_EPOCH}, collections::HashMap, sync::{Arc, Mutex, RwLock}, marker::PhantomData, hash::Hash};

use rocket::Route;

mod config_service;
mod user_service;
mod database_service;
mod post_service;
pub use config_service::{Config};
pub use user_service::{User, UserSession, SecurePassword};
pub use database_service::{Postgres};
pub use post_service::{Post};

lazy_static::lazy_static! {
    pub static ref SERVICE_REGISTRY: RwLock<HashMap<String, Vec<ServiceStats>>> = RwLock::new(HashMap::new());
}

//todo implement service_registry for all classes to avoid using self mut on all my functions...

pub struct Service<T> {
    name: String,
    category: String,
    status: bool,
    service: T,
    statistics: Vec<ServiceStats>,
}

pub struct ServiceStats {
    name: String,
    function: String,
    category: String,
    usage: u128,
    last_usage: u128
}

impl ServiceStats {
    // Return the name
    pub fn name(&self) -> &String {
        &self.name
    }
    // Return the function name...
    pub fn function(&self) -> &String {
        &self.function
    }
    // Return the function category
    pub fn category(&self) -> &String {
        &self.category
    }
    // Return the function usage
    pub fn usage(&self) -> &u128 {
        &self.usage
    }
    // Return the function last usage.
    pub fn last_usage(&self) -> &u128 {
        &self.usage
    }
}

impl<T> Service<T> {
    pub fn statistics(&self) -> &Vec<ServiceStats> {
        &self.statistics
    }
}