use crate::core::domain::{
    base::domain_error::DomainError, entities::produto::Produto,
    repositories::produto_repository::ProdutoRepository,
};

use crate::core::domain::entities::produto::Categoria;
use crate::core::domain::value_objects::ingredientes::Ingredientes;

use chrono::{DateTime, Utc};
use postgres_from_row::FromRow;
use serde_json::{json, Value};
use std::sync::Arc;
use std::time::SystemTime;
use tokio_postgres::types::{FromSql, ToSql, Type};
use tokio_postgres::Client;

use super::super::postgres::table::Table;

pub struct PostgresProdutoRepository {
    client: Client,
    tables: Vec<Table>,
}

const ALL_PRODUCT_SELECT: &str =
    "id, nome, foto, descricao, categoria, preco, ingredientes, data_criacao::TEXT, data_atualizacao::TEXT";
const CREATE_PRODUCT: &str = "INSERT INTO produto (nome, foto, descricao, categoria, preco, ingredientes, data_criacao, data_atualizacao) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *";
const QUERY_PRODUCT_BY_ID: &str = "SELECT * FROM produto WHERE id = $1";
const QUERY_PRODUCTS: &str = "SELECT * FROM produto";
const QUERY_PRODUCT_BY_CATEGORIA: &str = "SELECT * FROM produto WHERE categoria = $1";
const UPDATE_PRODUCT: &str = "UPDATE produto SET nome = $1, foto = $2, descricao = $3, categoria = $4, preco = $5, ingredientes = $6, data_atualizacao = $7 WHERE id = $8 RETURNING *";
const DELETE_PRODUCT: &str = "DELETE FROM produto WHERE id = $1";

impl PostgresProdutoRepository {
    pub async fn new(client: Client, tables: Vec<Table>) -> Self {
        let repo = PostgresProdutoRepository { client, tables };
        repo.check_for_tables().await;
        repo
    }

    async fn check_for_tables(&self) {
        for table in self.tables.iter() {
            let query = table.get_create_if_not_exists_query();
            self.client.execute(query.as_str(), &[]).await.unwrap();
        }
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

    async fn get_produtos_by_categoria(
        &self,
        categoria: Categoria,
    ) -> Result<Vec<Produto>, DomainError> {
        let categoria = tokio_postgres::types::Json(categoria);
        let lista_produtos = self
            .client
            .query(QUERY_PRODUCT_BY_CATEGORIA, &[&categoria])
            .await
            .unwrap();
        let mut produtos_vec = Vec::new();
        for produto in lista_produtos {
            produtos_vec.push(Produto::from_row(&produto));
        }
        Ok(produtos_vec)
    }

    async fn create_produto(&mut self, produto: Produto) -> Result<Produto, DomainError> {
        let ingredientes = produto.ingredientes();
        let ingredientes_vec: Vec<String> = ingredientes.to_vec_string();
        let _now: SystemTime = SystemTime::now();
        let new_produto = self
            .client
            .query(
                CREATE_PRODUCT,
                &[
                    &produto.nome(),
                    &produto.foto(),
                    &produto.descricao(),
                    &produto.categoria(),
                    &produto.preco(),
                    &ingredientes_vec,
                    &_now.clone(),
                    &_now,
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
        let ingredientes_vec: Vec<String> = ingredientes.to_vec_string();
        let _now: SystemTime = SystemTime::now();

        let updated_produto = self
            .client
            .query(
                UPDATE_PRODUCT,
                &[
                    &new_produto_data.nome(),
                    &new_produto_data.foto(),
                    &new_produto_data.descricao(),
                    &new_produto_data.categoria(),
                    &new_produto_data.preco(),
                    &ingredientes_vec,
                    &_now,
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
