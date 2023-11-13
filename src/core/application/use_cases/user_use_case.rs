use crate::core::domain::repositories::user_repository::UserRepository;
use crate::core::domain::base::domain_error::DomainError;
use crate::core::domain::entities::usuario::Usuario;

pub struct UserUseCase {
  user_repository: Box<dyn UserRepository>
}

impl UserUseCase {
  pub fn new(user_repository: Box<dyn UserRepository>) -> Self {
    UserUseCase {
      user_repository
    }
  }

  pub fn get_users(&self) -> Result<Vec<Usuario>, DomainError> {
    self.user_repository.get_users()
  }

  pub fn get_user_by_id(&self, id: i32) -> Result<Usuario, DomainError> {
    self.user_repository.get_user_by_id(id)
  }
}

unsafe impl Send for UserUseCase {}
unsafe impl Sync for UserUseCase {}