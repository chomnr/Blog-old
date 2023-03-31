use std::{default, time::{SystemTime, UNIX_EPOCH}, collections::HashMap, sync::{Arc, Mutex, RwLock}, marker::PhantomData, hash::Hash};

use rocket::Route;

mod config_service;
mod user_service;
mod database_service;
pub use config_service::{Config};
pub use user_service::{User};
pub use database_service::{Postgres};

lazy_static::lazy_static! {
    pub static ref SERVICE_REGISTRY: RwLock<HashMap<String, Vec<ServiceStats>>> = RwLock::new(HashMap::new());
}

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
    /*
    fn add_usage(&mut self, amount: u128) {
        let parent_func = std::any::type_name::<fn()>();
        if amount > 1 {
            println!("You can only increment by 1.")
        }
        if amount < 1 {
            println!("You cannot increment by a number that is less than 0.")
        }
        let size = self.statistics.len();
        let mut counter = 0;
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let mut found_stat = false;
        for stat in &mut self.statistics {
            if stat.name == parent_func {
                stat.usage += amount;
                stat.last_usage = time;
                found_stat = true;
                break;
            }
            counter +=1;
        }
        if !found_stat && counter == size {
            let new_stat = ServiceStats { 
                name: parent_func.to_owned(), 
                function: parent_func.to_owned(), 
                category: file!().to_string(), 
                usage: 1, 
                last_usage: time,
            };
            self.statistics.push(new_stat);
        }
    }
    */
}