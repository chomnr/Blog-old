use std::time::{SystemTime, UNIX_EPOCH};

use tokio_postgres::{types::ToSql, Row};

use crate::api::error::PostError;

use super::{Service, ServiceStats};

pub struct Post {
    pool: deadpool_postgres::Pool
}


impl Service<Post> {
    // Post Constraints
    const POST_TITLE_MIN: usize = 20;
    const POST_TITLE_MAX: usize = 70;
    const POST_CONTENT_MIN: usize = 20;
    const POST_CONTENT_MAX: usize = 100000; // 100k character limit..

    pub fn new(pool: deadpool_postgres::Pool) -> Self {
        let statistics: Vec<ServiceStats> = Vec::new();
        Service { 
            name: "Post".to_string(), 
            category: file!().to_string(), 
            status: true, 
            service: Post {
                pool
            }, 
            statistics
        }
    }

    pub async fn create(&self, author_uid: &str, title: &str, content: &str) -> Result<(), PostError> {
        // Calling the procedures and or constraints.
        Self::title_proc(title)?;
        Self::content_proc(content)?;
        // Specifies the SQL statement that will be executed to perform the desired action.
        let sql = format!("INSERT INTO posts (uid, title, content, created_on)  VALUES ($1, $2, $3, $4)");
        // Current time
        let current_time =  SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64;
        // Executing the query.
        self.short_query(sql.as_str(), 
            &[
                &author_uid, 
                &title, 
                &content, 
                &current_time
                ]).await.unwrap();
        Ok(())
    }

    pub fn title_proc(title: &str) -> Result<(), PostError> {
        // Verifies whether the length of the password is 
        // below the prescribed minimum.
        if title.len() < Self::POST_TITLE_MIN {
            return Err(PostError::PostViolation)
        }
        // Verifies whether the length of the username exceeds 
        // the permissible maximum value.
        if title.len() > Self::POST_TITLE_MAX {
            return Err(PostError::PostViolation)
        }
        Ok(())
    }

    pub fn content_proc(title: &str) -> Result<(), PostError> {
        // Verifies whether the length of the password is 
        // below the prescribed minimum.
        if title.len() < Self::POST_CONTENT_MIN {
            return Err(PostError::PostViolation)
        }
        // Verifies whether the length of the username exceeds 
        // the permissible maximum value.
        if title.len() > Self::POST_CONTENT_MAX {
            return Err(PostError::PostViolation)
        }
        Ok(())
    }

    /// This function encapsulates the existing postgres query 
    /// to streamline the requisite procedures for executing 
    /// a query. 
    async fn short_query(&self, sql: &str, params: &[&(dyn ToSql + Sync)]) -> Result<Vec<Row>, tokio_postgres::Error> {
        let conn = &self.service.pool.get().await.unwrap();
        // Prepare the query.
        let statement = conn.prepare(sql).await.unwrap();
        // Execute query.
        match conn.query(&statement, params).await {
            Ok(v) => {
                Ok(v)
            },
            Err(er) => {
                return Err(er)
            },
        }
    }
}