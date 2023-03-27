mod api;
pub mod quik_utils;

use api::services::{Service, User, ServiceInfo};
use config::Config;
use deadpool_postgres::{ManagerConfig, RecyclingMethod, Runtime};
use rocket::{get};
use tokio_postgres::NoTls;

pub const SETTING_FILE: &str = "Settings";

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error>  {
    let settings = Config::builder()
        .add_source(config::File::with_name(SETTING_FILE))
        .build()
        .unwrap();

    let postgres_host: String = settings.get("postgres_host").unwrap();
    let postgres_port: u16 = settings.get("postgres_port").unwrap();
    let postgres_username: String = settings.get("postgres_username").unwrap();
    let postgres_password: String = settings.get("postgres_password").unwrap();
    let postgres_database: String = settings.get("postgres_database").unwrap();

    let mut postgres_config = deadpool_postgres::Config::new();
    postgres_config.host = Some(postgres_host);
    postgres_config.port = Some(postgres_port);
    postgres_config.user = Some(postgres_username);
    postgres_config.password = Some(postgres_password);
    postgres_config.dbname = Some(postgres_database);
    postgres_config.manager = Some(ManagerConfig { recycling_method: RecyclingMethod::Fast });

    let postgres_pool = postgres_config.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();
    let conn = postgres_pool.get().await.unwrap();

    let mut user_service = User::register(None, conn);
    let user_routes = user_service.routers().unwrap();

    rocket::build().mount("/", user_routes).launch().await.unwrap();

    Ok(())
}
