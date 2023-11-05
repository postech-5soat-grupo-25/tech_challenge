use crate::core::domain::base::domain_exception::Result;
use crate::core::domain::entities::usuario::Usuario;

pub trait UserRepositoryInterface {
  fn get_users(&self) -> Result<Vec<Usuario>>;
}