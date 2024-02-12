use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use rocket_okapi::{openapi, openapi_get_routes};
use serde::de::IntoDeserializer;

use crate::adapter::api::error_handling::ErrorResponse;
use crate::adapter::api::request_guards::authentication_guard::AuthenticatedUser;
use crate::core::application::use_cases::pedidos_e_pagamentos_use_case::{
    CreatePedidoInput, PedidosEPagamentosUseCase,
};
use crate::core::application::use_cases::preparacao_e_entrega_use_case::PreparacaoeEntregaUseCase;
use crate::core::domain::entities::pedido::{self, Pedido};
use crate::core::domain::entities::produto::Categoria;

#[openapi(tag = "Pedidos")]
#[get("/")]
async fn get_pedidos(
    pedidos_e_pagamentos_use_case: &State<PedidosEPagamentosUseCase>,
    _logged_user_info: AuthenticatedUser,
) -> Result<Json<Vec<Pedido>>, Status> {
    let pedidos = pedidos_e_pagamentos_use_case.lista_pedidos().await?;
    Ok(Json(pedidos))
}

#[openapi(tag = "Pedidos")]
#[post("/", data = "<pedido_input>")]
async fn post_novo_pedido(
    pedido_e_pagamentos_use_case: &State<PedidosEPagamentosUseCase>,
    pedido_input: Json<CreatePedidoInput>,
) -> Result<Json<Pedido>, Status> {
    let pedido_input = pedido_input.into_inner();
    let novo_pedido = pedido_e_pagamentos_use_case
        .novo_pedido(pedido_input)
        .await?;
    Ok(Json(novo_pedido))
}

// #[openapi(tag = "Pedidos")]
// #[put("/pedido/<id>")]
// async fn get_pedido_by_id(
//     pedido_e_pagamentos_use_case: &State<PedidosEPagamentosUseCase>,
//     id: usize,
//     _logged_user_info: AuthenticatedUser,
// ) -> Result<Json<Pedido>, Status> {
//     let pedido_atualizado = pedido_e_pagamentos_use_case.;
//     Ok(Json(pedido_atualizado))
// }

#[openapi(tag = "Pedidos")]
#[get("/novos")]
async fn get_pedidos_novos(
    preparacao_e_entrega_use_case: &State<PreparacaoeEntregaUseCase>,
    __logged_user_info: AuthenticatedUser,
) -> Result<Json<Vec<Pedido>>, Status> {
    let pedidos_novos = preparacao_e_entrega_use_case.get_pedidos_novos().await?;
    Ok(Json(pedidos_novos))
}

#[openapi(tag = "Pedidos")]
#[put("/<id>/status/<status>")]
async fn update_status_pedido(
    preparacao_e_entrega_use_case: &State<PreparacaoeEntregaUseCase>,
    id: usize,
    status: &str,
    __logged_user_info: AuthenticatedUser,
) -> Result<Json<Pedido>, Status> {
    let status = match status {
        "cancelado" => pedido::Status::Cancelado,
        "emPreparacao" => pedido::Status::EmPreparacao,
        "finalizado" => pedido::Status::Finalizado,
        "invalido" => pedido::Status::Invalido,
        "pendente" => pedido::Status::Pendente,
        "pronto" => pedido::Status::Pronto,
        "recebido" => pedido::Status::Recebido,
        _ => return Err(Status::BadRequest),
    };
    let pedido = preparacao_e_entrega_use_case
        .atualiza_status(id, status)
        .await?;
    Ok(Json(pedido))
}

#[openapi(tag = "Pedidos")]
#[put("/<id>/produto/<categoria>/<produto_id>")]
async fn insert_produto(
    pedidos_e_pagamentos_use_case: &State<PedidosEPagamentosUseCase>,
    id: usize,
    categoria: &str,
    produto_id: usize,
) -> Result<Json<Pedido>, Status> {
    match categoria {
        "Lanche" => {
            let pedido = pedidos_e_pagamentos_use_case
                .adicionar_lanche_com_personalizacao(id, produto_id)
                .await?;
            Ok(Json(pedido))
        }
        "Acompanhamento" => {
            let pedido = pedidos_e_pagamentos_use_case
                .adicionar_acompanhamento(id, produto_id)
                .await?;
            Ok(Json(pedido))
        }
        "Bebida" => {
            let pedido = pedidos_e_pagamentos_use_case
                .adicionar_bebida(id, produto_id)
                .await?;
            Ok(Json(pedido))
        }
        _ => Err(Status::BadRequest),
    }
}

#[openapi(tag = "Pedidos")]
#[put("/<id>/pagamento")]
async fn pagar(
    pedidos_e_pagamentos_use_case: &State<PedidosEPagamentosUseCase>,
    id: usize,
) -> Result<Json<Pedido>, Status> {
    let pedido = pedidos_e_pagamentos_use_case
        .realizar_pagamento_do_pedido(id)
        .await?;
    Ok(Json(pedido))
}

pub fn routes() -> Vec<rocket::Route> {
    openapi_get_routes![
        get_pedidos,
        post_novo_pedido,
        get_pedidos_novos,
        update_status_pedido,
        insert_produto,
        pagar,
    ]
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
