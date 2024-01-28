use rocket::http::Status;
use rocket::response::status::NotFound;
use rocket::request::FromParam;
use rocket::serde::json::Json;
use rocket::State;
use rocket_okapi::{openapi, openapi_get_routes};

use crate::adapter::api::error_handling::ErrorResponse;
use crate::adapter::api::request_guards::authentication_guard::AuthenticatedUser;
use crate::core::application::use_cases::pedidos_e_pagamentos_use_case::{PedidosEPagamentosUseCase, CreatePedidoInput};
use crate::core::domain::entities::pedido::Pedido;


#[openapi(tag = "Pedidos")]
#[get("/pedido/<id>")]
async fn get_pedido(pedido_e_pagamentos_use_case: &State<PedidosEPagamentosUseCase>, id: usize, _logged_user_info: AuthenticatedUser) -> Result<Json<Pedido>, Status> {
    let pedido = pedido_e_pagamentos_use_case.get_order_by_id(id).await?;
    Ok(Json(pedido))
}


#[openapi(tag = "Pedidos")]
#[post("/pedido", data="<pedido_input>")]
async fn post_novo_pedido(
    pedido_e_pagamentos_use_case: &State<PedidosEPagamentosUseCase>,
    pedido_input: Json<CreatePedidoInput>,
    _logged_user_info: AuthenticatedUser,
) -> Result<Json<Pedido>, Status> {
    let novo_pedido = pedido_e_pagamentos_use_case.novo_pedido(pedido_input).await?;
    Ok(Json(novo_pedido))
}

pub fn routes() -> Vec<rocket::Route> {
    openapi_get_routes![post_novo_pedido]
}
