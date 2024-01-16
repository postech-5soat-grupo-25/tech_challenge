use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

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

#[derive(Clone, Serialize, Deserialize, Debug, JsonSchema)]
pub struct Produto {
    id: usize,
    nome: String,
    foto: String,
    descricao: String,
    categoria: Categoria,
    preco: f32,
    ingredientes: Ingredientes,
    data_criacao: String,
    data_atualizacao: String,
}

impl AggregateRoot for Produto {}

impl Produto {
    pub fn new(
        id: usize,
        nome: String,
        foto: String,
        descricao: String,
        categoria: Categoria,
        preco: f32,
        ingredientes: Ingredientes,
        data_criacao: String,
        data_atualizacao: String,
    ) -> Self {
        Produto {
            id,
            nome,
            foto,
            descricao,
            categoria,
            preco,
            ingredientes,
            data_criacao,
            data_atualizacao,
        }
    }
    
    fn validate_entity(&self) -> Result<(), String> {
        assertion_concern::assert_argument_not_empty(self.nome.clone(), "Nome não pode ser vazio".to_string());
        assertion_concern::assert_argument_not_empty(self.preco.clone().to_string(), "Preço não pode ser vazio".to_string());
        assertion_concern::assert_argument_date_format(&self.data_criacao, "Data de criação não está no formato correto (YYYY-MM-DD)".to_string());
        assertion_concern::assert_argument_date_format(&self.data_atualizacao, "Data de atualização não está no formato correto (YYYY-MM-DD)".to_string());
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

    pub fn data_criacao(&self) -> &String {
        &self.data_criacao
    }

    pub fn data_atualizacao(&self) -> &String {
        &self.data_atualizacao
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

    pub fn set_data_criacao(&mut self, data_criacao: String) {
        self.data_criacao = data_criacao;
    }

    pub fn set_data_atualizacao(&mut self, data_atualizacao: String) {
        self.data_atualizacao = data_atualizacao;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_produto_creation_valid() {
        let ingredientes = Ingredientes::new(vec!["Pão".to_string(), "Hambúrguer".to_string(), "Queijo".to_string()]).unwrap();
        let produto = Produto::new(
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

        assert_eq!(produto.id(), &1);
        assert_eq!(produto.nome(), "Cheeseburger");
        assert_eq!(produto.foto(), "cheeseburger.png");
        assert_eq!(produto.descricao(), "O clássico pão, carne e queijo!");
        assert_eq!(produto.categoria(), &Categoria::Lanche);
        assert_eq!(produto.preco(), 9.99);
        assert_eq!(produto.data_criacao(), "2024-01-16");
        assert_eq!(produto.data_atualizacao(), "2024-01-16");
    }

    #[test]
    fn test_produto_validate_entity_valid() {
        let ingredientes = Ingredientes::new(vec!["Pão".to_string(), "Hambúrguer".to_string(), "Queijo".to_string()]).unwrap();
        let produto = Produto::new(
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

        assert!(produto.validate_entity().is_ok());
    }

    #[test]
    #[should_panic(expected = "Nome não pode ser vazio")]
    fn test_produto_validate_entity_invalid_nome() {
        let ingredientes = Ingredientes::new(vec!["Pão".to_string(), "Hambúrguer".to_string(), "Queijo".to_string()]).unwrap();
        let produto = Produto::new(
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

        produto.validate_entity();
    }

    #[test]
    #[should_panic(expected = "Data de criação não está no formato correto (YYYY-MM-DD)")]
    fn test_produto_validate_entity_invalid_data_criacao() {
        let ingredientes = Ingredientes::new(vec!["Pão".to_string(), "Hambúrguer".to_string(), "Queijo".to_string()]).unwrap();
        let produto = Produto::new(
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

        produto.validate_entity();
    }
}