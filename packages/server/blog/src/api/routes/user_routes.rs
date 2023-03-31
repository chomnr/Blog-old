use std::any::{type_name, Any};


/*
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

#[post("/create", format = "application/json", data = "<post_data>")]
async fn create_account(post_data: Json<CreateUser>, user: &State<Mutex<Service<User>>>, settings: &State<Config>) -> Result<Redirect, Value> {
    let mut lock = user.lock().await;
    let create = lock.create(&post_data.username, &post_data.password, &post_data.email).await;
    match create {
        Ok(_) => {
           //return Ok(Redirect::to(uri!("http://127.0.0.1:5173/")))
           return Ok(Redirect::temporary(uri!("http://127.0.0.1:5173/")))
        },
        Err(err) => {
            Err(json!({
                "message": err.to_string()
            }))
        }
    }
}

#[post("/login", format = "application/json", data = "<post_data>")]
async fn login_account(jar: &CookieJar<'_>, post_data: Json<LoginUser>, user: &State<Mutex<Service<User>>>, settings: &State<Config>) -> Result<Redirect, Value> {
    let mut lock = user.lock().await;
    let login = lock.login(&post_data.login, &post_data.password).await;
    match login {
        Ok(res) => {
            let cookie = Cookie::build("sid", serde_json::to_string(&res)
                .unwrap())
                .expires(Expiration::DateTime(OffsetDateTime::now_utc().saturating_add(Duration::days(7)))) // fix duration use global value todo()
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

#[get("/logout",)]
async fn logout_account(jar: &CookieJar<'_>, settings: &State<Config>) -> Redirect {
    jar.remove(Cookie::named("sid"));
    Redirect::to(settings.get_string("frontend_url").unwrap())
}


pub fn routes() -> Vec<Route> {
    routes![create_account, login_account, logout_account]
}
*/