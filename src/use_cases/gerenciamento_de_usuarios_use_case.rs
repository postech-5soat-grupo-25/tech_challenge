use std::sync::Arc;

use chrono::Utc;

use schemars::JsonSchema;
use serde::Deserialize;
use tokio::sync::Mutex;

use crate::base::domain_error::DomainError;
use crate::entities::cpf::Cpf;
use crate::entities::usuario::{Status, Tipo, Usuario};
use crate::traits::usuario_gateway::UsuarioGateway;

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
    usuario_repository: Arc<Mutex<dyn UsuarioGateway + Sync + Send>>,
}

impl UsuarioUseCase {
    pub fn new(usuario_repository: Arc<Mutex<dyn UsuarioGateway + Sync + Send>>) -> Self {
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
        let _id = 0;
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
        id: usize,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::usuario_gateway::MockUsuarioGateway;
    use tokio;

    #[tokio::test]
    async fn test_get_usuarios() {
        let mut mock = MockUsuarioGateway::new();

        let returned_usuario = Usuario::new(
            1,
            "nome".to_string(),
            "email".to_string(),
            Cpf::new("000.000.000-00".to_string()).unwrap(),
            "senha".to_string(),
            Tipo::Cozinha,
            Status::Ativo,
            "2021-10-10".to_string(),
            "2021-10-10".to_string(),
        );

        let expected_usuario = returned_usuario.clone();

        mock.expect_get_usuarios()
            .times(1)
            .returning(move || Ok(vec![returned_usuario.clone()]));

        let use_case = UsuarioUseCase::new(Arc::new(Mutex::new(mock)));
        let result = use_case.get_usuarios().await;
        assert_eq!(result.unwrap()[0].id(), expected_usuario.id());
    }

    #[tokio::test]
    async fn test_get_usuario_by_id() {
        let mut mock = MockUsuarioGateway::new();

        let returned_usuario = Usuario::new(
            1,
            "nome".to_string(),
            "email".to_string(),
            Cpf::new("000.000.000-00".to_string()).unwrap(),
            "senha".to_string(),
            Tipo::Admin,
            Status::Ativo,
            "2021-10-10".to_string(),
            "2021-10-10".to_string(),
        );

        let expected_usuario = returned_usuario.clone();

        mock.expect_get_usuario_by_id()
            .times(1)
            .returning(move |_| Ok(returned_usuario.clone()));

        let use_case = UsuarioUseCase::new(Arc::new(Mutex::new(mock)));
        let result = use_case.get_usuario_by_id(1).await;
        assert_eq!(result.unwrap().id(), expected_usuario.id());
    }

    #[tokio::test]
    async fn test_get_usuario_by_cpf() {
        let mut mock = MockUsuarioGateway::new();

        let returned_usuario = Usuario::new(
            1,
            "nome".to_string(),
            "email".to_string(),
            Cpf::new("000.000.000-00".to_string()).unwrap(),
            "senha".to_string(),
            Tipo::Admin,
            Status::Ativo,
            "2021-10-10".to_string(),
            "2021-10-10".to_string(),
        );

        let expected_usuario = returned_usuario.clone();

        mock.expect_get_usuario_by_cpf()
            .times(1)
            .returning(move |_| Ok(returned_usuario.clone()));

        let use_case = UsuarioUseCase::new(Arc::new(Mutex::new(mock)));
        let result = use_case
            .get_usuario_by_cpf(Cpf::new("000.000.000-00".to_string()).unwrap())
            .await;
        assert_eq!(result.unwrap().id(), expected_usuario.id());
    }

    #[tokio::test]
    async fn test_create_usuario() {
        let mut mock = MockUsuarioGateway::new();

        let returned_usuario = Usuario::new(
            1,
            "nome".to_string(),
            "email".to_string(),
            Cpf::new("000.000.000-00".to_string()).unwrap(),
            "senha".to_string(),
            Tipo::Admin,
            Status::Ativo,
            "2021-10-10".to_string(),
            "2021-10-10".to_string(),
        );

        let expected_usuario = returned_usuario.clone();

        mock.expect_create_usuario()
            .times(1)
            .returning(move |_| Ok(returned_usuario.clone()));

        let use_case = UsuarioUseCase::new(Arc::new(Mutex::new(mock)));
        let result = use_case
            .create_usuario(CreateUsuarioInput {
                nome: "nome".to_string(),
                email: "email".to_string(),
                senha: "senha".to_string(),
                cpf: "000.000.000-00".to_string(),
                tipo: "Admin".to_string(),
                status: "Ativo".to_string(),
            })
            .await;
        assert_eq!(result.unwrap().id(), expected_usuario.id());
    }

    #[tokio::test]
    async fn test_update_usuario() {
        let mut mock = MockUsuarioGateway::new();

        let returned_usuario = Usuario::new(
            1,
            "nome".to_string(),
            "email".to_string(),
            Cpf::new("000.000.000-00".to_string()).unwrap(),
            "senha".to_string(),
            Tipo::Admin,
            Status::Ativo,
            "2021-10-10".to_string(),
            "2021-10-10".to_string(),
        );

        let expected_usuario = returned_usuario.clone();

        mock.expect_update_usuario()
            .times(1)
            .returning(move |_| Ok(returned_usuario.clone()));

        let use_case = UsuarioUseCase::new(Arc::new(Mutex::new(mock)));
        let result = use_case
            .update_usuario(
                1,
                CreateUsuarioInput {
                    nome: "nome".to_string(),
                    email: "email".to_string(),
                    senha: "senha".to_string(),
                    cpf: "000.000.000-00".to_string(),
                    tipo: "Cozinha".to_string(),
                    status: "Ativo".to_string(),
                },
            )
            .await;
        assert_eq!(result.unwrap().id(), expected_usuario.id());
    }

    #[tokio::test]
    async fn test_delete_usuario() {
        let mut mock = MockUsuarioGateway::new();

        mock.expect_delete_usuario()
            .times(1)
            .returning(move |_| Ok(()));

        let use_case = UsuarioUseCase::new(Arc::new(Mutex::new(mock)));
        let result = use_case
            .delete_usuario(Cpf::new("000.000.000-00".to_string()).unwrap())
            .await;
        assert!(result.is_ok());
    }
}
