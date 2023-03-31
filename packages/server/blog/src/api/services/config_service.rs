use std::slice::SliceIndex;

use config::Config as ConfigAPI;

use super::{Service, ServiceStats};

const SETTING_FILE: &str = "Settings";

#[derive(Clone)]
pub struct PostgresConfig {
    host: String,
    port: i64,
    username: String,
    password: String,
    database: String
}

//S3 AND REDIS SERVER...

impl PostgresConfig {
    pub fn host(&self) -> &String {
        &self.host
    }
    pub fn port(&self) -> &i64 {
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
    pub fn conn_string(self) -> String {
        let conn_string = &format!("
            host={} 
            port={} 
            user={} 
            pass={} 
            dbname={}",
        &self.host(), 
        &self.port(), 
        &self.username(), 
        &self.password(), 
        &self.database());
        conn_string.to_string()
    }
}

#[derive(Clone)]
pub struct ExtraConfig {
    frontend_url: String
}

impl ExtraConfig {
    pub fn frontend_url(&self) -> &String {
        &self.frontend_url
    }
}

pub struct Config {
    postgres: PostgresConfig,
    extra: ExtraConfig
}

impl Default for Config {
    fn default() -> Self {
        let settings = ConfigAPI::builder()
            .add_source(config::File::with_name(SETTING_FILE))
            .build()
         .unwrap();
        Self { 
            postgres: PostgresConfig { 
                host: settings.get_string("postgres_host").unwrap(), 
                port: settings.get_int("postgres_port").unwrap(), 
                username: settings.get_string("postgres_username").unwrap(), 
                password: settings.get_string("postgres_password").unwrap(), 
                database: settings.get_string("postgres_database").unwrap()
            },
            extra: ExtraConfig { 
                frontend_url: settings.get_string("frontend_url").unwrap()
            }, 
        }
    }
}

impl Service<Config> {
    pub fn new() -> Service<Config> {
        let statistics: Vec<ServiceStats> = Vec::new();
        Service { 
            name: "Config".to_string(), 
            category: file!().to_string(), 
            status: true, 
            service: Config::default(), 
            statistics
        }
    }

    pub fn postgres(&mut self) -> PostgresConfig {
        self.add_usage(1);
        self.service.postgres.clone()
    }

    pub fn extra(&mut self) -> ExtraConfig {
        self.add_usage(1);
        self.service.extra.clone()
    } 
}