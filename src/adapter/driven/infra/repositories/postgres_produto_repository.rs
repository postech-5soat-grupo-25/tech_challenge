use crate::core::domain::{
    entities::produto::Produto,
    repositories::produto_repository::ProdutoRepository,
    base::domain_error::DomainError,
};

use crate::core::domain::entities::produto::Categoria;
use crate::core::domain::value_objects::ingredientes::Ingredientes;

use std::sync::Arc;

use postgres_from_row::FromRow;
use tokio_postgres::Client;

use super::super::postgres::table::Table;

pub struct PostgresProdutoRepository {
client: Arc<Client>,
tables: Vec<Table>,
}

const CREATE_PRODUCT: &str = "INSERT INTO produtos (nome, foto, descricao, categoria, preco, ingredientes, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *";
const QUERY_PRODUCT_BY_ID: &str = "SELECT * FROM produtos WHERE id = $1";
const QUERY_PRODUCTS: &str = "SELECT * FROM produtos";
const UPDATE_PRODUCT: &str = "UPDATE produtos SET nome = $1, foto = $2, descricao = $3, categoria = $4, preco = $5, ingredientes = $6, created_at = $7, updated_at = $8 WHERE id = $10 RETURNING *";
const DELETE_PRODUCT: &str = "DELETE FROM produtos WHERE id = $1";

impl PostgresProdutoRepository {
pub async fn new(client: Arc<Client>, tables: Vec<Table>) -> Self {
    let mut repo = PostgresProdutoRepository { client, tables };

    repo.check_for_tables().await;

    repo
}

    async fn check_for_tables(&self) {
    for table in self.tables.iter() {
        let query = table.get_create_if_not_exists_query();
        self.client.execute(query.as_str(), &[]).await.unwrap();
    };
  }
}

#[async_trait]
impl ProdutoRepository for PostgresProdutoRepository {
async fn get_produtos(&self) -> Result<Vec<Produto>, DomainError> {
    let produtos = self.client.query(QUERY_PRODUCTS, &[]).await.unwrap();
    let mut produtos_vec = Vec::new();
    for produto in produtos {
        produtos_vec.push(Produto::from_row(&produto));
    }
    Ok(produtos_vec)
}

async fn get_produto_by_id(&self, id: usize) -> Result<Produto, DomainError> {
    let id = id as i32;
    let produto = self.client.query_one(QUERY_PRODUCT_BY_ID, &[&id]).await;
    match produto {
        Ok(produto) => Ok(Produto::from_row(&produto)),
        Err(_) => Err(DomainError::NotFound),
    }
}

async fn create_produto(&mut self, produto: Produto) -> Result<Produto, DomainError> {

    let ingredientes = produto.ingredientes();
    let ingredientes_json = tokio_postgres::types::Json(ingredientes);

    let categoria = produto.categoria();
    let categoria_json = tokio_postgres::types::Json(categoria);

    let new_produto = self
        .client
        .query(
            CREATE_PRODUCT,
            &[
                &produto.nome(),
                &produto.foto(),
                &produto.descricao(),
                &categoria_json,
                &produto.preco(),
                &ingredientes_json,
                &produto.data_criacao(),
                &produto.data_atualizacao(),
            ],
        )
        .await
        .unwrap();
    let new_produto = new_produto.get(0);
    match new_produto {
        Some(produto) => Ok(Produto::from_row(produto)),
        None => Err(DomainError::Invalid("Produto".to_string())),
    }
}

async fn update_produto(&mut self, new_produto_data: Produto) -> Result<Produto, DomainError> {
    let id = new_produto_data.id().clone() as i32;

    let ingredientes = new_produto_data.ingredientes();
    let ingredientes_json = tokio_postgres::types::Json(ingredientes);

    let categoria = new_produto_data.categoria();
    let categoria_json = tokio_postgres::types::Json(categoria);

    let updated_produto = self
        .client
        .query(
            UPDATE_PRODUCT,
            &[
                &new_produto_data.nome(),
                &new_produto_data.foto(),
                &new_produto_data.descricao(),
                &categoria_json,
                &new_produto_data.preco(),
                &ingredientes_json,
                &new_produto_data.data_criacao(),
                &new_produto_data.data_atualizacao(),
                &id,
            ],
        )
        .await
        .unwrap();
    let updated_produto = updated_produto.get(0);
    match updated_produto {
        Some(produto) => Ok(Produto::from_row(produto)),
        None => Err(DomainError::NotFound),
    }
}

async fn delete_produto(&mut self, id: usize) -> Result<(), DomainError> {
    let id = id as i32;
    let deleted_produto = self.client.query_one(DELETE_PRODUCT, &[&id]).await;
    match deleted_produto {
        Ok(_) => Ok(()),
        _ => Err(DomainError::NotFound),
    }
}
}