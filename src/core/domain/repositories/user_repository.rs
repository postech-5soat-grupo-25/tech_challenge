use crate::core::domain::base::domain_error::DomainError;
use crate::core::domain::entities::usuario::Usuario;

#[async_trait]
pub trait UserRepository {
  async fn get_users(&self) -> Result<Vec<Usuario>, DomainError>;

  async fn get_user_by_id(&self, id: i32) -> Result<Usuario, DomainError>;
}