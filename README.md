# BLOG <br>
Simple but scalable blog system written in Rust & React

## ENDPOINTS

### User authentication
/api/user/create <- Create an account<br>
/api/user/login <- Login to an account<br>
### Create and delete posts
/api/post/create <- Create a post<br>
/api/post/entries <- List all posts<br>
/api/post/entries/{id} <- Retrieve a specific post<br>


Table creation queries
CREATE TABLE posts (
    id SERIAL PRIMARY KEY,
    uid VARCHAR(255) NOT NULL,
    title VARCHAR(255) NOT NULL,
    content VARCHAR(100000) NOT NULL,
    created_on BIGINT NOT NULL
)
CREATE TABLE sessions (
    uid VARCHAR(255) PRIMARY KEY NOT NULL,
    sid VARCHAR(255) UNIQUE NOT NULL,
    expires_on BIGINT NOT NULL
)

CREATE TABLE users (
    uid VARCHAR(255) PRIMARY KEY,
    username citext UNIQUE NOT NULL,
    email citext UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    password_salt VARCHAR(255) NOT NULL
);
