use rocket::http::Status;
use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};

use crate::core::domain::entities::usuario::Usuario;

// generate save jwt key
static JWT_KEY: &str  = "secret";

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: usize,
}

pub fn get_token(user: Usuario) -> Result<String, Status> {
    let my_claims = Claims {
        sub: user.id().to_string(),
        company: "wdrops".to_string(),
        exp: 10000000000,
    };

    let header = Header::new(Algorithm::HS512);
    let token = encode(&header, &my_claims, &EncodingKey::from_secret(JWT_KEY.as_ref()));
    match token {
        Ok(t) => Ok(t),
        Err(_) => Err(Status::InternalServerError)
    }
}

pub fn validate_token(token: String) -> Result<String, Status> {
    let token_data = decode::<Claims>(&token, &DecodingKey::from_secret(JWT_KEY.as_ref()), &Validation::new(Algorithm::HS512));
    match token_data {
        Ok(t) => Ok(t.claims.sub),
        Err(err) => {
            eprintln!("Invalid Token: {}", err);
            Err(Status::Unauthorized)
        }
    }
}


