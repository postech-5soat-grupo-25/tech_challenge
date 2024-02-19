use chrono::Utc;

use tokio::sync::Mutex;
use schemars::JsonSchema;
use serde::Deserialize;
use std::sync::Arc;

use crate::base::domain_error::DomainError;
use crate::entities::{
    ingredientes::Ingredientes,
    produto::{Categoria, Produto},
};
use crate::traits::produto_repository::ProdutoRepository;

#[derive(Clone, Debug, Deserialize, JsonSchema)]
pub struct CreateProdutoInput {
    nome: String,
    foto: String,
    descricao: String,
    categoria: Categoria,
    preco: f64,
    ingredientes: Ingredientes,
}

impl CreateProdutoInput {
    pub fn new(
        nome: String,
        foto: String,
        descricao: String,
        categoria: Categoria,
        preco: f64,
        ingredientes: Ingredientes,
    ) -> Self {
        Self {
            nome,
            foto,
            descricao,
            categoria,
            preco,
            ingredientes,
        }
    }
}

#[derive(Clone, Debug, Deserialize, JsonSchema)]
pub struct UpdateProdutoInput {
    nome: Option<String>,
    foto: Option<String>,
    descricao: Option<String>,
    categoria: Option<Categoria>,
    preco: Option<f32>,
    ingredientes: Option<Ingredientes>,
}

#[derive(Clone)]
pub struct ProdutoUseCase {
    produto_repository: Arc<Mutex<dyn ProdutoRepository + Sync + Send>>,
}

impl ProdutoUseCase {
    pub fn new(produto_repository: Arc<Mutex<dyn ProdutoRepository + Sync + Send>>) -> Self {
        ProdutoUseCase { produto_repository }
    }

    pub async fn get_produtos(&self) -> Result<Vec<Produto>, DomainError> {
        let produto_repository = self.produto_repository.lock().await;
        produto_repository.get_produtos().await
    }

    pub async fn get_produto_by_id(&self, id: usize) -> Result<Produto, DomainError> {
        let produto_repository = self.produto_repository.lock().await;
        produto_repository.get_produto_by_id(id).await
    }

    pub async fn create_produto(
        &self,
        produto: CreateProdutoInput,
    ) -> Result<Produto, DomainError> {
        let mut produto_repository = self.produto_repository.lock().await;

        let _now = Utc::now().format("%Y-%m-%d %H:%M:%S%.3f%z").to_string();

        let produto = produto_repository
            .create_produto(Produto::new(
                0,
                produto.nome,
                produto.foto,
                produto.descricao,
                produto.categoria,
                produto.preco,
                produto.ingredientes.clone(),
                _now.clone(),
                _now,
            ))
            .await?;

        Ok(produto.clone())
    }

    pub async fn update_produto(
        &self,
        id: usize,
        fields_to_update: CreateProdutoInput,
    ) -> Result<Produto, DomainError> {
        let mut produto_repository = self.produto_repository.lock().await;

        let _now = Utc::now().format("%Y-%m-%d %H:%M:%S%.3f%z").to_string();
        let produto_atualizado = produto_repository
            .update_produto(Produto::new(
                id,
                fields_to_update.nome,
                fields_to_update.foto,
                fields_to_update.descricao,
                fields_to_update.categoria,
                fields_to_update.preco,
                fields_to_update.ingredientes,
                _now.clone(),
                _now,
            ))
            .await?;

        Ok(produto_atualizado.clone())
    }

    pub async fn delete_produto(&self, id: usize) -> Result<(), DomainError> {
        let mut produto_repository = self.produto_repository.lock().await;
        produto_repository.delete_produto(id).await?;
        Ok(())
    }
}

unsafe impl Send for ProdutoUseCase {}
unsafe impl Sync for ProdutoUseCase {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::{ingredientes::Ingredientes, produto::Produto};
    use crate::traits::produto_repository::MockProdutoRepository;
    use mockall::predicate::*;
    use tokio;

    #[tokio::test]
    async fn test_get_produtos() {
        let mut mock = MockProdutoRepository::new();

        let returned_produto = Produto::new(
            1,
            "nome".to_string(),
            "foto".to_string(),
            "descricao".to_string(),
            Categoria::Lanche,
            10.0,
            Ingredientes::new(vec!["ingrediente1".to_string(), "ingrediente2".to_string()])
                .unwrap(),
            "2021-10-10".to_string(),
            "2021-10-10".to_string(),
        );

        let expected_produto = returned_produto.clone();

        mock.expect_get_produtos()
            .times(1)
            .returning(move || Ok(vec![returned_produto.clone()]));

        let use_case = ProdutoUseCase::new(Arc::new(Mutex::new(mock)));
        let result = use_case.get_produtos().await;
        assert_eq!(result.unwrap()[0].id(), expected_produto.id());
    }

    #[tokio::test]
    async fn test_get_produto_by_id() {
        let mut mock = MockProdutoRepository::new();

        let returned_produto = Produto::new(
            1,
            "nome".to_string(),
            "foto".to_string(),
            "descricao".to_string(),
            Categoria::Lanche,
            10.0,
            Ingredientes::new(vec!["ingrediente1".to_string(), "ingrediente2".to_string()])
                .unwrap(),
            "2021-10-10".to_string(),
            "2021-10-10".to_string(),
        );

        let expected_produto = returned_produto.clone();

        mock.expect_get_produto_by_id()
            .times(1)
            .with(eq(1))
            .returning(move |_| Ok(returned_produto.clone()));

        let use_case = ProdutoUseCase::new(Arc::new(Mutex::new(mock)));
        let result = use_case.get_produto_by_id(1).await;
        assert_eq!(result.unwrap().id(), expected_produto.id());
    }

    #[tokio::test]
    async fn test_create_produto() {
        let mut mock = MockProdutoRepository::new();

        let returned_produto = Produto::new(
            1,
            "nome".to_string(),
            "foto".to_string(),
            "descricao".to_string(),
            Categoria::Lanche,
            10.0,
            Ingredientes::new(vec!["ingrediente1".to_string(), "ingrediente2".to_string()])
                .unwrap(),
            "2021-10-10".to_string(),
            "2021-10-10".to_string(),
        );

        let expected_produto = returned_produto.clone();

        mock.expect_create_produto()
            .times(1)
            .returning(move |_| Ok(returned_produto.clone()));

        let use_case = ProdutoUseCase::new(Arc::new(Mutex::new(mock)));
        let result = use_case
            .create_produto(CreateProdutoInput::new(
                "nome".to_string(),
                "foto".to_string(),
                "descricao".to_string(),
                Categoria::Lanche,
                10.0,
                Ingredientes::new(vec!["ingrediente1".to_string(), "ingrediente2".to_string()])
                    .unwrap(),
            ))
            .await;
        assert_eq!(result.unwrap().id(), expected_produto.id());
    }

    #[tokio::test]
    async fn test_update_produto() {
        let mut mock = MockProdutoRepository::new();

        let returned_produto = Produto::new(
            1,
            "nome".to_string(),
            "foto".to_string(),
            "descricao".to_string(),
            Categoria::Lanche,
            10.0,
            Ingredientes::new(vec!["ingrediente1".to_string(), "ingrediente2".to_string()])
                .unwrap(),
            "2021-10-10".to_string(),
            "2021-10-10".to_string(),
        );

        let expected_produto = returned_produto.clone();

        mock.expect_update_produto()
            .times(1)
            .returning(move |_| Ok(returned_produto.clone()));

        let use_case = ProdutoUseCase::new(Arc::new(Mutex::new(mock)));
        let result = use_case
            .update_produto(
                1,
                CreateProdutoInput::new(
                    "nome".to_string(),
                    "foto".to_string(),
                    "descricao".to_string(),
                    Categoria::Lanche,
                    10.0,
                    Ingredientes::new(vec!["ingrediente1".to_string(), "ingrediente2".to_string()])
                        .unwrap(),
                ),
            )
            .await;
        assert_eq!(result.unwrap().id(), expected_produto.id());
    }

    #[tokio::test]
    async fn test_delete_produto() {
        let mut mock = MockProdutoRepository::new();

        mock.expect_delete_produto()
            .times(1)
            .returning(move |_| Ok(()));

        let use_case = ProdutoUseCase::new(Arc::new(Mutex::new(mock)));
        let result = use_case.delete_produto(1).await;
        assert_eq!(result.unwrap(), ());
    }
}
