use crate::core::domain::base::domain_error::DomainError;

use crate::core::domain::entities::produto::{Produto, Categoria};

#[async_trait]
pub trait ProdutoRepository {
    async fn create_produto(&mut self, produto: Produto) -> Result<Produto, DomainError>;

    async fn get_produto_by_id(&self, id: usize) -> Result<Produto, DomainError>;

    async fn get_produtos_by_categoria(&self, categoria: Categoria) -> Result<Vec<Produto>, DomainError>;
}