use crate::core::domain::base::domain_error::DomainError;
use crate::core::domain::entities::pedido::Pedido;

#[async_trait]
pub trait PedidoRepository {
  
  async fn get_pedidos_novos(&self) -> Result<Vec<Pedido>, DomainError>;

  async fn set_pedido_em_preparacao(&mut self, id: usize) -> Result<Pedido, DomainError>;
  
  async fn set_pedido_pronto(&mut self,  id: usize) -> Result<Pedido, DomainError>;
  
  async fn set_pedido_finalizado(&mut self,  id: usize) -> Result<Pedido, DomainError>;
  
  async fn set_pedido_cancelado(&mut self, id: usize) -> Result<Pedido, DomainError>;
}