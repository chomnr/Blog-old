#![feature(proc_macro_hygiene, decl_macro)]

use std::sync::Arc;
use rocket::{serde::json::{serde_json::json, Value}, get, routes};
use rocket_cors::{AllowedHeaders, AllowedOrigins};
use tracing_subscriber::layer::SubscriberExt;

use api::{services::{Config, Postgres, Service, User, SecurePassword}, routes::{user_routes, self}};
use dotenv::dotenv;

mod api;

#[get("/api")]
async fn api_index() -> Value {
    json!({
        "message": "Nothing to see here."
    })
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    dotenv().ok();
    // Cors
    // Services
    let config_service = Service::<Config>::new();
    let postgres_service = Service::<Postgres>::new(config_service.postgres());
    let user_service = Service::<User>::new(postgres_service.new_pool());
    /// Routes
    let user_routes = api::routes::user_routes::routes();
    // Rocket
    let rocket = rocket::build()
        .attach(rocket_cors::CorsOptions::default().to_cors().unwrap())
        .mount("/", rocket_cors::catch_all_options_routes())
        .mount("/", routes![api_index])
        .mount("/api/user", user_routes)
        .manage(user_service)
        .manage(rocket_cors::CorsOptions::default().to_cors().unwrap())
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