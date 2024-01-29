use crate::core::domain::base::domain_error::DomainError;
use crate::core::domain::entities::produto::Produto;


#[async_trait]
pub trait ProdutoRepository {
  async fn get_produtos(&self) -> Result<Vec<Produto>, DomainError>;

  async fn get_produto_by_id(&self, id: usize) -> Result<Produto, DomainError>;

  async fn create_produto(&mut self, produto: Produto) -> Result<Produto, DomainError>;

  async fn update_produto(&mut self, new_produto_data: Produto) -> Result<Produto, DomainError>;

  async fn delete_produto(&mut self, id: usize) -> Result<(), DomainError>;
}