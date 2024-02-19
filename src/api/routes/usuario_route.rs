use std::sync::Arc;

use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use rocket_okapi::{openapi, openapi_get_routes};
use tokio::sync::Mutex;

use crate::api::error_handling::ErrorResponse;
use crate::api::request_guards::admin_guard::AdminUser;
use crate::controllers::usuario_controller::UsuarioController;
use crate::entities::usuario::Usuario;
use crate::entities::cpf::Cpf;
use crate::traits::usuario_repository::UsuarioRepository;
use crate::use_cases::gerenciamento_de_usuarios_use_case::CreateUsuarioInput;


#[openapi(tag = "Usuarios")]
#[get("/")]
async fn get_usuarios(
    usuario_repository: &State<Arc<Mutex<dyn UsuarioRepository + Sync + Send>>>,
    _logged_user_info: AdminUser,
) -> Result<Json<Vec<Usuario>>, Status> {
    let usuario_controller = UsuarioController::new(usuario_repository.inner().clone());
    let usuarios = usuario_controller.get_usuarios().await?;
    Ok(Json(usuarios))
}

#[openapi(tag = "Usuarios")]
#[get("/<id>")]
async fn get_usuario(
    usuario_repository: &State<Arc<Mutex<dyn UsuarioRepository + Sync + Send>>>,
    id: usize,
    _logged_user_info: AdminUser,
) -> Result<Json<Usuario>, Status> {
    let usuario_controller = UsuarioController::new(usuario_repository.inner().clone());
    let usuario = usuario_controller.get_usuario(id).await?;
    Ok(Json(usuario))
}

#[openapi(tag = "Usuarios")]
#[post("/", data = "<usuario_input>")]
async fn create_usuario(
    usuario_repository: &State<Arc<Mutex<dyn UsuarioRepository + Sync + Send>>>,
    usuario_input: Json<CreateUsuarioInput>,
    _logged_user_info: AdminUser,
) -> Result<Json<Usuario>, Status> {
    let usuario_controller = UsuarioController::new(usuario_repository.inner().clone());
    let usuario_input: CreateUsuarioInput = usuario_input.into_inner();
    let usuario = usuario_controller.create_usuario(usuario_input).await?;
    Ok(Json(usuario))
}

#[openapi(tag = "Usuarios")]
#[put("/<id>", data = "<usuario_input>")]
async fn update_usuario(
    usuario_repository: &State<Arc<Mutex<dyn UsuarioRepository + Sync + Send>>>,
    usuario_input: Json<CreateUsuarioInput>,
    id: usize,
    _logged_user_info: AdminUser,
) -> Result<Json<Usuario>, Status> {
    let usuario_controller = UsuarioController::new(usuario_repository.inner().clone());
    let usuario_input: CreateUsuarioInput = usuario_input.into_inner();
    let usuario = usuario_controller.update_usuario(id, usuario_input).await?;
    Ok(Json(usuario))
}

#[openapi(tag = "Usuarios")]
#[delete("/<cpf>")]
async fn delete_usuario(
    usuario_repository: &State<Arc<Mutex<dyn UsuarioRepository + Sync + Send>>>,
    cpf: Cpf,
    _logged_user_info: AdminUser,
) -> Result<Json<String>, Status> {
    let usuario_controller = UsuarioController::new(usuario_repository.inner().clone());
    usuario_controller.delete_usuario(cpf).await?;
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
