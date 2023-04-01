#![feature(proc_macro_hygiene, decl_macro)]

use std::sync::Arc;
use rocket::{serde::json::{serde_json::json, Value}, get, routes, http::Method};
use rocket_cors::{AllowedHeaders, AllowedOrigins, CorsOptions};
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
    //dotenv
    dotenv().ok();
    // your frontend url...
    let allowed_origins = AllowedOrigins::some_exact(&["http://localhost:3000/"]);
    // You can also deserialize this
    let cors = rocket_cors::CorsOptions {
        allowed_origins: allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post, Method::Options].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept", "Content-Type"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors().unwrap();
    // Services
    let config_service = Service::<Config>::new();
    let postgres_service = Service::<Postgres>::new(config_service.postgres());
    let user_service = Service::<User>::new(postgres_service.new_pool());
    // Routes
    let user_routes = api::routes::user_routes::routes();
    // Rocket
    let rocket = rocket::build()
        .mount("/", routes![api_index])
        .mount("/api/user", user_routes)
        .attach(cors)
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