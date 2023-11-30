use rocket::State;
use rocket::serde::json::Json;
use rocket_okapi::{openapi, openapi_get_routes};
use rocket::http::Status;

use crate::adapter::api::error_handling::ErrorResponse;
use crate::adapter::api::request_guards::authentication_guard::AuthenticatedUser;
use crate::core::application::use_cases::user_use_case::{CreateUserInput, UpdateUserInput};
use crate::core::{application::use_cases::user_use_case::UserUseCase, domain::entities::usuario::Usuario};

#[openapi(tag = "Users")]
#[get("/")]
async fn get_users(user_use_case: &State<UserUseCase>, _logged_user_info: AuthenticatedUser) -> Result<Json<Vec<Usuario>>, Status> {
    let users = user_use_case.get_users().await?;
    Ok(Json(users))
}

#[openapi(tag = "Users")]
#[get("/<id>")]
async fn get_user(user_use_case: &State<UserUseCase>, id: usize, _logged_user_info: AuthenticatedUser) -> Result<Json<Usuario>, Status> {
    let user = user_use_case.get_user_by_id(id).await?;
    Ok(Json(user))
}

#[openapi(tag = "Users")]
#[post("/", data = "<user_input>")]
async fn create_user(user_use_case: &State<UserUseCase>, user_input: Json<CreateUserInput>, _logged_user_info: AuthenticatedUser) -> Result<Json<Usuario>, Status> {
    let user_input = user_input.into_inner();
    let user = user_use_case.create_user(user_input).await?;
    Ok(Json(user))
}

#[openapi(tag = "Users")]
#[put("/<id>", data = "<update_user_input>")]
async fn update_user(user_use_case: &State<UserUseCase>, id: usize, update_user_input: Json<UpdateUserInput>, _logged_user_info: AuthenticatedUser) -> Result<Json<Usuario>, Status> {
    let update_user_input = update_user_input.into_inner();
    let user = user_use_case.update_user_info(id, update_user_input).await?;
    Ok(Json(user.to_owned()))
}

#[openapi(tag = "Users")]
#[delete("/<id>")]
async fn delete_user(user_use_case: &State<UserUseCase>, id: usize, _logged_user_info: AuthenticatedUser) -> Result<Json<String>, Status> {
    user_use_case.delete_user(id).await?;
    Ok(Json("success".to_string()))
}

pub fn routes() -> Vec<rocket::Route> {
    openapi_get_routes![get_users, get_user, create_user, update_user, delete_user]
}

#[catch(404)]
fn user_not_found() -> Json<ErrorResponse> {
    let error = ErrorResponse {
        msg: "User not found".to_string(),
        status: 404,
    };
    Json(error)
}

pub fn catchers() -> Vec<rocket::Catcher> {
    catchers![user_not_found]
}
