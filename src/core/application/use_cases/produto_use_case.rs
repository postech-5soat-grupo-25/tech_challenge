
use chrono::Utc;

use std::sync::Arc;
use rocket::futures::lock::Mutex;
use schemars::JsonSchema;
use serde::Deserialize;

use crate::core::domain::base::domain_error::DomainError;
use crate::core::domain::entities::produto::Produto;
use crate::core::domain::entities::produto::Categoria;
use crate::core::domain::value_objects::ingredientes::Ingredientes;
use crate::core::domain::repositories::produto_repository::ProdutoRepository;

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
    pub fn new(nome: String, foto: String, descricao: String, categoria: Categoria, preco: f64, ingredientes: Ingredientes) -> Self {
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

    pub async fn create_produto(&self, produto: CreateProdutoInput) -> Result<Produto, DomainError> {
        let mut produto_repository = self.produto_repository.lock().await;
        let valid_categoria = Categoria::Bebida;

        let _now = Utc::now().format("%Y-%m-%d %H:%M:%S%.3f%z").to_string();

        let produto = produto_repository.create_produto(Produto::new(
            0,
            produto.nome,
            produto.foto,
            produto.descricao,
            valid_categoria,
            produto.preco,
            produto.ingredientes.clone(),
            _now.clone(),
            _now,
        )).await?;


        Ok(produto.clone())
    }

    pub async fn update_produto(&self, id: usize, fields_to_update: UpdateProdutoInput) -> Result<Produto, DomainError> {
        let mut produto_repository = self.produto_repository.lock().await;
        let mut produto = produto_repository.get_produto_by_id(id).await?;

        if let Some(nome) = fields_to_update.nome {
            produto.set_nome(nome);
        }

        if let Some(foto) = fields_to_update.foto {
            produto.set_foto(foto);
        }

        if let Some(descricao) = fields_to_update.descricao {
            produto.set_descricao(descricao);
        }

        if let Some(categoria) = fields_to_update.categoria {
            produto.set_categoria(categoria);
        }

        if let Some(ingredientes) = fields_to_update.ingredientes {
            produto.set_ingredientes(ingredientes);
        }

        produto_repository.update_produto(produto).await
    }

    pub async fn delete_produto(&self, id: usize) -> Result<(), DomainError> {
        let mut produto_repository = self.produto_repository.lock().await;
        produto_repository.delete_produto(id).await?;
        Ok(())
    }
}

unsafe impl Send for ProdutoUseCase {}
unsafe impl Sync for ProdutoUseCase {}