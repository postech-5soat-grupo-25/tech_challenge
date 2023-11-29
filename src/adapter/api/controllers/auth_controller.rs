use rocket::State;
use rocket::serde::json::Json;
use rocket_okapi::{openapi, openapi_get_routes};
use rocket::http::Status;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::adapter::api::error_handling::ErrorResponse;
use crate::adapter::api::helpers::auth_helper::get_token;
use crate::core::domain::value_objects::cpf::Cpf;
use crate::core::{application::use_cases::user_use_case::UserUseCase, domain::entities::usuario::Usuario};

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
struct LoginInput {
    cpf: String,
    senha: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
struct AuthenticationResponse {
    token: String,
    user: Usuario,
}

#[openapi(tag = "Auth")]
#[post("/login", data = "<login_input>")]
async fn login(user_use_case: &State<UserUseCase>, login_input: Json<LoginInput>) -> Result<Json<AuthenticationResponse>, Status> {
    let login_input = login_input.into_inner();
    let cpf = Cpf::new(login_input.cpf.clone());
    let user = user_use_case.get_user_by_cpf(cpf).await?;
    let token = get_token(user.clone())?;
    let response = AuthenticationResponse {
        token: token,
        user: user,
    };
    Ok(Json(response))
}

pub fn routes() -> Vec<rocket::Route> {
    openapi_get_routes![login]
}

#[catch(401)]
fn forbidden() -> Json<ErrorResponse> {
    let error = ErrorResponse {
        msg: "Credenciais invalidas".to_string(),
        status: 401,
    };
    Json(error)
}

pub fn catchers() -> Vec<rocket::Catcher> {
    catchers![forbidden]
}
