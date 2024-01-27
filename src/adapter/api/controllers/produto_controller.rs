use rocket::http::Status;
use rocket::response::status::NotFound;
use rocket::request::FromParam;
use rocket::serde::json::Json;
use rocket::State;
use rocket_okapi::{openapi, openapi_get_routes};

use crate::adapter::api::error_handling::ErrorResponse;
use crate::adapter::api::request_guards::authentication_guard::AuthenticatedUser;
use crate::core::application::use_cases::product_use_case::ProductUseCase;
use crate::core::domain::entities::produto::Produto;

#[openapi(tag = "Produto")]
#[get("/produto")]
async fn get_produto(
    produto_use_case: &State<ProductUseCaseUseCase>,
    _logged_user_info: AuthenticatedUser,
) -> Result<Json<Vec<Pedido>>, Status> {
    let produtos = produto_use_case.get_products().await?;
    Ok(Json(produtos))
}

#[openapi(tag = "Produto")]
#[put("/<id>")]
async fn update_status_pedido(
    produto_use_case: &State<ProductUseCaseUseCase>,
    id: usize,
    _logged_user_info: AuthenticatedUser,
) -> Result<Json<Pedido>, Status> {
    let produto = produto_use_case.get_products_by_id(id).await?;
    Ok(Json(produto))
}

#[openapi(tag = "Produto")]
#[post("/", data = "<produto_input>")]
async fn create_produto(
    produto_use_case: &State<ProductUseCaseUseCase>,
    produto_input: Json<CreateProductInput>,
    _logged_user_info: AuthenticatedUser,
) -> Result<Json<Cliente>, Status> {
    let produto_input = produto_input.into_inner();
    let produto = produto_use_case.create_produto(produto_input).await?;
    Ok(Json(produto))
}

#[openapi(tag = "Produto")]
#[post("/<id>", data = "<produto_input>")]
async fn update_produto(
    produto_use_case: &State<ProductUseCaseUseCase>,
    produto_input: Json<UpdateProductInput>,
    id: usize,
    _logged_user_info: AuthenticatedUser,
) -> Result<Json<Cliente>, Status> {
    let produto_input = produto_input.into_inner();
    let produto = produto_use_case.update_produto(id, produto_input).await?;
    Ok(Json(produto))
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
