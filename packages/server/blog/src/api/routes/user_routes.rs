use rocket::{Route, routes, post, http::{Status, CookieJar, Cookie, private::cookie::Expiration, SameSite}, request::{FromRequest, Outcome}, State, futures::lock::Mutex, Response, response::{content, Redirect, status}, serde::{json::{Json, Value}, self}, time::{OffsetDateTime, Duration}};
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
async fn create_account(post_data: Json<CreateUser>, user: &State<Mutex<Service<User>>>) -> Value {
    let mut lock = user.lock().await;
    let result = lock.create(&post_data.username, &post_data.password, &post_data.email).await;

    match result {
        Ok(_) => json!({"status": "SUCCESS"}),
        Err(err) => {
            // Catch the error and return a custom JSON response
            json!({
                "status": "FAILED",
                "reason": err.to_string()
            })
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
    routes![create_account]
}