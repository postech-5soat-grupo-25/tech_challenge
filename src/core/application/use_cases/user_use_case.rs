use std::sync::Arc;

use rocket::futures::lock::Mutex;
use schemars::JsonSchema;
use serde::Deserialize;

use crate::core::domain::base::domain_error::DomainError;
use crate::core::domain::entities::usuario::Usuario;
use crate::core::domain::repositories::user_repository::UserRepository;
use crate::core::domain::value_objects::cpf::Cpf;
use crate::core::domain::value_objects::endereco::Endereco;

// TODO: onde faço a validação dos dados? [ ] UseCase [ ] Repository [ ] Entity
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
    user_repository: Arc<Mutex<dyn UserRepository + Sync + Send>>,
}

impl UserUseCase {
    pub fn new(user_repository: Arc<Mutex<dyn UserRepository + Sync + Send>>) -> Self {
        UserUseCase { user_repository }
    }

    pub async fn get_users(&self) -> Result<Vec<Usuario>, DomainError> {
        let user_repository = self.user_repository.lock().await;
        user_repository.get_users().await
    }

    pub async fn get_user_by_id(&self, id: usize) -> Result<Usuario, DomainError> {
        let user_repository = self.user_repository.lock().await;
        user_repository.get_user_by_id(id).await
    }

    pub async fn get_user_by_cpf(&self, cpf: Cpf) -> Result<Usuario, DomainError> {
        let user_repository = self.user_repository.lock().await;
        user_repository.get_user_by_cpf(cpf).await
    }

    pub async fn create_user(&self, user: CreateUserInput) -> Result<Usuario, DomainError> {
        let mut user_repository = self.user_repository.lock().await;
        // TODO: mover geracao de id pro repositorio (?)
        let new_id = user_repository.get_users().await?.len() + 1;
        // TODO: devo validar aqui ou na entidade?
        let valid_cpf = Cpf::new(user.cpf.clone());
        let valid_endereco = Endereco::new(user.endereco.clone());

        let user = user_repository
            .create_user(Usuario::new(new_id, user.nome, user.email, user.senha, valid_cpf, valid_endereco))
            .await?;

        Ok(user.clone())
    }

    pub async fn update_user_info(&self, id: usize, fields_to_update: UpdateUserInput) -> Result<Usuario, DomainError> {
        let mut user_repository = self.user_repository.lock().await;
        let mut user = user_repository.get_user_by_id(id).await?;
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
        user_repository.update_user(user).await
    }

    pub async fn delete_user(&self, id: usize) -> Result<(), DomainError> {
        let mut user_repository = self.user_repository.lock().await;
        user_repository.delete_user(id).await?;
        Ok(())
    }
}

unsafe impl Send for UserUseCase {}
unsafe impl Sync for UserUseCase {}
