#![feature(type_ascription)]
use std::{any::{type_name, Any}, sync::Mutex};

use rocket::{serde::{Serialize, Deserialize, json::{Json, Value, serde_json::{json, self}}, self}, State, post, Route, http::{private::cookie::Expiration, Cookie, CookieJar, SameSite, Status}, time::{OffsetDateTime, Duration}};

use crate::api::services::Service;
use crate::User;

use rocket::routes;

#[derive(Serialize, Deserialize)]
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

#[post("/create", format = "application/json", data = "<post_data>")]
async fn create_account(post_data: Json<CreateUser>, user: &State<Service<User>>) -> (Status, Value) {
    let create = user.create(&post_data.username, &post_data.password, &post_data.email).await;
    match create {
        Ok(_) => (Status::Ok, json!({"message": "Success"})),
        Err(err) => (Status::Conflict, json!({"message": err.to_string()}))
    }
}

#[post("/login", format = "application/json", data = "<post_data>")]
async fn login_account(jar: &CookieJar<'_>, post_data: Json<LoginUser>, user: &State<Service<User>>) -> Result<(Status, Value), (Status, Value)> {
    let login = user.login(&post_data.login, &post_data.password).await;
    match login {
        Ok(res) => {
            let cookie = Cookie::build("sid", serde_json::to_string(&res)
                .unwrap())
                .same_site(SameSite::None)
                .expires(Expiration::DateTime(OffsetDateTime::now_utc().saturating_add(Duration::days(7))))
                .secure(false) // enable secure if you're using https...
                .finish();
            jar.add(cookie);
            Ok((Status::Ok, json!({"message": "Success"})))
        },
        Err(err) => {
            Err((Status::Conflict, json!({"message": err.to_string()})))
        }
    }
}

pub fn routes() -> Vec<Route> {
    routes![create_account, login_account]
}