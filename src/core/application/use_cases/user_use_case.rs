use std::sync::Arc;

use rocket::futures::lock::Mutex;
use schemars::JsonSchema;
use serde::Deserialize;

use crate::core::domain::base::domain_error::DomainError;
use crate::core::domain::entities::usuario::Usuario;
use crate::core::domain::repositories::usuario_repository::UsuarioRepository;
use crate::core::domain::value_objects::cpf::Cpf;
use crate::core::domain::value_objects::endereco::Endereco;

#[derive(Clone, Debug, Deserialize, JsonSchema)]
pub struct CreateUserInput {
    nome: String,
    email: String,
    senha: String,
    endereco: String,
    cpf: String,
}

#[derive(Clone, Debug, Deserialize, JsonSchema)]
pub struct UpdateUserInput {
    nome: Option<String>,
    email: Option<String>,
    endereco: Option<String>,
}

#[derive(Clone)]
pub struct UserUseCase {
    usuario_repository: Arc<Mutex<dyn UsuarioRepository + Sync + Send>>,
}

impl UserUseCase {
    pub fn new(usuario_repository: Arc<Mutex<dyn UsuarioRepository + Sync + Send>>) -> Self {
        UserUseCase { usuario_repository }
    }

    pub async fn get_users(&self) -> Result<Vec<Usuario>, DomainError> {
        let usuario_repository = self.usuario_repository.lock().await;
        usuario_repository.get_usuarios().await
    }

    pub async fn get_user_by_id(&self, id: usize) -> Result<Usuario, DomainError> {
        let usuario_repository = self.usuario_repository.lock().await;
        usuario_repository.get_usuario_by_id(id).await
    }

    pub async fn get_user_by_cpf(&self, cpf: Cpf) -> Result<Usuario, DomainError> {
        let usuario_repository = self.usuario_repository.lock().await;
        usuario_repository.get_usuario_by_cpf(cpf).await
    }

    pub async fn create_user(&self, user: CreateUserInput) -> Result<Usuario, DomainError> {
        let mut usuario_repository = self.usuario_repository.lock().await;
        let new_id = 0;
        let valid_cpf = Cpf::new(user.cpf.clone())?;
        let valid_endereco = Endereco::new(user.endereco.clone());

        let user = usuario_repository
            .create_usuario(Usuario::new(new_id, user.nome, user.email, user.senha, valid_cpf, valid_endereco))
            .await?;

        Ok(user.clone())
    }

    pub async fn update_user_info(&self, id: usize, fields_to_update: UpdateUserInput) -> Result<Usuario, DomainError> {
        let mut usuario_repository = self.usuario_repository.lock().await;
        let mut user = usuario_repository.get_usuario_by_id(id).await?;
        if let Some(nome) = fields_to_update.nome {
            user.set_nome(nome);
        }
        if let Some(email) = fields_to_update.email {
            user.set_email(email);
        }
        if let Some(endereco) = fields_to_update.endereco {
            let valid_endereco = Endereco::new(endereco);
            user.set_endereco(valid_endereco);
        }
        usuario_repository.update_usuario(user).await
    }

    pub async fn delete_user(&self, id: usize) -> Result<(), DomainError> {
        let mut usuario_repository = self.usuario_repository.lock().await;
        usuario_repository.delete_usuario(id).await?;
        Ok(())
    }
}

unsafe impl Send for UserUseCase {}
unsafe impl Sync for UserUseCase {}
