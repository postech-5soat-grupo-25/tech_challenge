use crate::core::domain::base::domain_error::DomainError;
use crate::core::domain::entities::order::Order;


#[async_trait]
pub trait OrderRepository {
  async fn get_orders(&self) -> Result<Vec<Order>, DomainError>;

  async fn get_order_by_id(&self, id: usize) -> Result<Order, DomainError>;

  async fn create_order(&mut self, order: Order) -> Result<Order, DomainError>;

  async fn update_order(&mut self, new_order_data: Order) -> Result<Order, DomainError>;

  async fn delete_order(&mut self, id: usize) -> Result<(), DomainError>;
}