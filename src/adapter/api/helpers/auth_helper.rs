use rocket::{Error, http::Status};
use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};

use crate::core::domain::entities::usuario::Usuario;

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: usize,
}

pub fn get_token(user: Usuario) -> Result<String, Status> {
    let my_claims = Claims {
        sub: "".to_string(),
        company: "".to_string(),
        exp: 10000000000,
    };

    let mut header = Header::new(Algorithm::HS512);
    header.kid = Some("blabla".to_owned());
    let token = encode(&header, &my_claims, &EncodingKey::from_secret("secret".as_ref()));
    match token {
        Ok(t) => Ok(t),
        Err(_) => Err(Status::InternalServerError)
    }
}


