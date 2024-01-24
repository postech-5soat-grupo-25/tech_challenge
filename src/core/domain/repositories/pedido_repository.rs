use crate::core::domain::{base::domain_error::DomainError, entities::pedido::Pedido};

#[async_trait]
pub trait PedidoRepository {
  async fn create_pedido(&mut self, pedido: Pedido) -> Result<Pedido, DomainError>;
}