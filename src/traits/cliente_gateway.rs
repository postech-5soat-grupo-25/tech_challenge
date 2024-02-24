use mockall::*;

use crate::base::domain_error::DomainError;
use crate::entities::{
    cliente::Cliente,
    cpf::Cpf
};

#[automock]
#[async_trait]
pub trait ClienteGateway {
    async fn get_clientes(&self) -> Result<Vec<Cliente>, DomainError>;

    async fn get_cliente_by_cpf(&self, cpf: Cpf) -> Result<Cliente, DomainError>;

    async fn get_cliente_by_id(&self, id: usize) -> Result<Cliente, DomainError>;

    async fn create_cliente(&mut self, cliente: Cliente) -> Result<Cliente, DomainError>;

    async fn delete_cliente(&mut self, cpf: Cpf) -> Result<(), DomainError>;
}
