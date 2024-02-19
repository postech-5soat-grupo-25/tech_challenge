use mockall::*;

use crate::{
    base::domain_error::DomainError,
    entities::usuario::{Tipo, Usuario},
};

#[automock]
#[async_trait]
pub trait AuthenticationAdapter{
    async fn get_token(&self, user: Usuario) -> Result<String, DomainError>;
    async fn validate_token(&self, token: String, role: Option<Tipo>) -> Result<String, DomainError>;
}
