use rocket::error::ErrorKind;
use rocket::{State, Error};
use rocket::serde::json::Json;
use rocket_okapi::{openapi, openapi_get_routes};
use rocket::http::Status;

use crate::core::{application::use_cases::user_use_case::UserUseCase, domain::entities::usuario::Usuario};

#[openapi(tag = "Users")]
#[get("/")]
pub async fn get_users(user_use_case: &State<UserUseCase>) -> Result<Json<Vec<Usuario>>, Status> {
    let users = user_use_case.get_users().await;
    match users {
        Ok(users) => Ok(Json(users)),
        Err(_) => Err(Status::NotFound)
    }
}

#[openapi(tag = "Users")]
#[get("/<id>")]
pub async fn get_user(user_use_case: &State<UserUseCase>, id: i32) -> Result<Json<Usuario>, Status> {
    let user = user_use_case.get_user_by_id(id).await;
    match user {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::NotFound)
    }
}

pub fn routes() -> Vec<rocket::Route> {
    openapi_get_routes![get_users, get_user]
}
