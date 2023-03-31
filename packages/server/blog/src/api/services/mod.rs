use std::{default, time::{SystemTime, UNIX_EPOCH}, collections::HashMap, sync::{Arc, Mutex, RwLock}, marker::PhantomData, hash::Hash};

use rocket::Route;

mod config_service;
mod user_service;
pub use config_service::Config;
pub use user_service::User;

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
}
/*
impl<T> Service<T> {
    pub fn new() -> Self {
        let statistics: Vec<ServiceStats> = Vec::new();
        Service {
            name: file!().to_string()
                .replace("_service", "to"),
            category: file!().to_string(),
            status: true,
            service: todo!(),
            statistics,
        }
    }
}
*/

/* 
lazy_static::lazy_static! {
    pub static ref SERVICE_REGISTRY: RwLock<HashMap<String, Box<dyn ServiceInfo>>> = RwLock::new(HashMap::new());
}


#[derive(Clone)]
pub struct Service<S: ServiceInfo> {
    service: S,
    stats: ServiceStats,
    routes: Option<Vec<Route>>,
}

impl<T: ServiceInfo> Service<T> {
    pub fn routes(&self) -> Vec<Route> {
        self.routes.clone().unwrap_or_default()
    }
}

pub trait ServiceInfo: Send + Sync {
    fn register_service(self) -> Service<Self> where Self: Sized;
}

#[derive(Clone)]
pub struct ServiceStats {
    name: String,
    usage: HashMap<String, u128>
}

impl Default for ServiceStats {
    fn default() -> Self {
        Self { 
            name: file!().to_string(), 
            usage: HashMap::default(), 
        }
    }
}

impl ServiceStats {
    pub fn new(service_info: Box<dyn ServiceInfo>) -> ServiceStats {
        let service_stats = ServiceStats::default();
        let write = SERVICE_REGISTRY.write().unwrap();
        write.insert(service_stats.name.clone(), service_info).unwrap();
        service_stats
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn usage(&self) -> &String {
        &self.name
    }

    pub fn add_usage(&mut self, amount: u128) {
        if amount > 1 {
            println!("You can only increment by 1.")
        }
        if amount < 1 {
            println!("You cannot increment by a number that is less than 0.")
        }
        let parent_func = std::any::type_name::<fn()>();
        if let Some(current_amount) = self.usage.get_mut(parent_func) {
            *current_amount += amount
        } else {
            self.usage.insert(String::from(parent_func), 1);
        }
    }
    
    pub fn list_services() -> Vec<String> {
        let read = SERVICE_REGISTRY.read().unwrap();
        let mut list: Vec<String> = Vec::new();

        for (k, v) in read.iter() {
            list.push(k.to_string());
        }
        list
    }
}
*/