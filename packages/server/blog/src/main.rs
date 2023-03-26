mod api;
pub mod quik_utils;

use std::{collections::{hash_map::DefaultHasher, HashMap}, hash::Hash, fmt::format};

use api::account;
use config::Config;
use rouille::router;

#[tokio::main] 
async fn main() {
    let settings = Config::builder()
        .add_source(config::File::with_name("Settings"))
        .build()
        .unwrap();
    
    let rouille_host: String = settings.get("rouille_host").unwrap();
    let rouille_port: String = settings.get("rouille_port").unwrap();
    let rouille_addr = format!("{}:{}", rouille_host, rouille_port);

    rouille::start_server(rouille_addr, move |request| {
        router!(request,
            (GET) (/) => {
                rouille::Response::redirect_302("/hello/world")
            },
            _ => rouille::Response::empty_404()
        )
    });
}
