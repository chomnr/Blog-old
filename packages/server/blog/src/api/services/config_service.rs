use super::{Service, ServiceStats};

#[derive(Clone)]
pub struct PostgresConfig {
    host: String,
    port: u16,
    username: String,
    password: String,
    database: String
}

impl PostgresConfig {
    pub fn host(&self) -> &String {
        &self.host
    }
    pub fn port(&self) -> &u16 {
        &self.port
    }
    pub fn username(&self) -> &String {
        &self.username
    }
    pub fn password(&self) -> &String {
        &self.password
    }
    pub fn database(&self) -> &String {
        &self.database
    }
}

impl Default for PostgresConfig {
    fn default() -> Self {
        PostgresConfig { 
            host: dotenv::var("PG__HOST").unwrap().to_string(), 
            port: dotenv::var("PG__PORT").unwrap().parse::<u16>().unwrap(),
            username: dotenv::var("PG__USER").unwrap().to_string(), 
            password: dotenv::var("PG__PASS").unwrap().to_string(), 
            database: dotenv::var("PG__DBNAME").unwrap().to_string()
        }
    }
}

#[derive(Clone)]
pub struct Config {
    postgres: PostgresConfig
}

impl Service<Config> {
    pub fn new() -> Self {
        let statistics: Vec<ServiceStats> = Vec::new();
        Service { 
            name: "Config".to_string(), 
            category: file!().to_string(), 
            status: true, 
            service: Config { postgres: PostgresConfig::default() }, 
            statistics
        }
    }

    pub fn postgres(&self) -> &PostgresConfig {
        &self.service.postgres
    }
}