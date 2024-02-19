use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::entities::cpf::Cpf;
use crate::use_cases::gerenciamento_de_usuarios_use_case::UsuarioUseCase;
use crate::entities::usuario::Usuario;

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

pub async fn login(
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
