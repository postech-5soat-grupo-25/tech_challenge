use std::sync::Arc;

use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use rocket_okapi::{openapi, openapi_get_routes};
use tokio::sync::Mutex;

use crate::controllers::auth_controller::{
    AuthController,
    LoginInput,
    AuthenticationResponse,
};

use crate::traits::authentication_adapter::AuthenticationAdapter;
use crate::traits::usuario_repository::UsuarioRepository;


#[openapi(tag = "Auth")]
#[post("/login", data = "<login_input>")]
async fn login(
    usuario_repository: &State<Arc<Mutex<dyn UsuarioRepository + Send + Sync>>>,
    authentication_adapter: &State<Arc<dyn AuthenticationAdapter + Sync + Send>>,
    login_input: Json<LoginInput>,
) -> Result<Json<AuthenticationResponse>, Status> {
    let usuario_repository = usuario_repository.inner().clone();
    let authentication_adapter = authentication_adapter.inner().clone();
    let auth_controller = AuthController::new(usuario_repository, authentication_adapter);
    let login_input = login_input.into_inner();
    let authentication_response = auth_controller.login(login_input).await?;
    Ok(Json(authentication_response))
}

pub fn routes() -> Vec<rocket::Route> {
    openapi_get_routes![login]
}
