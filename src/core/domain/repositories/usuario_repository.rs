use mockall::*;

use crate::core::domain::base::domain_error::DomainError;
use crate::core::domain::entities::usuario::Usuario;
use crate::core::domain::value_objects::cpf::Cpf;

#[automock]
#[async_trait]
pub trait UsuarioRepository {
    async fn get_usuarios(&self) -> Result<Vec<Usuario>, DomainError>;

    async fn get_usuario_by_id(&self, id: usize) -> Result<Usuario, DomainError>;

    async fn get_usuario_by_cpf(&self, cpf: Cpf) -> Result<Usuario, DomainError>;

    async fn create_usuario(&mut self, user: Usuario) -> Result<Usuario, DomainError>;

    async fn update_usuario(&mut self, dados_usuario_atualizado: Usuario) -> Result<Usuario, DomainError>;

    async fn delete_usuario(&mut self, cpf: Cpf) -> Result<(), DomainError>;
}
