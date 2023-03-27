mod api;
pub mod quik_utils;

use api::services::{Service, User, ServiceInfo};
use config::Config;
use rocket::{launch, get, routes};

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    let mut test = User::register(None);
    
    rocket::build().mount("/", routes![hello])
}

//pub const SETTING_FILE: &str = "Settings";

/*
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
    postgres_config.host = Some(postgres_host);
    postgres_config.port = Some(postgres_port);
    postgres_config.user = Some(postgres_username);
    postgres_config.password = Some(postgres_password);
    postgres_config.dbname = Some(postgres_database);
    postgres_config.manager = Some(ManagerConfig { recycling_method: RecyclingMethod::Fast });

    let postgres_pool = postgres_config.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();
    let postgres = postgres_pool.get().await.unwrap();
    
    /*
        let manager = acc.manager(postgres_pool.await.unwrap());
        let acc = Account::new("Paperdsasd", "harryd@gmail.com", "adsdsadsadsasdadsa");
        manager.add_account().await.unwrap();

        let response = api::account::routes(&request, &postgres);
        response

    */
    let sessions_storage: Mutex<HashMap<String, &SessionData>> = Mutex::new(HashMap::new());

    rouille::start_server(rouille_addr, move |request| {
        rouille::log(&request, io::stdout(), || {
            rouille::session::session(request, "SID", 3600, |session| {
                let mut session_data = if session.client_has_sid() {
                    if let Some(data) = sessions_storage.lock().unwrap().get(session.id()) {
                        Some(data.clone())
                    } else {
                        None
                    }
                } else {
                    None
                };

                let response = routes(&request, &postgres, &mut session_data);
                if let Some(d) = session_data {
                    sessions_storage
                        .lock()
                        .unwrap()
                        .insert(session.id().to_owned(), d);
                } else if session.client_has_sid() {
    
                    sessions_storage.lock().unwrap().remove(session.id());
                }
                response
            })
        })
    });
}
*/
