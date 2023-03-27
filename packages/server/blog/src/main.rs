mod api;
pub mod quik_utils;

use std::{collections::{hash_map::DefaultHasher, HashMap}, hash::Hash, fmt::format, ptr::null};

use api::account::{self, Account};
use config::Config;
use deadpool_postgres::{ManagerConfig, RecyclingMethod, Runtime};
use log::warn;
use rouille::router;
use tokio_postgres::NoTls;

pub const SETTING_FILE: &str = "Settings";

#[tokio::main]
async fn main() {
    let settings = Config::builder()
        .add_source(config::File::with_name(SETTING_FILE))
        .build()
        .unwrap();
    
    let rouille_host: String = settings.get("rouille_host").unwrap();
    let rouille_port: String = settings.get("rouille_port").unwrap();
    let rouille_addr = format!("{}:{}", rouille_host, rouille_port);

    let postgres_host: String = settings.get("postgres_host").unwrap();
    let postgres_port: u16 = settings.get("postgres_port").unwrap();
    let postgres_username: String = settings.get("postgres_username").unwrap();
    let postgres_password: String = settings.get("postgres_password").unwrap();
    let postgres_database: String = settings.get("postgres_database").unwrap();

    let mut postgres_config = deadpool_postgres::Config::new();
    postgres_config.port = Some(postgres_port);
    postgres_config.user = Some(postgres_username);
    postgres_config.password = Some(postgres_password);
    postgres_config.dbname = Some(postgres_database);
    postgres_config.manager = Some(ManagerConfig { recycling_method: RecyclingMethod::Fast });

    let postgres_pool = postgres_config.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();

    let postgres_pool = postgres_pool.get();
    let acc = Account::new("Paperdsasd", "harryd@gmail.com", "adsdsadsadsasdadsa");
    let manager = acc.manager(postgres_pool.await.unwrap());

    manager.add_account().await.unwrap();

    rouille::start_server(rouille_addr, move |request| {
        let response = api::account::routes(&request);
        response
    });
}
