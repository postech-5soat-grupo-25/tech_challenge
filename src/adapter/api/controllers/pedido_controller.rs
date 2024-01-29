use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use rocket_okapi::{openapi, openapi_get_routes};

use crate::adapter::api::error_handling::ErrorResponse;
use crate::adapter::api::request_guards::authentication_guard::AuthenticatedUser;
use crate::core::application::use_cases::preparacao_e_entrega_use_case::PreparacaoeEntregaUseCase;
use crate::core::domain::entities::pedido::Pedido;
use crate::core::application::use_cases::pedidos_e_pagamentos_use_case::{PedidosEPagamentosUseCase, CreatePedidoInput};

#[openapi(tag = "Pedidos")]
#[get("/pedido")]
async fn get_pedidos(
    pedidos_e_pagamentos_use_case: &State<PedidosEPagamentosUseCase>,
    _logged_user_info: AuthenticatedUser,
) -> Result<Json<Vec<Pedido>>, Status> {
    let pedidos = preparacao_e_entrega_use_case.get_pedidos_novos().await?;
    Ok(Json(pedidos_novos))
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

#[openapi(tag = "Pedidos")]
#[get("/pedido/<id>", data="<pedido_input>")]
async fn get_pedido_by_id(
    pedido_e_pagamentos_use_case: &State<PedidosEPagamentosUseCase>,
    pedido_input: Json<CreatePedidoInput>,
    _logged_user_info: AuthenticatedUser,
) -> Result<Json<Pedido>, Status> {
    let pedido_atualizado = pedido_e_pagamentos_use_case.seleciona_pedido_por_id(pedido_input).await?;
    Ok(Json(pedido_atualizado))
}

#[openapi(tag = "Pedidos")]
#[get("/pedido_novos")]
async fn get_pedidos_novos(
    preparacao_e_entrega_use_case: &State<PreparacaoeEntregaUseCase>,
    _logged_user_info: AuthenticatedUser,
) -> Result<Json<Vec<Pedido>>, Status> {
    let pedidos_novos = preparacao_e_entrega_use_case.get_pedidos_novos().await?;
    Ok(Json(pedidos_novos))
}

#[openapi(tag = "Pedidos")]
#[put("/<id>/status/<status>")]
async fn update_status_pedido(
    preparacao_e_entrega_use_case: &State<PreparacaoeEntregaUseCase>,
    id: usize,
    status: String,
    _logged_user_info: AuthenticatedUser,
) -> Result<Json<Pedido>, Status> {
    let pedido = preparacao_e_entrega_use_case.atualizar_status_pedido(id, status).await?;
    Ok(Json(pedido))
}

pub fn routes() -> Vec<rocket::Route> {
    openapi_get_routes![post_novo_pedido, get_pedidos_novos, update_status_pedido]
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


// #[openapi(tag = "Pedidos")]
// #[get("/pedido/<id>")]
// async fn get_pedido(pedido_e_pagamentos_use_case: &State<PedidosEPagamentosUseCase>, id: usize, _logged_user_info: AuthenticatedUser) -> Result<Json<Pedido>, Status> {
//     let pedido = pedido_e_pagamentos_use_case.get_order_by_id(id).await?;
//     Ok(Json(pedido))
// }

// #[openapi(tag = "Pedidos")]
// #[get("/pedido/acompanhamento/")]
// async fn get_acompanhamentos(pedido_e_pagamentos_use_case: &State<PedidosEPagamentosUseCase>, id: usize, _logged_user_info: AuthenticatedUser) -> Result<Json<Vec<Produtos>>, Status> {
//     let acompanhamentos = pedido_e_pagamentos_use_case.lista_acompanhamentos().await?;
//     Ok(Json(acompanhamentos))
// }

// #[openapi(tag = "Pedidos")]
// #[get("/pedido/bebida/")]
// async fn get_bebidas(pedido_e_pagamentos_use_case: &State<PedidosEPagamentosUseCase>, id: usize, _logged_user_info: AuthenticatedUser) -> Result<Json<Vec<Produtos>>, Status> {
//     let bebidas = pedido_e_pagamentos_use_case.lista_bebidas().await?;
//     Ok(Json(bebidas))
// }


// #[openapi(tag = "Pedidos")]
// #[post("/pedido", data="<pedido_input>")]
// async fn post_novo_pedido(
//     pedido_e_pagamentos_use_case: &State<PedidosEPagamentosUseCase>,
//     pedido_input: Json<CreatePedidoInput>,
//     _logged_user_info: AuthenticatedUser,
// ) -> Result<Json<Pedido>, Status> {
//     let novo_pedido = pedido_e_pagamentos_use_case.novo_pedido(pedido_input).await?;
//     Ok(Json(novo_pedido))
// }

// #[openapi(tag = "Pedidos")]
// #[post("/pedido", data="<pedido_input>")]
// async fn post_novo_pedido(
//     pedido_e_pagamentos_use_case: &State<PedidosEPagamentosUseCase>,
//     pedido_input: Json<CreatePedidoInput>,
//     _logged_user_info: AuthenticatedUser,
// ) -> Result<Json<Pedido>, Status> {
//     let novo_pedido = pedido_e_pagamentos_use_case.novo_pedido(pedido_input).await?;
//     Ok(Json(novo_pedido))
// }

// #[openapi(tag = "Pedidos")]
// #[get("/pedido/<id>")]
// async fn get_pedido(pedido_e_pagamentos_use_case: &State<PedidosEPagamentosUseCase>, id: usize, _logged_user_info: AuthenticatedUser) -> Result<Json<Pedido>, Status> {
//     let pedido = pedido_e_pagamentos_use_case.seleciona_pedido_por_id(id).await?;
//     Ok(Json(pedido))
// }

// #[openapi(tag = "Pedidos")]
// #[post("/pedido/acompanhamento/<pedido_id>/<acompanhamento_id>")]
// async fn post_acompanhamentos(
//     pedido_e_pagamentos_use_case: &State<PedidosEPagamentosUseCase>, 
//     pedido_id: usize,
//     acompanhamento_id: usize,
//     _logged_user_info: AuthenticatedUser
// ) -> Result<Json<Pedido>, Status> {
//     let pedido = pedido_e_pagamentos_use_case.adicionar_acompanhamento(pedido_id, acompanhamento_id).await?;
//     Ok(Json(pedido))
// }


// #[openapi(tag = "Pedidos")]
// #[post("/pedido/lanche/<pedido_id>/<lanche_id>")]
// async fn post_lanches(
//     pedido_e_pagamentos_use_case: &State<PedidosEPagamentosUseCase>, 
//     pedido_id: usize,
//     lanche_id: usize,
//     _logged_user_info: AuthenticatedUser
// ) -> Result<Json<Pedido>, Status> {
//     let pedido = pedido_e_pagamentos_use_case.adicionar_lanche(pedido_id, lanche_id).await?;
//     Ok(Json(pedido))
// }


// #[openapi(tag = "Pedidos")]
// #[post("/pedido/bebida/<pedido_id>/<bebida_id>")]
// async fn post_bebidas(
//     pedido_e_pagamentos_use_case: &State<PedidosEPagamentosUseCase>, 
//     pedido_id: usize,
//     bebida_id: usize,
//     _logged_user_info: AuthenticatedUser
// ) -> Result<Json<Pedido>, Status> {
//     let pedido = pedido_e_pagamentos_use_case.adicionar_bebida(pedido_id, bebida_id).await?;
//     Ok(Json(pedido))
// }