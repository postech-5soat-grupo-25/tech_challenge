use std::sync::Arc;
use rocket::futures::lock::Mutex;
use schemars::JsonSchema;
use serde::Deserialize;

use crate::core::domain::base::domain_error::DomainError;
use crate::core::domain::entities::product::Product;
use crate::core::domain::entities::product::Categoria;
use crate::core::domain::value_objects::ingredientes::Ingredientes;
use crate::core::domain::repositories::product_repository::ProductRepository;

#[derive(Clone, Debug, Deserialize, JsonSchema)]
pub struct CreateProductInput {
    nome: String,
    foto: String,
    descricao: String,
    categoria: Categoria,
    preco: f32,
    ingredientes: Ingredientes,
    created_at: String,
}

impl CreateProductInput {
    pub fn new(nome: String, foto: String, descricao: String, categoria: Categoria, preco: f32, ingredientes: Ingredientes, created_at: String) -> Self {
        Self {
            nome,
            foto,
            descricao,
            categoria,
            preco,
            ingredientes,
            created_at,
        }
    }
}

#[derive(Clone, Debug, Deserialize, JsonSchema)]
pub struct UpdateProductInput {
    nome: Option<String>,
    foto: Option<String>,
    descricao: Option<String>,
    categoria: Option<Categoria>,
    preco: Option<f32>,
    ingredientes: Option<Ingredientes>,
}

#[derive(Clone)]
pub struct ProductUseCase {
    product_repository: Arc<Mutex<dyn ProductRepository + Sync + Send>>,
}

impl ProductUseCase {
    pub fn new(product_repository: Arc<Mutex<dyn ProductRepository + Sync + Send>>) -> Self {
        ProductUseCase { product_repository }
    }

    pub async fn get_products(&self) -> Result<Vec<Product>, DomainError> {
        let product_repository = self.product_repository.lock().await;
        product_repository.get_products().await
    }

    pub async fn get_product_by_id(&self, id: usize) -> Result<Product, DomainError> {
        let product_repository = self.product_repository.lock().await;
        product_repository.get_product_by_id(id).await
    }

    pub async fn create_product(&self, product: CreateProductInput) -> Result<Product, DomainError> {
        let mut product_repository = self.product_repository.lock().await;
        let new_id = 0;  // Set the appropriate logic to generate a new ID
        let valid_categoria = Categoria::Bebida;

        let ingredientes_list = vec![String::from("Ingredient 1"), String::from("Ingredient 2")];
        let ingredientes = match Ingredientes::new(ingredientes_list) {
            Ok(ing) => ing,
            Err(e) => panic!("Failed to create Ingredientes: {:?}", e),
        };

        let product = product_repository.create_product(Product::new(
            new_id,
            product.nome,
            product.foto,
            product.descricao,
            valid_categoria,
            product.preco,
            ingredientes,
            product.created_at,
            "".to_string(), // Add the missing argument for updated_at
        )).await?;


        Ok(product.clone())
    }

    pub async fn update_product(&self, id: usize, fields_to_update: UpdateProductInput) -> Result<Product, DomainError> {
        let mut product_repository = self.product_repository.lock().await;
        let mut product = product_repository.get_product_by_id(id).await?;

        if let Some(nome) = fields_to_update.nome {
            product.set_nome(nome);
        }

        if let Some(foto) = fields_to_update.foto {
            product.set_foto(foto);
        }
        
        if let Some(descricao) = fields_to_update.descricao {
            product.set_descricao(descricao);
        }

        if let Some(categoria) = fields_to_update.categoria {
            product.set_categoria(categoria);
        }

        //TODO: Fazer update de Igredientes

        // Similar logic for other fields in UpdateProductInput
        product_repository.update_product(product).await
    }

    pub async fn delete_product(&self, id: usize) -> Result<(), DomainError> {
        let mut product_repository = self.product_repository.lock().await;
        product_repository.delete_product(id).await?;
        Ok(())
    }
}

unsafe impl Send for ProductUseCase {}
unsafe impl Sync for ProductUseCase {}
