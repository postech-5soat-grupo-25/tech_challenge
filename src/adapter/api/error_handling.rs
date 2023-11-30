use crate::core::domain::base::domain_error::DomainError;
use rocket::http::Status;
use rocket::serde::json::Json;
use schemars::JsonSchema;
use serde::Serialize;

impl From<DomainError> for Status {
    fn from(error: DomainError) -> Self {
        match error {
            DomainError::AlreadyExists => Status::Conflict,
            DomainError::NotFound => Status::NotFound,
            _ => Status::InternalServerError,
        }
    }
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct ErrorResponse {
    pub msg: String,
    pub status: usize,
}

#[catch(401)]
fn unauthorized() -> Json<ErrorResponse> {
    let error = ErrorResponse {
        msg: "Credenciais invalidas".to_string(),
        status: 401,
    };
    Json(error)
}

#[catch(500)]
fn internal() -> Json<ErrorResponse> {
    let error = ErrorResponse {
        msg: "Erro inesperado. Tente novamente mais tarde".to_string(),
        status: 500,
    };
    Json(error)
}

pub fn generic_catchers() -> Vec<rocket::Catcher> {
    catchers![unauthorized, internal]
}
