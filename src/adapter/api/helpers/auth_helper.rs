use rocket::http::Status;
use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use crate::core::domain::entities::usuario;
use usuario::Usuario;
use crate::adapter::api::config::Config;

#[derive(Debug)]
pub enum AuthError {
  InvalidToken,
  MissingToken,
}
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    role: usuario::Tipo,
    company: String,
    exp: usize,
}

pub fn get_token(user: Usuario) -> Result<String, Status> {
    let my_claims = Claims {
        sub: user.id().to_string(),
        role: user.tipo().clone(),
        company: "wdrops".to_string(),
        exp: 10000000000,
    };

    let header = Header::new(Algorithm::HS512);
    let secret = Config::build().secret;
    let token = encode(&header, &my_claims, &EncodingKey::from_secret(secret.as_ref()));
    match token {
        Ok(t) => Ok(t),
        Err(_) => Err(Status::InternalServerError)
    }
}

pub fn validate_token(token: String, role: Option<usuario::Tipo>) -> Result<String, Status> {
    let secret = Config::build().secret;
    let token_data = decode::<Claims>(&token, &DecodingKey::from_secret(secret.as_ref()), &Validation::new(Algorithm::HS512));
    match token_data {
        Ok(t) => {
            match role {
                Some(r) => {
                    if t.claims.role != r {
                        return Err(Status::Unauthorized);
                    }
                    Ok(t.claims.sub)
                }
                None => Ok(t.claims.sub)
            }
        }
        Err(err) => {
            eprintln!("Invalid Token: {}", err);
            Err(Status::Unauthorized)
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;

    use crate::core::domain::entities::usuario::{Usuario, Tipo, Status};
    use crate::adapter::api::helpers::auth_helper::{get_token, validate_token};
    use crate::core::domain::value_objects::cpf::Cpf;

    
    #[test]
    fn should_generate_token() {
        let cpf = Cpf::new("123.456.789-09".to_string()).unwrap();
        let now = Utc::now().format("%Y-%m-%d %H:%M:%S%.3f%z").to_string();
        let user = Usuario::new(
            1,
            "Teste".to_string(),
            "teste@email.com".to_string(),
            cpf.clone(),
            "senha_segura".to_string(),
            Tipo::Admin,
            Status::Ativo,
            now.clone(),
            now,
        );
        let token = get_token(user);
        assert!(token.is_ok());
    }

    #[test]
    fn should_validate_token_for_any_user() {
        let cpf = Cpf::new("123.456.789-09".to_string()).unwrap();
        let now = Utc::now().format("%Y-%m-%d %H:%M:%S%.3f%z").to_string();
        let user = Usuario::new(
            1,
            "Teste".to_string(),
            "teste@email.com".to_string(),
            cpf.clone(),
            "senha_segura".to_string(),
            Tipo::Cozinha,
            Status::Ativo,
            now.clone(),
            now,
        );
        let token = get_token(user.clone());
        assert!(token.is_ok());
        let token = token.unwrap();
        let user_id = validate_token(token, None);
        assert!(user_id.is_ok());
        assert_eq!(user_id.unwrap(), user.id().to_string());
    }

    #[test]
    fn should_validate_token_for_admin_user() {
        let cpf = Cpf::new("123.456.789-09".to_string()).unwrap();
        let now = Utc::now().format("%Y-%m-%d %H:%M:%S%.3f%z").to_string();
        let user = Usuario::new(
            1,
            "Teste".to_string(),
            "teste@email.com".to_string(),
            cpf.clone(),
            "senha_segura".to_string(),
            Tipo::Admin,
            Status::Ativo,
            now.clone(),
            now,
        );
        let token = get_token(user.clone());
        assert!(token.is_ok());
        let token = token.unwrap();
        let user_id = validate_token(token, Some(Tipo::Admin));
        assert!(user_id.is_ok());
        assert_eq!(user_id.unwrap(), user.id().to_string());
    }

    #[test]
    fn should_block_token_for_non_admin_user() {
        let cpf = Cpf::new("123.456.789-09".to_string()).unwrap();
        let now = Utc::now().format("%Y-%m-%d %H:%M:%S%.3f%z").to_string();
        let user = Usuario::new(
            1,
            "Teste".to_string(),
            "teste@email.com".to_string(),
            cpf.clone(),
            "senha_segura".to_string(),
            Tipo::Cozinha,
            Status::Ativo,
            now.clone(),
            now,
        );
        let token = get_token(user.clone());
        assert!(token.is_ok());
        let token = token.unwrap();
        let user_id = validate_token(token, Some(Tipo::Admin));
        assert!(user_id.is_err());
    }
}


