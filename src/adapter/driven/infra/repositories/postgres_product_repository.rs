use crate::core::domain::{
        entities::product::Product,
        repositories::product_repository::ProductRepository,
        base::domain_error::DomainError,
    };

use crate::core::domain::entities::product::Categoria;
use crate::core::domain::value_objects::ingredientes::Ingredientes;

use std::sync::Arc;

use postgres_from_row::FromRow;
use tokio_postgres::Client;

use super::super::postgres::table::Table;

pub struct PostgresProductRepository {
    client: Arc<Client>,
    tables: Vec<Table>,
}

const CREATE_PRODUCT: &str = "INSERT INTO products (nome, foto, descricao, categoria, preco, ingredientes, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *";
const QUERY_PRODUCT_BY_ID: &str = "SELECT * FROM products WHERE id = $1";
const QUERY_PRODUCTS: &str = "SELECT * FROM products";
const UPDATE_PRODUCT: &str = "UPDATE products SET nome = $1, foto = $2, descricao = $3, categoria = $4, preco = $5, ingredientes = $6, created_at = $7, updated_at = $8 WHERE id = $10 RETURNING *";
const DELETE_PRODUCT: &str = "DELETE FROM products WHERE id = $1";

impl PostgresProductRepository {
    pub async fn new(client: Arc<Client>, tables: Vec<Table>) -> Self {
        let mut repo = PostgresProductRepository { client, tables };

        repo.check_for_tables().await;
        repo.create_first_product().await;

        repo
    }

    async fn check_for_tables(&self) {
        for table in self.tables.iter() {
          let queries = table.get_create_if_not_exists_query();
          for query in queries {
            self.client.execute(query.as_str(), &[]).await.unwrap();
        }
        };
      }
    
    // Função que criei pra testar a inserção no banco
    async fn create_first_product(&mut self) {
        let product = Product::new(
            1,
            "Product Name".to_string(),
            "Product Photo URL".to_string(),
            "Product Description".to_string(),
            Categoria::Bebida,
            10.0,
            Ingredientes::new(vec!["Ingredient 1".to_string(), "Ingredient 2".to_string()]).unwrap(),
            "2022-03-01".to_string(),
            "2022-03-01".to_string(),
        );

        self.create_product(product).await.unwrap();
    }
}

#[async_trait]
impl ProductRepository for PostgresProductRepository {
    async fn get_products(&self) -> Result<Vec<Product>, DomainError> {
        let products = self.client.query(QUERY_PRODUCTS, &[]).await.unwrap();
        let mut products_vec = Vec::new();
        for product in products {
            products_vec.push(Product::from_row(&product));
        }
        Ok(products_vec)
    }

    async fn get_product_by_id(&self, id: usize) -> Result<Product, DomainError> {
        let id = id as i32;
        let product = self.client.query_one(QUERY_PRODUCT_BY_ID, &[&id]).await;
        match product {
            Ok(product) => Ok(Product::from_row(&product)),
            Err(_) => Err(DomainError::NotFound),
        }
    }

    async fn create_product(&mut self, product: Product) -> Result<Product, DomainError> {

        let ingredientes = product.ingredientes();
        let ingredientes_json = tokio_postgres::types::Json(ingredientes);

        let categoria = product.categoria();
        let categoria_json = tokio_postgres::types::Json(categoria);

        let new_product = self
            .client
            .query(
                CREATE_PRODUCT,
                &[
                    &product.nome(),
                    &product.foto(),
                    &product.descricao(),
                    &categoria_json,
                    &product.preco(),
                    &ingredientes_json,
                    &product.created_at(),
                    &product.updated_at(),
                ],
            )
            .await
            .unwrap();
        let new_product = new_product.get(0);
        match new_product {
            Some(product) => Ok(Product::from_row(product)),
            None => Err(DomainError::Invalid("Product".to_string())),
        }
    }

    async fn update_product(&mut self, new_product_data: Product) -> Result<Product, DomainError> {
        let id = new_product_data.id().clone() as i32;

        let ingredientes = new_product_data.ingredientes();
        let ingredientes_json = tokio_postgres::types::Json(ingredientes);

        let categoria = new_product_data.categoria();
        let categoria_json = tokio_postgres::types::Json(categoria);

        let updated_product = self
            .client
            .query(
                UPDATE_PRODUCT,
                &[
                    &new_product_data.nome(),
                    &new_product_data.foto(),
                    &new_product_data.descricao(),
                    &categoria_json,
                    &new_product_data.preco(),
                    &ingredientes_json,
                    &new_product_data.created_at(),
                    &new_product_data.updated_at(),
                    &id,
                ],
            )
            .await
            .unwrap();
        let updated_product = updated_product.get(0);
        match updated_product {
            Some(product) => Ok(Product::from_row(product)),
            None => Err(DomainError::NotFound),
        }
    }

    async fn delete_product(&mut self, id: usize) -> Result<(), DomainError> {
        let id = id as i32;
        let deleted_product = self.client.query_one(DELETE_PRODUCT, &[&id]).await;
        match deleted_product {
            Ok(_) => Ok(()),
            _ => Err(DomainError::NotFound),
        }
    }
}
