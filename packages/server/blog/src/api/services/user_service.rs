use rocket::Route;

use super::{Service, ServiceInfo, ServiceStats};

#[derive(Default)]
struct UserPassword {
    hash: Box<[u8]>,
    salt: Box<[u8]>,
}

pub struct User {
    uid: u64,
    username: String,
    password: UserPassword, 
    email: String
}

impl ServiceInfo for User {
    fn register(routes: Option<Vec<Route>>, conn: deadpool_postgres::Object) -> Service<Self> {
        Service {
            service: Self {
                uid: u64::default(),
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
    pub fn create(&self) {
        
    }
    pub fn read(){}
    pub fn update(){}
    pub fn delete(){}
}