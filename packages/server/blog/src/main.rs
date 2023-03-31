#![feature(proc_macro_hygiene, decl_macro)]

use api::services::{Config, User};
use rocket_db_pools::{deadpool_postgres, Database};

mod api;

// Postgres Deadpool Pooling...
#[derive(Database)]
#[database("blog")]
struct PostgresPool(deadpool_postgres::Pool);

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let mut config_service = api::services::Service::<Config>::new();
    let mut user_service = api::services::Service::<User>::new();

    let rocket = rocket::build()
        .attach(PostgresPool::init())
        .ignite().await?
        .launch().await?;
    Ok(())
}

/* 
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
}*/


/*
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
    let pool = Arc::new(postgres_config.create_pool(Some(Runtime::Tokio1), NoTls).unwrap());

    let user_service = Mutex::new(User::register_service(pool.clone()));
    let user_routes = user_service.lock().await.routes().to_vec();

    rocket::build()
    .mount("/api/user", user_routes)
    .attach(CorsOptions::default().to_cors().unwrap())
    .manage(user_service)
    .manage(settings)
    .ignite().await.unwrap()
    .launch().await.unwrap();

    Ok(())
}
*/