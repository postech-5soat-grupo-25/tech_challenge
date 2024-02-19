use std::sync::Arc;
use tokio::sync::Mutex;

use crate::base::domain_error::DomainError;
use crate::entities::usuario::Usuario;
use crate::entities::cpf::Cpf;
use crate::traits::usuario_gateway::UsuarioGateway;
use crate::use_cases::gerenciamento_de_usuarios_use_case::{CreateUsuarioInput, UsuarioUseCase};

pub struct UsuarioController {
    pub usuario_use_case: UsuarioUseCase,
}

impl UsuarioController {
    pub fn new(usuario_repository: Arc<Mutex<dyn UsuarioGateway + Sync + Send>>) -> UsuarioController {
        let usuario_use_case = UsuarioUseCase::new(usuario_repository);
        UsuarioController {
            usuario_use_case,
        }
    }

    pub async fn get_usuarios(
        &self,
    ) -> Result<Vec<Usuario>, DomainError> {
        self.usuario_use_case.get_usuarios().await
    }

    pub async fn get_usuario(
        &self,
        id: usize,
    ) -> Result<Usuario, DomainError> {
        self.usuario_use_case.get_usuario_by_id(id).await
    }

    pub async fn create_usuario(
        &self,
        usuario_input: CreateUsuarioInput,
    ) -> Result<Usuario, DomainError> {
        self.usuario_use_case.create_usuario(usuario_input).await
    }

    pub async fn update_usuario(
        &self,
        id: usize,
        usuario_input: CreateUsuarioInput,
    ) -> Result<Usuario, DomainError> {
        self.usuario_use_case.update_usuario(id, usuario_input).await
    }

    pub async fn delete_usuario(
        &self,
        cpf: Cpf,
    ) -> Result<(), DomainError> {
        self.usuario_use_case.delete_usuario(cpf).await
    }
}
