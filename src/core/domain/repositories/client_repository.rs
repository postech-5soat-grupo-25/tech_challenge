use crate::core::domain::base::domain_error::DomainError;
use crate::core::domain::entities::client::Client;


#[async_trait]
pub trait ClientRepository {
  async fn get_clients(&self) -> Result<Vec<Client>, DomainError>;

  async fn get_client_by_id(&self, id: usize) -> Result<Client, DomainError>;

  async fn create_client(&mut self, client: Client) -> Result<Client, DomainError>;

  async fn update_client(&mut self, new_client_data: Client) -> Result<Client, DomainError>;

  async fn delete_client(&mut self, id: usize) -> Result<(), DomainError>;
}