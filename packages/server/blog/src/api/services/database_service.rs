use deadpool_postgres::{ManagerConfig, RecyclingMethod, Manager, Pool};
use tokio_postgres::NoTls;

use super::{Service, ServiceStats, config_service::PostgresConfig};

pub struct Postgres<'a> {
    config: &'a PostgresConfig
}

impl<'a> Service<Postgres<'a>> {
    pub fn new(config: &'a PostgresConfig) -> Self {
        let statistics: Vec<ServiceStats> = Vec::new();
        Service {
            name: "Database".to_string(),
            category: file!().to_string(),
            status: true,
            service: Postgres { 
                config, 
            },
            statistics
        }
    }

    pub fn new_pool(&self) -> deadpool_postgres::Pool {
        let (pg_cfg, mgr_cfg) = self.new_config(RecyclingMethod::Fast);
        let pg_mgr = Manager::from_config(pg_cfg.to_owned(), NoTls, mgr_cfg);
        Pool::builder(pg_mgr).max_size(16).build().unwrap()
    }

    fn new_config(&self, recycling_method: RecyclingMethod) -> (tokio_postgres::Config, ManagerConfig) {
        let env = self.service.config;
        let mut pg_cfg = tokio_postgres::Config::new();
        pg_cfg.host(&env.host().to_string());
        pg_cfg.port(*env.port());
        pg_cfg.user(&env.username());
        pg_cfg.password(&env.password());
        pg_cfg.dbname(&env.database());
        let pg_mgr_config = ManagerConfig {
            recycling_method
        };
        (pg_cfg, pg_mgr_config)
    }
}

