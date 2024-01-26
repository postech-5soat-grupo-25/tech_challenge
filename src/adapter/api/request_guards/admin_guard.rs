use rocket::{
  request::{self, Request, FromRequest, Outcome},
  http::Status
};
use rocket_okapi::{
  request::{OpenApiFromRequest, RequestHeaderInput},
  gen::OpenApiGenerator,
  OpenApiError,
  okapi::openapi3::{SecurityScheme, SecuritySchemeData, Object, SecurityRequirement}
};

use crate::{adapter::api::helpers::auth_helper::{validate_token, AuthError}, core::domain::entities::usuario::Tipo};

pub struct AdminUser {
  user_id: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AdminUser {
  type Error = AuthError;

  async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
      match req.headers().get_one("Authorization") {
          Some(token) => {
              let token = token.replace("Bearer ", "");
              match validate_token(token.to_string(), Some(Tipo::Admin)) {
                  Ok(user_id) => Outcome::Success(AdminUser {
                      user_id,
                  }),
                  Err(_) => return Outcome::Failure((Status::Unauthorized, AuthError::InvalidToken))
              }
          }
          None => Outcome::Failure((Status::BadRequest, AuthError::MissingToken)),
      }
  }
}

impl<'a> OpenApiFromRequest<'a> for AdminUser {
  fn from_request_input(
      _gen: &mut OpenApiGenerator,
      _name: String,
      _required: bool,
  ) -> Result<RequestHeaderInput, OpenApiError> {
      let security_scheme = SecurityScheme {
          description: Some(
              "Authorization: 'Bearer: `token`'".to_owned(),
          ),

          data: SecuritySchemeData::Http {
              scheme: "bearer".to_owned(),
              bearer_format: Some("bearer".to_owned()),
          },
          extensions: Object::default(),
      };

      let mut security_req = SecurityRequirement::new();
      security_req.insert("HttpAuth".to_owned(), Vec::new());
      Ok(RequestHeaderInput::Security(
          "HttpAuth".to_owned(),
          security_scheme,
          security_req,
      ))
  }
}
