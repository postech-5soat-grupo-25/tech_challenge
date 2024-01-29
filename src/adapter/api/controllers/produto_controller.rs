use rocket::http::Status;
use rocket::response::status::NotFound;
use rocket::request::FromParam;
use rocket::serde::json::Json;
use rocket::State;
use rocket_okapi::{openapi, openapi_get_routes};

use crate::adapter::api::error_handling::ErrorResponse;
use crate::adapter::api::request_guards::authentication_guard::AuthenticatedUser;
use crate::core::application::use_cases::produto_use_case::{ProdutoUseCase, CreateProdutoInput, UpdateProdutoInput};
use crate::core::domain::entities::produto::Produto;

#[openapi(tag = "Produto")]
#[get("/produto")]
async fn get_produto(
    produto_use_case: &State<ProdutoUseCase>,
    _logged_user_info: AuthenticatedUser,
) -> Result<Json<Vec<Produto>>, Status> {
    let produtos = produto_use_case.get_produtos().await?;
    Ok(Json(produtos))
}

#[openapi(tag = "Produto")]
#[put("/<id>")]
async fn get_produto_by_id(
    produto_use_case: &State<ProdutoUseCase>,
    id: usize,
    _logged_user_info: AuthenticatedUser,
) -> Result<Json<Produto>, Status> {
    let produto = produto_use_case.get_products_by_id(id).await?;
    Ok(Json(produto))
}

#[openapi(tag = "Produto")]
#[post("/", data = "<produto_input>")]
async fn create_produto(
    produto_use_case: &State<ProdutoUseCase>,
    produto_input: Json<CreateProdutoInput>,
    _logged_user_info: AuthenticatedUser,
) -> Result<Json<Produto>, Status> {
    let produto_input = produto_input.into_inner();
    let produto = produto_use_case.create_produto(produto_input).await?;
    Ok(Json(produto))
}

#[openapi(tag = "Produto")]
#[post("/<id>", data = "<produto_input>")]
async fn update_produto(
    produto_use_case: &State<ProdutoUseCase>,
    produto_input: Json<UpdateProdutoInput>,
    id: usize,
    _logged_user_info: AuthenticatedUser,
) -> Result<Json<Produto>, Status> {
    let produto_input = produto_input.into_inner();
    let produto = produto_use_case.update_produto(id, produto_input).await?;
    Ok(Json(produto))
}

pub fn routes() -> Vec<rocket::Route> {
    openapi_get_routes![get_produto, get_produto_by_id, create_produto, update_produto]
}

#[catch(404)]
fn produto_not_found() -> Json<ErrorResponse> {
    let error = ErrorResponse {
        msg: "Produto não encontrado!".to_string(),
        status: 404,
    };
    Json(error)
}

pub fn catchers() -> Vec<rocket::Catcher> {
    catchers![produto_not_found]
}