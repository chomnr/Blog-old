use rocket::{Route, routes, post, http::Status, request::{FromRequest, Outcome}, State, futures::lock::Mutex, Response, response::{content, Redirect}, serde::{json::{Json, Value}, self}};
use crate::api::services::{User, Service};
use rocket::serde::json::json;

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(crate = "rocket::serde")]
struct CreateUser {
    username: String,
    password: String,
    email: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(crate = "rocket::serde")]
struct LoginUser {
    login: String,
    password: String,
}

// login method... with cookie...
#[post("/create", format = "json", data = "<post_data>")]
async fn create_account(post_data: Json<CreateUser>, user: &State<Mutex<Service<User>>>) -> Result<Redirect, ()> {
    let create = user.lock().await.create(&post_data.username, &post_data.password, &post_data.email).await;
    // add messages...
    match create {
        Ok(v) => Ok(Redirect::to("/")), //todo
        Err(_) => Err(())
    }
}

#[post("/login", format = "json", data = "<post_data>")]
async fn login_account(post_data: Json<LoginUser>, user: &State<Mutex<Service<User>>>) -> Result<Redirect, ()> {
    //let create = user.lock().await.login(login, password)
    // add messages...
    match create {
        Ok(v) => Ok(Redirect::to("/")), //todo
        Err(_) => Err(())
    }
}

pub fn routes() -> Vec<Route> {
    routes![create_account, login_account]
}