use chrono::Utc;
use rocket::futures::lock::Mutex;
use schemars::JsonSchema;
use serde::Deserialize;
use std::sync::Arc;

use crate::core::domain::base::domain_error::DomainError;
use crate::core::domain::entities::usuario::{Status, Tipo, Usuario};
use crate::core::domain::repositories::usuario_repository::UsuarioRepository;
use crate::core::domain::value_objects::cpf::Cpf;

#[derive(Clone, Debug, Deserialize, JsonSchema)]
pub struct CreateUsuarioInput {
    nome: String,
    email: String,
    senha: String,
    cpf: String,
    tipo: String,
    status: String,
}


#[derive(Clone)]
pub struct UsuarioUseCase {
    usuario_repository: Arc<Mutex<dyn UsuarioRepository + Sync + Send>>,
}

impl UsuarioUseCase {
    pub fn new(usuario_repository: Arc<Mutex<dyn UsuarioRepository + Sync + Send>>) -> Self {
        UsuarioUseCase { usuario_repository }
    }

    pub async fn get_usuarios(&self) -> Result<Vec<Usuario>, DomainError> {
        let usuario_repository = self.usuario_repository.lock().await;
        usuario_repository.get_usuarios().await
    }

    pub async fn get_usuario_by_id(&self, id: usize) -> Result<Usuario, DomainError> {
        let usuario_repository = self.usuario_repository.lock().await;
        usuario_repository.get_usuario_by_id(id).await
    }

    pub async fn get_usuario_by_cpf(&self, cpf: Cpf) -> Result<Usuario, DomainError> {
        let usuario_repository = self.usuario_repository.lock().await;
        usuario_repository.get_usuario_by_cpf(cpf).await
    }

    pub async fn create_usuario(
        &self,
        usuario: CreateUsuarioInput,
    ) -> Result<Usuario, DomainError> {
        let mut usuario_repository = self.usuario_repository.lock().await;
        let _id= 0;
        let valid_cpf = Cpf::new(usuario.cpf.clone())?;
        let valid_tipo: Tipo = usuario.tipo.parse().unwrap();
        let valid_status: Status = usuario.status.parse().unwrap();
        let _now = Utc::now().format("%Y-%m-%d %H:%M:%S%.3f%z").to_string();

        let usuario = usuario_repository
            .create_usuario(Usuario::new(
                _id,
                usuario.nome,
                usuario.email,
                valid_cpf,
                usuario.senha,
                valid_tipo,
                valid_status,
                _now.clone(),
                _now,
            ))
            .await?;

        Ok(usuario.clone())
    }

    pub async fn update_usuario(
        &self,
        id : usize,
        usuario: CreateUsuarioInput,
    ) -> Result<Usuario, DomainError> {
        let mut usuario_repository = self.usuario_repository.lock().await;

        let valid_cpf = Cpf::new(usuario.cpf.clone())?;
        let valid_tipo: Tipo = usuario.tipo.parse().unwrap();
        let valid_status: Status = usuario.status.parse().unwrap();
        let _now = Utc::now().format("%Y-%m-%d %H:%M:%S%.3f%z").to_string();

        let usuario = usuario_repository
            .update_usuario(Usuario::new(
                id,
                usuario.nome,
                usuario.email,
                valid_cpf,
                usuario.senha,
                valid_tipo,
                valid_status,
                _now.clone(),
                _now,
            ))
            .await?;

        Ok(usuario.clone())
    }

    pub async fn delete_usuario(&self, cpf: Cpf) -> Result<(), DomainError> {
        let mut usuario_repository = self.usuario_repository.lock().await;
        usuario_repository.delete_usuario(cpf).await?;
        Ok(())
    }
}

unsafe impl Send for UsuarioUseCase {}
unsafe impl Sync for UsuarioUseCase {}
