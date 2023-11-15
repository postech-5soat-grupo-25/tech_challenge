use rocket::http::Status;
use schemars::JsonSchema;
use serde::Serialize;

use crate::core::domain::base::domain_error::DomainError;

impl From<DomainError> for Status  {
  fn from(error: DomainError) -> Self {
      match error {
          DomainError::AlreadyExists => Status::Conflict,
          DomainError::NotFound => Status::NotFound,
          _ => Status::InternalServerError
      }
  }
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct ErrorResponse {
  pub msg: String,
  pub status: usize,
}