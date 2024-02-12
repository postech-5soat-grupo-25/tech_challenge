use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use rocket_okapi::{openapi, openapi_get_routes};

use crate::adapter::api::error_handling::ErrorResponse;
use crate::adapter::api::request_guards::admin_guard::AdminUser;
use crate::core::domain::entities::usuario::Usuario;
use crate::core::domain::value_objects::cpf::Cpf;
use crate::core::application::use_cases::gerenciamento_de_usuarios_use_case::UsuarioUseCase;
use crate::core::application::use_cases::gerenciamento_de_usuarios_use_case::CreateUsuarioInput;


#[openapi(tag = "Usuarios")]
#[get("/")]
async fn get_usuarios(
    usuario_use_case: &State<UsuarioUseCase>,
    _logged_user_info: AdminUser,
) -> Result<Json<Vec<Usuario>>, Status> {
    let usuarios = usuario_use_case.get_usuarios().await?;
    Ok(Json(usuarios))
}

#[openapi(tag = "Usuarios")]
#[get("/<id>")]
async fn get_usuario(
    usuario_use_case: &State<UsuarioUseCase>,
    id: usize,
    _logged_user_info: AdminUser,
) -> Result<Json<Usuario>, Status> {
    let usuario = usuario_use_case.get_usuario_by_id(id).await?;
    Ok(Json(usuario))
}

#[openapi(tag = "Usuarios")]
#[post("/", data = "<usuario_input>")]
async fn create_usuario(
    usuario_use_case: &State<UsuarioUseCase>,
    usuario_input: Json<CreateUsuarioInput>,
    _logged_user_info: AdminUser,
) -> Result<Json<Usuario>, Status> {
    let usuario_input: CreateUsuarioInput = usuario_input.into_inner();
    let usuario = usuario_use_case.create_usuario(usuario_input).await?;
    Ok(Json(usuario))
}

#[openapi(tag = "Usuarios")]
#[put("/<id>", data = "<usuario_input>")]
async fn update_usuario(
    usuario_use_case: &State<UsuarioUseCase>,
    usuario_input: Json<CreateUsuarioInput>,
    id: usize,
    _logged_user_info: AdminUser,
) -> Result<Json<Usuario>, Status> {
    let usuario_input: CreateUsuarioInput = usuario_input.into_inner();
    let usuario = usuario_use_case.update_usuario(id, usuario_input).await?;
    Ok(Json(usuario))
}

#[openapi(tag = "Usuarios")]
#[delete("/<cpf>")]
async fn delete_usuario(
    usuario_use_case: &State<UsuarioUseCase>,
    cpf: Cpf,
    _logged_user_info: AdminUser,
) -> Result<Json<String>, Status> {
    usuario_use_case.delete_usuario(cpf).await?;
    Ok(Json("success".to_string()))
}

pub fn routes() -> Vec<rocket::Route> {
    openapi_get_routes![get_usuarios, get_usuario, create_usuario, update_usuario, delete_usuario]
}

#[catch(404)]
fn usuario_not_found() -> Json<ErrorResponse> {
    let error = ErrorResponse {
        msg: "Usuário não encontrado!".to_string(),
        status: 404,
    };
    Json(error)
}

pub fn catchers() -> Vec<rocket::Catcher> {
    catchers![usuario_not_found]
}
