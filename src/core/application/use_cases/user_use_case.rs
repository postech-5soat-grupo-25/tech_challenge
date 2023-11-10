use crate::core::domain::repositories::user_repository::UserRepositoryInterface;
use crate::core::domain::base::domain_exception::Result;
use crate::core::domain::entities::usuario::Usuario;

pub struct UserUseCase {
  user_repository: Box<dyn UserRepositoryInterface>
}

impl UserUseCase {
  pub fn new(user_repository: Box<dyn UserRepositoryInterface>) -> Self {
    UserUseCase {
      user_repository
    }
  }

  pub fn get_users(&self) -> Result<Vec<Usuario>> {
    self.user_repository.get_users()
  }
}

unsafe impl Send for UserUseCase {}
unsafe impl Sync for UserUseCase {}