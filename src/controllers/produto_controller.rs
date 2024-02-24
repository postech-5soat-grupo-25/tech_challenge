use std::sync::Arc;
use tokio::sync::Mutex;

use crate::base::domain_error::DomainError;
use crate::entities::produto::Produto;
use crate::traits::produto_gateway::ProdutoGateway;
use crate::use_cases::gerenciamento_de_produtos_use_case::{CreateProdutoInput, ProdutoUseCase};

pub struct ProdutoController {
    produto_use_case: ProdutoUseCase,
}

impl ProdutoController {
    pub fn new(produto_repository: Arc<Mutex<dyn ProdutoGateway + Sync + Send>>) -> ProdutoController {
        let produto_use_case = ProdutoUseCase::new(produto_repository);
        ProdutoController { produto_use_case }
    }

    pub async fn get_produto(
        &self,
    ) -> Result<Vec<Produto>, DomainError> {
        self.produto_use_case.get_produtos().await
    }

    pub async fn get_produto_by_id(
        &self,
        id: usize,
    ) -> Result<Produto, DomainError> {
        self.produto_use_case.get_produto_by_id(id).await
    }

    pub async fn create_produto(
        &self,
        produto_input: CreateProdutoInput,
    ) -> Result<Produto, DomainError> {
        self.produto_use_case.create_produto(produto_input).await
    }

    pub async fn update_produto(
        &self,
        id: usize,
        produto_input: CreateProdutoInput,
    ) -> Result<Produto, DomainError> {
        self.produto_use_case.update_produto(id, produto_input).await
    }

    pub async fn delete_produto(
        &self,
        id: usize,
    ) -> Result<(), DomainError> {
        self.produto_use_case.delete_produto(id).await
    }
}
