use std::time::{SystemTime, UNIX_EPOCH};

use rocket::serde::{self, Deserialize, Serialize, json::{Value, self}};
use tokio_postgres::{types::ToSql, Row};

use crate::api::error::PostError;

use super::{Service, ServiceStats};

pub struct Post {
    pool: deadpool_postgres::Pool
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct PostEntry {
    blog_id: i32,
    uid: String,
    author: String,
    title: String,
    created_on: i64
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct PostSingleEntry {
    uid: String,
    title: String,
    content: String,
    created_on: i64,
    author: String
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

    pub async fn entries(&self) -> Value {
        //let mut post_entries: PostInfo = vec![];
        //let sql = "SELECT posts.id, posts.uid, posts.title, posts.content, users.username FROM posts JOIN users ON posts.uid = users.uid";
        let sql = "SELECT posts.id, posts.uid, posts.title, users.username, posts.created_on FROM posts JOIN users ON posts.uid = users.uid";
        let rows = self.short_query(sql, &[]).await.unwrap();
        let mut posts: Vec<PostEntry> = Vec::new();
        for row in rows {
            let blog_id: i32 = row.get(0);
            let uid: String = row.get(1);
            let title: String = row.get(2);
            let author: String = row.get(3);
            let created_on: i64 = row.get(4);
            let post_info = PostEntry {
                blog_id,
                uid, 
                title,
                author,
                created_on
            };
            posts.push(post_info);
        }
        serde::json::to_value(&posts).unwrap()
    }

    pub async fn entry(&self, id: i32) -> Result<Value, PostError> {
        let sql = "SELECT posts.uid, posts.title, posts.content, posts.created_on, users.username FROM posts JOIN users on posts.uid = users.uid WHERE id = $1";
        
        let row = self.short_query(sql, &[&id]).await.unwrap();
        if row.len() < 1 {
            return Err(PostError::InvalidPostId);
        }
        let row = self.short_query(sql, &[&id]).await.unwrap();

        let uid: String = row[0].get(0);
        let title: String = row[0].get(1);
        let content: String = row[0].get(2);
        let created_on: i64 = row[0].get(3);
        let author: String = row[0].get(4);
        let post = PostSingleEntry {
            uid,
            title,
            content,
            created_on,
            author
        };

        Ok(json::to_value(&post).unwrap())
        /* 
        let row = self.short_query(sql, &[&id]).await.unwrap();
        
        let uid: String = row[0].get(0);
        let title: String = row[0].get(1);
        let content: String = row[0].get(2);
        let created_on: i64 = row[0].get(3);
        let author: String = row[0].get(4);

        let post = PostSingleEntry {
            uid,
            title,
            content,
            created_on,
            author
        };

        json::to_string(&post).unwrap()
        */
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