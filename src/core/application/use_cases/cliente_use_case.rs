use chrono::Utc;
use rocket::futures::lock::Mutex;
use schemars::JsonSchema;
use serde::Deserialize;
use std::sync::Arc;

use crate::core::domain::base::domain_error::DomainError;
use crate::core::domain::entities::cliente::Cliente;
use crate::core::domain::repositories::cliente_repository::ClienteRepository;
use crate::core::domain::value_objects::cpf::Cpf;

#[derive(Clone, Debug, Deserialize, JsonSchema)]
pub struct CreateClienteInput {
    nome: String,
    email: String,
    cpf: String,
}

#[derive(Clone)]
pub struct ClienteUseCase {
    cliente_repository: Arc<Mutex<dyn ClienteRepository + Sync + Send>>,
}

impl ClienteUseCase {
    pub fn new(cliente_repository: Arc<Mutex<dyn ClienteRepository + Sync + Send>>) -> Self {
        ClienteUseCase { cliente_repository }
    }

    pub async fn get_clientes(&self) -> Result<Vec<Cliente>, DomainError> {
        let cliente_repository = self.cliente_repository.lock().await;
        cliente_repository.get_clientes().await
    }

    pub async fn get_cliente_by_cpf(&self, cpf: Cpf) -> Result<Cliente, DomainError> {
        let cliente_repository = self.cliente_repository.lock().await;
        cliente_repository.get_cliente_by_cpf(cpf).await
    }

    pub async fn create_cliente(
        &self,
        cliente: CreateClienteInput,
    ) -> Result<Cliente, DomainError> {
        let mut cliente_repository = self.cliente_repository.lock().await;
        let _id = 0;
        let cpf = Cpf::new(cliente.cpf.clone())?;
        let _now = Utc::now().format("%Y-%m-%d %H:%M:%S%.3f%z").to_string();
        let cliente = cliente_repository
            .create_cliente(Cliente::new(
                _id,
                cliente.nome,
                cliente.email,
                cpf,
                _now.clone(),
                _now,
            ))
            .await?;

        Ok(cliente.clone())
    }

    pub async fn delete_cliente(&self, cpf: Cpf) -> Result<(), DomainError> {
        let mut cliente_repository = self.cliente_repository.lock().await;
        cliente_repository.delete_cliente(cpf).await?;
        Ok(())
    }
}

unsafe impl Send for ClienteUseCase {}
unsafe impl Sync for ClienteUseCase {}
