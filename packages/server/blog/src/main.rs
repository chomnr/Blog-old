mod api;

use std::sync::Arc;

use api::{services::{Service, User, ServiceInfo}, routes::user_routes};
use config::Config;
use deadpool_postgres::{ManagerConfig, RecyclingMethod, Runtime};
use rocket::{get, routes, Route, futures::lock::Mutex, fairing::{Fairing, Info, Kind}, Request, http::{Header, Method, Status}, Response};
use tokio_postgres::NoTls;

pub const SETTING_FILE: &str = "Settings";

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        if request.method() == Method::Options {
            response.set_status(Status::NoContent);
            response.set_header(Header::new(
                "Access-Control-Allow-Methods",
                "POST, PATCH, GET, DELETE",
            ));
            response.set_header(Header::new(
                "Access-Control-Allow-Headers",
                "content-type, authorization",
            ));
        }
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
        response.set_header(Header::new("Vary", "Origin"));
    }
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error>  {
    let settings = Config::builder()
        .add_source(config::File::with_name(SETTING_FILE))
        .build()
        .unwrap();
    let mut postgres_config = deadpool_postgres::Config::new();
    postgres_config.host = Some(settings.get("postgres_host").unwrap());
    postgres_config.port = Some(settings.get("postgres_port").unwrap());
    postgres_config.user = Some(settings.get("postgres_username").unwrap());
    postgres_config.password = Some(settings.get("postgres_password").unwrap());
    postgres_config.dbname = Some(settings.get("postgres_database").unwrap());
    postgres_config.manager = Some(ManagerConfig { recycling_method: RecyclingMethod::Fast });
    let pool = postgres_config.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();
    //let conn = postgres_pool.get().await.unwrap();
    
    let user_service = Mutex::new(User::register_service(pool));
    let user_routes = user_service.lock().await.routes().to_vec();
    

    rocket::build()
    .mount("/api/user", user_routes).manage(user_service).attach(CORS).launch().await.unwrap();

    Ok(())
}
