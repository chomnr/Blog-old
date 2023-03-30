use std::any::{type_name, Any};

use config::Config;
use rocket::{Route, routes, post, State, futures::lock::Mutex, serde::{json::{Json, Value}, self}, response::Redirect, http::{Cookie, private::cookie::Expiration, SameSite, CookieJar}, time::{Duration, OffsetDateTime}};
use crate::api::{services::{User, Service}};
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
async fn create_account(post_data: Json<CreateUser>, user: &State<Mutex<Service<User>>>, settings: &State<Config>) -> Result<Redirect, Value> {
    let mut lock = user.lock().await;
    let create = lock.create(&post_data.username, &post_data.password, &post_data.email).await;
    match create {
        Ok(_) => {
            Ok(Redirect::to(settings.get_string("frontend_url").unwrap()))
        },
        Err(err) => {
            // Catch the error and return a custom JSON response
            Err(json!({
                "message": err.to_string()
            }))
        }
    }
}

#[post("/login", format = "json", data = "<post_data>")]
async fn login_account(jar: &CookieJar<'_>, post_data: Json<LoginUser>, user: &State<Mutex<Service<User>>>, settings: &State<Config>) -> Result<Redirect, Value> {
    let mut lock = user.lock().await;
    let login = lock.login(&post_data.login, &post_data.password).await;
    match login {
        Ok(res) => {
            let cookie = Cookie::build("sid", serde_json::to_string(&res)
                .unwrap())
                .expires(Expiration::DateTime(OffsetDateTime::now_utc().saturating_add(Duration::days(7)))) // fix duration make
                .same_site(SameSite::None)
                .finish();
            jar.add(cookie);
            Ok(Redirect::to(settings.get_string("frontend_url").unwrap()))
        },
        Err(err) => {
            Err(json!({
                "message": err.to_string()
            }))
        }
    }
}
/*
#[post("/login", format = "json", data = "<post_data>")]
async fn login_account(cookies: &CookieJar<'_>, post_data: Json<LoginUser>, user: &State<Mutex<Service<User>>>) -> Option<String>  {
    let login = post_data.0.login.as_str();
    let password = post_data.0.password.as_str();
    let login_meth = user.lock().await.login(login, password).await.unwrap();
    let cookie = Cookie::build("sid", serde_json::to_string(&login_meth)
                    .unwrap())
                    .expires(Expiration::DateTime(OffsetDateTime::now_utc().saturating_add(Duration::days(7))))
                    .same_site(SameSite::None)
                    .finish();
    cookies.add(cookie);
    cookies.get("sid").map(|crumb| format!("Message: {}", crumb.value()))
}
*/


pub fn routes() -> Vec<Route> {
    routes![create_account, login_account]
}