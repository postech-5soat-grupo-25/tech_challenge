use crate::core::domain::base::domain_error::DomainError;
use crate::core::domain::entities::usuario::Usuario;

pub trait UserRepositoryInterface {
  fn get_users(&self) -> Result<Vec<Usuario>, DomainError>;

  fn get_user_by_id(&self, id: i32) -> Result<Usuario, DomainError>;
}