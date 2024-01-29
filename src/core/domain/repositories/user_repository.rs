use crate::core::domain::base::domain_error::DomainError;
use crate::core::domain::entities::usuario::Usuario;
use crate::core::domain::value_objects::cpf::Cpf;

#[async_trait]
pub trait UserRepository {
  async fn get_users(&self) -> Result<Vec<Usuario>, DomainError>;

  async fn get_user_by_id(&self, id: usize) -> Result<Usuario, DomainError>;

  async fn get_user_by_cpf(&self, cpf: Cpf) -> Result<Usuario, DomainError>;

  async fn create_user(&mut self, user: Usuario) -> Result<Usuario, DomainError>;

  async fn update_user(&mut self, new_user_data: Usuario) -> Result<Usuario, DomainError>;

  async fn delete_user(&mut self, id: usize) -> Result<(), DomainError>;
}