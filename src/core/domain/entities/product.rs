use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::str::FromStr;


use crate::core::domain::base::aggregate_root::AggregateRoot;
use crate::core::domain::value_objects::ingredientes::Ingredientes;
use crate::core::domain::base::assertion_concern;


#[derive(Clone, Serialize, Deserialize, Debug, JsonSchema, PartialEq)]
pub enum Categoria {
    Lanche,
    Bebida,
    Acompanhamento,
    Sobremesa,
}

impl FromStr for Categoria {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Lanche" => Ok(Categoria::Lanche),
            "Bebida" => Ok(Categoria::Bebida),
            "Acompanhamento" => Ok(Categoria::Acompanhamento),
            "Sobremesa" => Ok(Categoria::Sobremesa),
            _ => Err(()),
        }
    }
}


#[derive(Clone, Serialize, Deserialize, Debug, JsonSchema)]
pub struct Product {
    id: usize,
    nome: String,
    foto: String,
    descricao: String,
    categoria: Categoria,
    preco: f32,
    ingredientes: Ingredientes,
    created_at: String,
    updated_at: String,
}

impl AggregateRoot for Product {}

impl Product {
    pub fn new(
        id: usize,
        nome: String,
        foto: String,
        descricao: String,
        categoria: Categoria,
        preco: f32,
        ingredientes: Ingredientes,
        created_at: String,
        updated_at: String,
    ) -> Self {
        Product {
            id,
            nome,
            foto,
            descricao,
            categoria,
            preco,
            ingredientes,
            created_at,
            updated_at,
        }
    }
    
    fn validate_entity(&self) -> Result<(), String> {
        assertion_concern::assert_argument_not_empty(self.nome.clone(), "Nome não pode ser vazio".to_string());
        assertion_concern::assert_argument_not_empty(self.preco.clone().to_string(), "Preço não pode ser vazio".to_string());
        assertion_concern::assert_argument_date_format(&self.created_at, "Data de criação não está no formato correto (YYYY-MM-DD)".to_string());
        assertion_concern::assert_argument_date_format(&self.updated_at, "Data de atualização não está no formato correto (YYYY-MM-DD)".to_string());
        Ok(())
    }

    // Getters
    pub fn id(&self) -> &usize {
        &self.id
      }

    pub fn nome(&self) -> &String {
        &self.nome
    }

    pub fn foto(&self) -> &String {
        &self.foto
    }

    pub fn descricao(&self) -> &String {
        &self.descricao
    }

    pub fn categoria(&self) -> &Categoria {
        &self.categoria
    }

    pub fn preco(&self) -> f32 {
        self.preco
    }

    pub fn ingredientes(&self) -> &Ingredientes {
        &self.ingredientes
    }

    pub fn created_at(&self) -> &String {
        &self.created_at
    }

    pub fn updated_at(&self) -> &String {
        &self.updated_at
    }

    // Setters
    pub fn set_nome(&mut self, nome: String) {
        self.nome = nome;
    }

    pub fn set_foto(&mut self, foto: String) {
        self.foto = foto;
    }

    pub fn set_descricao(&mut self, descricao: String) {
        self.descricao = descricao;
    }

    pub fn set_categoria(&mut self, categoria: Categoria) {
        self.categoria = categoria;
    }

    pub fn set_preco(&mut self, preco: f32) {
        self.preco = preco;
    }

    pub fn set_ingredientes(&mut self, ingredientes: Ingredientes) {
        self.ingredientes = ingredientes;
    }

    pub fn set_created_at(&mut self, created_at: String) {
        self.created_at = created_at;
    }

    pub fn set_updated_at(&mut self, updated_at: String) {
        self.updated_at = updated_at;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_product_creation_valid() {
        let ingredientes = Ingredientes::new(vec!["Pão".to_string(), "Hambúrguer".to_string(), "Queijo".to_string()]).unwrap();
        let product = Product::new(
            1,
            "Cheeseburger".to_string(),
            "cheeseburger.png".to_string(),
            "O clássico pão, carne e queijo!".to_string(),
            Categoria::Lanche,
            9.99,
            ingredientes,
            "2024-01-16".to_string(),
            "2024-01-16".to_string(),
        );

        assert_eq!(product.id(), &1);
        assert_eq!(product.nome(), "Cheeseburger");
        assert_eq!(product.foto(), "cheeseburger.png");
        assert_eq!(product.descricao(), "O clássico pão, carne e queijo!");
        assert_eq!(product.categoria(), &Categoria::Lanche);
        assert_eq!(product.preco(), 9.99);
        assert_eq!(product.created_at(), "2024-01-16");
        assert_eq!(product.updated_at(), "2024-01-16");
    }

    #[test]
    fn test_product_validate_entity_valid() {
        let ingredientes = Ingredientes::new(vec!["Pão".to_string(), "Hambúrguer".to_string(), "Queijo".to_string()]).unwrap();
        let product = Product::new(
            1,
            "Cheeseburger".to_string(),
            "cheeseburger.png".to_string(),
            "O clássico pão, carne e queijo!".to_string(),
            Categoria::Lanche,
            9.99,
            ingredientes,
            "2024-01-16".to_string(),
            "2024-01-16".to_string(),
        );

        assert!(product.validate_entity().is_ok());
    }

    #[test]
    #[should_panic(expected = "Nome não pode ser vazio")]
    fn test_product_validate_entity_invalid_nome() {
        let ingredientes = Ingredientes::new(vec!["Pão".to_string(), "Hambúrguer".to_string(), "Queijo".to_string()]).unwrap();
        let product = Product::new(
            1,
            "".to_string(),
            "cheeseburger.png".to_string(),
            "O clássico pão, carne e queijo!".to_string(),
            Categoria::Lanche,
            9.99,
            ingredientes,
            "2024-01-16".to_string(),
            "2024-01-16".to_string(),
        );

        product.validate_entity();
    }

    #[test]
    #[should_panic(expected = "Data de criação não está no formato correto (YYYY-MM-DD)")]
    fn test_product_validate_entity_invalid_created_at() {
        let ingredientes = Ingredientes::new(vec!["Pão".to_string(), "Hambúrguer".to_string(), "Queijo".to_string()]).unwrap();
        let product = Product::new(
            1,
            "Cheeseburger".to_string(),
            "cheeseburger.png".to_string(),
            "O clássico pão-carne-queijo!".to_string(),
            Categoria::Lanche,
            9.99,
            ingredientes,
            "16-01-2024".to_string(),
            "2024-01-16".to_string(),
        );

        product.validate_entity();
    }
}