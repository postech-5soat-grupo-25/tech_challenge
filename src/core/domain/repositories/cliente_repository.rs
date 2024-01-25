use crate::core::domain::{
    base::domain_error::DomainError,
    entities::cliente::Cliente,
};

#[async_trait]
pub trait ClienteRepository {
    async fn get_cliente_by_id(&mut self, cliente_id: usize) -> Result<Cliente, DomainError>;

}