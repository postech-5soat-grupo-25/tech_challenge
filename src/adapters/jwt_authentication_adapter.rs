use crate::{
    base::domain_error::DomainError,
    entities::usuario::{Tipo, Usuario},
    traits::authentication_adapter::AuthenticationAdapter,
};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    role: Tipo,
    company: String,
    exp: usize,
}

#[derive(Clone)]
pub struct JWTAuthenticationAdapter {
    secret: String,
}

impl JWTAuthenticationAdapter {
    pub fn new(secret: String) -> Self {
        JWTAuthenticationAdapter { secret: secret }
    }
}

#[async_trait]
impl AuthenticationAdapter for JWTAuthenticationAdapter {
    async fn get_token(&self, user: Usuario) -> Result<String, DomainError> {
        let my_claims = Claims {
            sub: user.id().to_string(),
            role: user.tipo().clone(),
            company: "wdrops".to_string(),
            exp: 10000000000,
        };

        let header = Header::new(Algorithm::HS512);
        let token = encode(
            &header,
            &my_claims,
            &EncodingKey::from_secret(self.secret.as_ref()),
        );
        match token {
            Ok(t) => Ok(t),
            Err(_) => Err(DomainError::Invalid("Erro ao gerar token".to_string())),
        }
    }

    async fn validate_token(&self, token: String, role: Option<Tipo>) -> Result<String, DomainError> {
        let token_data = decode::<Claims>(
            &token,
            &DecodingKey::from_secret(self.secret.as_ref()),
            &Validation::new(Algorithm::HS512),
        );
        match token_data {
            Ok(t) => match role {
                Some(r) => {
                    if t.claims.role != r {
                        return Err(DomainError::Unauthorized);
                    }
                    Ok(t.claims.sub)
                }
                None => Ok(t.claims.sub),
            },
            Err(err) => {
                eprintln!("Invalid Token: {}", err);
                Err(DomainError::Unauthorized)
            }
        }
    }
}

unsafe impl Sync for JWTAuthenticationAdapter {}
unsafe impl Send for JWTAuthenticationAdapter {}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use crate::adapters::jwt_authentication_adapter;
    use crate::entities::cpf::Cpf;
    use crate::entities::usuario::{Status, Tipo};
    use tokio;

    #[tokio::test]
    async fn should_generate_token() {
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
        let jwt_authentication_adapter = jwt_authentication_adapter::JWTAuthenticationAdapter::new("secret".to_string());
        let token = jwt_authentication_adapter.get_token(user).await;
        assert!(token.is_ok());
    }

    #[tokio::test]
    async fn should_validate_token_for_any_user() {
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
        let jwt_authentication_adapter = jwt_authentication_adapter::JWTAuthenticationAdapter::new("secret".to_string());
        let token = jwt_authentication_adapter.get_token(user.clone()).await;
        assert!(token.is_ok());
        let token = token.unwrap();
        let user_id = jwt_authentication_adapter.validate_token(token, None).await;
        assert!(user_id.is_ok());
        assert_eq!(user_id.unwrap(), user.id().to_string());
    }

    #[tokio::test]
    async fn should_validate_token_for_admin_user() {
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
        let jwt_authentication_adapter = jwt_authentication_adapter::JWTAuthenticationAdapter::new("secret".to_string());
        let token = jwt_authentication_adapter.get_token(user.clone()).await;
        assert!(token.is_ok());
        let token = token.unwrap();
        let user_id = jwt_authentication_adapter.validate_token(token, Some(Tipo::Admin)).await;
        assert!(user_id.is_ok());
        assert_eq!(user_id.unwrap(), user.id().to_string());
    }

    #[tokio::test]
    async fn should_block_token_for_non_admin_user() {
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
        let jwt_authentication_adapter = jwt_authentication_adapter::JWTAuthenticationAdapter::new("secret".to_string());
        let token = jwt_authentication_adapter.get_token(user.clone()).await;
        assert!(token.is_ok());
        let token = token.unwrap();
        let user_id = jwt_authentication_adapter.validate_token(token, Some(Tipo::Admin)).await;
        assert!(user_id.is_err());
    }
}