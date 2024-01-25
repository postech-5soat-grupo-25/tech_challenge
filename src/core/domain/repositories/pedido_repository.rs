use crate::core::domain::base::domain_error::DomainError;
use crate::core::domain::entities::pedido::Pedido;
use crate::core::domain::entities::pedido::Status;

#[async_trait]
pub trait PedidoRepository {
  
  async fn get_pedidos_novos(&self) -> Result<Vec<Pedido>, DomainError>;

  async fn set_pedido_status(&mut self, id: usize, status :String) -> Result<Pedido, DomainError>;

}