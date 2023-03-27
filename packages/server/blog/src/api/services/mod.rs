use std::{default, time::{SystemTime, UNIX_EPOCH}};

use rocket::Route;


mod user_service;
pub use user_service::{User};

pub struct Service<T: ServiceInfo> {
    service: T, // have no clue what i was going to do with this...
    pub stats: ServiceStats,
    routers: Option<Vec<Route>>,
    conn: deadpool_postgres::Object
}

impl<T: ServiceInfo> Service<T> {
    pub fn routers(self) -> Option<Vec<Route>> {
        self.routers
    }
}

pub trait ServiceInfo {
    fn register(routes: Option<Vec<Route>>, conn: deadpool_postgres::Object) -> Service<Self> where Self: Sized;
}

pub struct ServiceStats {
    name: String,
    usage: u64,
    last_usage: u128,
}

impl Default for ServiceStats {
    fn default() -> Self {
        Self { name: 
            file!().to_string(), 
            usage: 0, 
            last_usage: 0 
        }
    }
}

impl ServiceStats {
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn usage(&self) -> &String {
        &self.name
    }

    pub fn last_usage(&self) -> &String {
        &self.name
    }

    pub fn add_usage(&mut self, amount: u64) {
        if amount > 1 {
            println!("You can only increment by 1.")
        }
        if amount < 1 {
            println!("You cannot increment by a number that is less than 0.")
        }
        self.usage += amount;
        self.last_usage = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    }
}