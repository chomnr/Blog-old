use rocket::Route;

use super::{Service, ServiceInfo, ServiceStats};

pub struct User {

}

impl ServiceInfo for User {
    fn register(routes: Option<Vec<Route>>) -> Service<Self> {
        Service {
            info: None,
            stats: ServiceStats::default(),
            routers: routes,
        }
    }
}

impl Service<User> {
    pub fn create(){}
    pub fn read(){}
    pub fn update(){}
    pub fn delete(){}
}