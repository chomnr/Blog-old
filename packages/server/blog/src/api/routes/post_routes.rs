use base64::Engine;
use base64::engine::general_purpose;
use rocket::Route;
use rocket::State;
use rocket::data::Limits;
use rocket::get;
use rocket::http::CookieJar;
use rocket::http::Status;
use rocket::post;
use rocket::routes;
use rocket::serde;
use rocket::serde::Deserialize;
use rocket::serde::Serialize;
use rocket::serde::json::Json;
use rocket::serde::json::Value;
use rocket::serde::json::serde_json::json;

use crate::api::services::Post;
use crate::api::services::Service;
use crate::api::services::User;
use crate::api::services::UserSession;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreatePost {
    title: String,
    content: String
}

#[post("/create", format = "application/json", data = "<post_data>")]
async fn create_post(jar: &CookieJar<'_>, post_data: Json<CreatePost>, user: &State<Service<User>>, post: &State<Service<Post>>) -> (Status, Value) {
    let sid = jar.get("sid");    
    if sid.is_none() {
        return (Status::Unauthorized, json!({"message": "You're not authorized to perform this action."}))
    } else {
        let session: UserSession = serde::json::from_str(sid.unwrap().value()).unwrap();
        match user.session_validate(&session.sid).await {
            Ok(()) => {
                let post_content = general_purpose::STANDARD.encode(&post_data.content);
                let post = post.create(&session.uid, &post_data.title, &post_content).await;
                match post {
                    Ok(_) => return (Status::Ok, json!({"message": "Success"})),
                    Err(er) => return (Status::Unauthorized, json!({"message": er.to_string()})),
                }
            },
            Err(_) => {
                return (Status::Unauthorized, json!({"message": "You're not authorized to perform this action."}))
            }
        }
    }
}

#[get("/entries")]
async fn get_entries() -> (Status, Value) {
    todo!()
}

pub fn routes() -> Vec<Route> {
    routes![create_post, get_entries]
}