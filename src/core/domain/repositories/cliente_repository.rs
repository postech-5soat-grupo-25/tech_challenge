use crate::core::domain::base::domain_error::DomainError;
use crate::core::domain::entities::cliente::Cliente;
use crate::core::domain::value_objects::cpf::Cpf;

#[async_trait]
pub trait ClienteRepository: Send + Sync{
    async fn get_clientes(&self) -> Result<Vec<Cliente>, DomainError>;

    async fn get_cliente_by_cpf(&self, cpf: Cpf) -> Result<Cliente, DomainError>;

    async fn get_cliente_by_id(&self, id: usize) -> Result<Cliente, DomainError>;

    async fn create_cliente(&mut self, cliente: Cliente) -> Result<Cliente, DomainError>;

    async fn delete_cliente(&mut self, cpf: Cpf) -> Result<(), DomainError>;
}
