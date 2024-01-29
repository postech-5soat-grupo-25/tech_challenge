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
#[post("/pedido", data="<pedido_input>")]
async fn post_novo_pedido(
    pedido_e_pagamentos_use_case: &State<PedidosEPagamentosUseCase>,
    pedido_input: Json<CreatePedidoInput>,
    _logged_user_info: AuthenticatedUser,
) -> Result<Json<Pedido>, Status> {
    let novo_pedido = pedido_e_pagamentos_use_case.novo_pedido(pedido_input).await?;
    Ok(Json(novo_pedido))
}

#[openapi(tag = "Pedidos")]
#[get("/pedido/<id>")]
async fn get_pedido(pedido_e_pagamentos_use_case: &State<PedidosEPagamentosUseCase>, id: usize, _logged_user_info: AuthenticatedUser) -> Result<Json<Pedido>, Status> {
    let pedido = pedido_e_pagamentos_use_case.seleciona_pedido_por_id(id).await?;
    Ok(Json(pedido))
}

#[openapi(tag = "Pedidos")]
#[post("/pedido/acompanhamento/<pedido_id>/<acompanhamento_id>")]
async fn post_acompanhamentos(
    pedido_e_pagamentos_use_case: &State<PedidosEPagamentosUseCase>, 
    pedido_id: usize,
    acompanhamento_id: usize,
    _logged_user_info: AuthenticatedUser
) -> Result<Json<Pedido>, Status> {
    let pedido = pedido_e_pagamentos_use_case.adicionar_acompanhamento(pedido_id, acompanhamento_id).await?;
    Ok(Json(pedido))
}


#[openapi(tag = "Pedidos")]
#[post("/pedido/lanche/<pedido_id>/<lanche_id>")]
async fn post_lanches(
    pedido_e_pagamentos_use_case: &State<PedidosEPagamentosUseCase>, 
    pedido_id: usize,
    lanche_id: usize,
    _logged_user_info: AuthenticatedUser
) -> Result<Json<Pedido>, Status> {
    let pedido = pedido_e_pagamentos_use_case.adicionar_lanche(pedido_id, lanche_id).await?;
    Ok(Json(pedido))
}


#[openapi(tag = "Pedidos")]
#[post("/pedido/bebida/<pedido_id>/<bebida_id>")]
async fn post_bebidas(
    pedido_e_pagamentos_use_case: &State<PedidosEPagamentosUseCase>, 
    pedido_id: usize,
    bebida_id: usize,
    _logged_user_info: AuthenticatedUser
) -> Result<Json<Pedido>, Status> {
    let pedido = pedido_e_pagamentos_use_case.adicionar_bebida(pedido_id, bebida_id).await?;
    Ok(Json(pedido))
}

pub fn routes() -> Vec<rocket::Route> {
    openapi_get_routes![post_novo_pedido]
}
