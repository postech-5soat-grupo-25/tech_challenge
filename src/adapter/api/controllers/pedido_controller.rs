use rocket::http::Status;
use rocket::response::status::NotFound;
use rocket::request::FromParam;
use rocket::serde::json::Json;
use rocket::State;
use rocket_okapi::{openapi, openapi_get_routes};

use crate::adapter::api::error_handling::ErrorResponse;
use crate::adapter::api::request_guards::authentication_guard::AuthenticatedUser;
use crate::core::application::use_cases::preparation_and_deliver_use_case::PreparationAndDeliverUseCase;
use crate::core::domain::entities::pedido::Pedido;

#[openapi(tag = "Pedidos")]
#[get("/pedido_novos")]
async fn get_pedidos_novos(
    preparation_and_deliver_use_case: &State<PreparationAndDeliverUseCase>,
    _logged_user_info: AuthenticatedUser,
) -> Result<Json<Vec<Pedido>>, Status> {
    let pedidos_novos = preparation_and_deliver_use_case.get_pedidos_novos().await?;
    Ok(Json(pedidos_novos))
}

#[openapi(tag = "Pedidos")]
#[put("/<id>/status/<status>")]
async fn update_status_pedido(
    preparation_and_deliver_use_case: &State<PreparationAndDeliverUseCase>,
    id: usize,
    status: String,
    _logged_user_info: AuthenticatedUser,
) -> Result<Json<Pedido>, Status> {
    let pedido;
    match status.as_str() {
        "em_preparacao" => pedido = preparation_and_deliver_use_case.set_pedido_em_preparacao(id).await?,
        "pronto" => pedido = preparation_and_deliver_use_case.set_pedido_pronto(id).await?,
        "finalizado" => pedido = preparation_and_deliver_use_case.set_pedido_finalizado(id).await?,
        "set_pedido_cancelado" => pedido = preparation_and_deliver_use_case.set_pedido_cancelado(id).await?,
        &_ => todo!(),
    };
    Ok(Json(pedido))
}

pub fn routes() -> Vec<rocket::Route> {
    openapi_get_routes![get_pedidos_novos, update_status_pedido]
}

#[catch(404)]
fn pedido_not_found() -> Json<ErrorResponse> {
    let error = ErrorResponse {
        msg: "Pedido não encontrado!".to_string(),
        status: 404,
    };
    Json(error)
}

pub fn catchers() -> Vec<rocket::Catcher> {
    catchers![pedido_not_found]
}
