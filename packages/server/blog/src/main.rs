#![feature(proc_macro_hygiene, decl_macro)]

use std::sync::Arc;
use tracing_subscriber::layer::SubscriberExt;

use api::{services::{Config, Postgres, Service, User, SecurePassword}, routes::{user_routes, self}};
use dotenv::dotenv;

mod api;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    dotenv().ok();
    // Services
    let config_service = Service::<Config>::new();
    let postgres_service = Service::<Postgres>::new(config_service.postgres());
    let user_service = Service::<User>::new(postgres_service.new_pool());
    /// Routes
    let user_routes = api::routes::user_routes::routes();
    // Rocket
    let rocket = rocket::build()
        .mount("/api/user", user_routes)
        .manage(user_service)
        .ignite().await?
        .launch().await?;
    Ok(())
}


/*
    tracing::subscriber::set_global_default(
        tracing_subscriber::registry()
            .with(tracing_tracy::TracyLayer::new()),
    ).expect("set up the subscriber");

// Postgres Deadpool Pooling...
#[derive(Database)]
#[database("blog")]
struct PostgresPool(deadpool_postgres::Pool);
*/