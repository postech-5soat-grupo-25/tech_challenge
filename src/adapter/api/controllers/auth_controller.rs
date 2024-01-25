use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use rocket_okapi::{openapi, openapi_get_routes};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::adapter::api::helpers::auth_helper::get_token;
use crate::core::domain::value_objects::cpf::Cpf;
use crate::core::{
    application::use_cases::usuario_use_case::UsuarioUseCase, domain::entities::usuario::Usuario,
};

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
struct LoginInput {
    cpf: String,
    senha: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
struct AuthenticationResponse {
    token: String,
    usuario: Usuario,
}

#[openapi(tag = "Auth")]
#[post("/login", data = "<login_input>")]
async fn login(
    usuario_use_case: &State<UsuarioUseCase>,
    login_input: Json<LoginInput>,
) -> Result<Json<AuthenticationResponse>, Status> {
    let login_input = login_input.into_inner();
    let cpf = Cpf::new(login_input.cpf.clone())?;
    let usuario = usuario_use_case.get_usuario_by_cpf(cpf).await;
    match usuario {
        Ok(usuario) => {
            if !usuario.validate_senha(&login_input.senha) {
                return Err(Status::Unauthorized);
            }
            let token = get_token(usuario.clone())?;
            let response = AuthenticationResponse {
                token,
                usuario,
            };
            Ok(Json(response))
        }
        Err(_) => return Err(Status::Unauthorized),
    }
}

pub fn routes() -> Vec<rocket::Route> {
    openapi_get_routes![login]
}
