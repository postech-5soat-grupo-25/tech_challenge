use crate::core::domain::base::domain_error::DomainError;
use crate::core::domain::entities::pedido::Pedido;


#[async_trait]
pub trait OrderRepository {
  async fn get_orders(&self) -> Result<Vec<Pedido>, DomainError>;

  async fn get_order_by_id(&self, id: usize) -> Result<Pedido, DomainError>;

  async fn get_order_by_user(&self, user_id: usize) -> Result<Pedido, DomainError>; // TODO evaluate if needed

  async fn create_order(&mut self, order: Pedido) -> Result<Pedido, DomainError>;

  async fn update_order(&mut self, new_order_data: Pedido) -> Result<Pedido, DomainError>;

  async fn delete_order(&mut self, id: usize) -> Result<(), DomainError>;
}