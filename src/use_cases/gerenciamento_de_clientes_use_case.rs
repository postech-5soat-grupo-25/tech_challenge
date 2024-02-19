use chrono::Utc;
use tokio::sync::Mutex;
use schemars::JsonSchema;
use serde::Deserialize;
use std::sync::Arc;

use crate::base::domain_error::DomainError;
use crate::entities::{
    cliente::Cliente,
    cpf::Cpf,
};
use crate::traits::cliente_gateway::ClienteGateway;

#[derive(Clone, Debug, Deserialize, JsonSchema)]
pub struct CreateClienteInput {
    nome: String,
    email: String,
    cpf: String,
}

#[derive(Clone)]
pub struct ClienteUseCase {
    cliente_repository: Arc<Mutex<dyn ClienteGateway + Sync + Send>>,
}

impl ClienteUseCase {
    pub fn new(cliente_repository: Arc<Mutex<dyn ClienteGateway + Sync + Send>>) -> Self {
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

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;
    use crate::entities::cliente::Cliente;
    use crate::traits::cliente_gateway::MockClienteGateway;
    use tokio::sync::Mutex;
    use std::sync::Arc;
    use tokio;

    #[tokio::test]
    async fn test_get_clientes() {
        let mut mock = MockClienteGateway::new();

        let returned_cliente = Cliente::new(
            1,
            "nome".to_string(),
            "email".to_string(),
            Cpf::new("000.000.000-00".to_string()).unwrap(),
            "2021-10-10".to_string(),
            "2021-10-10".to_string(),
        );

        let expected_cliente = returned_cliente.clone();

        mock.expect_get_clientes()
            .times(1)
            .returning(move || Ok(vec![returned_cliente.clone()]));

        let use_case = ClienteUseCase::new(Arc::new(Mutex::new(mock)));
        let result = use_case.get_clientes().await;
        assert_eq!(result.unwrap()[0].id(), expected_cliente.id());
    }

    #[tokio::test]
    async fn test_get_cliente_by_cpf() {
        let mut mock = MockClienteGateway::new();

        let returned_cliente = Cliente::new(
            1,
            "nome".to_string(),
            "email".to_string(),
            Cpf::new("000.000.000-00".to_string()).unwrap(),
            "2021-10-10".to_string(),
            "2021-10-10".to_string(),
        );

        let expected_cliente = returned_cliente.clone();

        mock.expect_get_cliente_by_cpf()
            .times(1)
            .with(eq(Cpf::new("000.000.000-00".to_string()).unwrap()))
            .returning(move |_| Ok(returned_cliente.clone()));

        let use_case = ClienteUseCase::new(Arc::new(Mutex::new(mock)));
        let result = use_case.get_cliente_by_cpf(Cpf::new("000.000.000-00".to_string()).unwrap()).await;
        assert_eq!(result.unwrap().id(), expected_cliente.id());
    }

    #[tokio::test]
    async fn test_create_cliente() {
        let mut mock = MockClienteGateway::new();

        let returned_cliente = Cliente::new(
            1,
            "nome".to_string(),
            "email".to_string(),
            Cpf::new("000.000.000-00".to_string()).unwrap(),
            "2021-10-10".to_string(),
            "2021-10-10".to_string(),
        );

        let expected_cliente = returned_cliente.clone();

        mock.expect_create_cliente()
            .times(1)
            .returning(move |_| Ok(returned_cliente.clone()));

        let use_case = ClienteUseCase::new(Arc::new(Mutex::new(mock)));
        let result = use_case.create_cliente(CreateClienteInput {
            nome: "nome".to_string(),
            email: "email".to_string(),
            cpf: "000.000.000-00".to_string(),
        }).await;

        assert_eq!(result.unwrap().id(), expected_cliente.id());
    }

    #[tokio::test]
    async fn test_delete_cliente() {
        let mut mock = MockClienteGateway::new();

        mock.expect_delete_cliente()
            .times(1)
            .with(eq(Cpf::new("000.000.000-00".to_string()).unwrap()))
            .returning(move |_| Ok(()));

        let use_case = ClienteUseCase::new(Arc::new(Mutex::new(mock)));
        let result = use_case.delete_cliente(Cpf::new("000.000.000-00".to_string()).unwrap()).await;
        assert_eq!(result.unwrap(), ());
    }
}