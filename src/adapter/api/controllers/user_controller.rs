use std::sync::Arc;

use rocket::State;
use rocket::serde::json::Json;
use rocket::tokio::sync::Mutex;
use rocket_okapi::{openapi, openapi_get_routes};
use rocket::http::Status;

use crate::core::application::use_cases::user_use_case::{CreateUserInput, self, UpdateUserInput};
use crate::core::domain::base::domain_error::DomainError;
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
pub async fn get_user(user_use_case: &State<UserUseCase>, id: usize) -> Result<Json<Usuario>, Status> {
    let user = user_use_case.get_user_by_id(id).await;
    match user {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::NotFound)
    }
}

#[openapi(tag = "Users")]
#[post("/", data = "<user_input>")]
pub async fn create_user(user_use_case: &State<UserUseCase>, user_input: Json<CreateUserInput>) -> Result<Json<Usuario>, Status> {
    let user_input = user_input.into_inner();
    let user = user_use_case.create_user(user_input).await;
    match user {
        Ok(user) => Ok(Json(user.to_owned())),
        Err(err) => match err {
            DomainError::AlreadyExists => Err(Status::Conflict),
            _ => Err(Status::InternalServerError)
        }
    }
}

#[openapi(tag = "Users")]
#[post("/<id>", data = "<update_user_input>")]
pub async fn update_user(user_use_case: &State<UserUseCase>, id: usize, update_user_input: Json<UpdateUserInput>) -> Result<Json<Usuario>, Status> {
    let update_user_input = update_user_input.into_inner();
    let user = user_use_case.update_user_info(id, update_user_input).await;
    match user {
        Ok(user) => Ok(Json(user.to_owned())),
        Err(err) => match err {
            DomainError::NotFound => Err(Status::NotFound),
            DomainError::AlreadyExists => Err(Status::Conflict),
            _ => Err(Status::InternalServerError)
        }
    }
}

#[openapi(tag = "Users")]
#[delete("/<id>")]
pub async fn delete_user(user_use_case: &State<UserUseCase>, id: usize) -> Status {
    let user = user_use_case.delete_user(id).await;
    match user {
        Ok(()) => Status::Ok,
        Err(err) => match err {
            DomainError::NotFound => Status::NotFound,
            DomainError::AlreadyExists => Status::Conflict,
            _ => Status::InternalServerError
        }
    }
}

pub fn routes() -> Vec<rocket::Route> {
    openapi_get_routes![get_users, get_user, create_user, update_user, delete_user]
}
