use std::sync::Arc;

use rocket::http::Status;
use rocket::request::FromParam;
use rocket::serde::json::Json;
use rocket::State;
use rocket_okapi::{openapi, openapi_get_routes};
use tokio::sync::Mutex;

use crate::api::error_handling::ErrorResponse;
use crate::api::request_guards::authentication_guard::AuthenticatedUser;
use crate::controllers::cliente_controller::ClienteController;
use crate::traits::cliente_repository::ClienteRepository;
use crate::use_cases::gerenciamento_de_clientes_use_case::CreateClienteInput;
use crate::entities::cliente::Cliente;
use crate::entities::cpf::Cpf;

impl<'a> FromParam<'a> for Cpf {
    type Error = String;
    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        Cpf::new(param.to_string()).map_err(|_| format!("CPF inválido: {}", param))
    }
}

#[openapi(tag = "Clientes")]
#[get("/")]
async fn lista_clientes(
    cliente_repository: &State<Arc<Mutex<dyn ClienteRepository + Sync + Send>>>,
    _logged_user_info: AuthenticatedUser,
) -> Result<Json<Vec<Cliente>>, Status> {
    let cliente_controller = ClienteController::new(cliente_repository.inner().clone());
    let clientes = cliente_controller.lista_clientes().await?;
    Ok(Json(clientes))
}

#[openapi(tag = "Clientes")]
#[get("/<cpf>")]
async fn busca_cliente_por_cpf(
    cliente_repository: &State<Arc<Mutex<dyn ClienteRepository + Sync + Send>>>,
    cpf: Cpf,
) -> Result<Json<Cliente>, Status> {
    let cliente_controller = ClienteController::new(cliente_repository.inner().clone());
    let cliente = cliente_controller.busca_cliente_por_cpf(cpf).await?;
    Ok(Json(cliente))
}

#[openapi(tag = "Clientes")]
#[post("/", data = "<cliente_input>")]
async fn cadastro_cliente(
    cliente_repository: &State<Arc<Mutex<dyn ClienteRepository + Sync + Send>>>,
    cliente_input: Json<CreateClienteInput>,
) -> Result<Json<Cliente>, Status> {
    let cliente_controller = ClienteController::new(cliente_repository.inner().clone());
    let cliente_input = cliente_input.into_inner();
    let cliente = cliente_controller.cadastro_cliente(cliente_input).await?;
    Ok(Json(cliente))
}

pub fn routes() -> Vec<rocket::Route> {
    openapi_get_routes![lista_clientes, busca_cliente_por_cpf, cadastro_cliente]
}

#[catch(404)]
fn cliente_not_found() -> Json<ErrorResponse> {
    let error = ErrorResponse {
        msg: "Cliente não encontrado!".to_string(),
        status: 404,
    };
    Json(error)
}

pub fn catchers() -> Vec<rocket::Catcher> {
    catchers![cliente_not_found]
}
