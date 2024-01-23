use crate::core::domain::base::domain_error::DomainError;
use crate::core::domain::entities::product::Product;


#[async_trait]
pub trait ProductRepository {
  async fn get_products(&self) -> Result<Vec<Product>, DomainError>;

  async fn get_product_by_id(&self, id: usize) -> Result<Product, DomainError>;

  async fn create_product(&mut self, product: Product) -> Result<Product, DomainError>;

  async fn update_product(&mut self, new_product_data: Product) -> Result<Product, DomainError>;

  async fn delete_product(&mut self, id: usize) -> Result<(), DomainError>;
}