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

    pub fn validate_entity(&self) -> Result<(), String> {
        match self.categoria {
            Categoria::Lanche | Categoria::Acompanhamento | Categoria::Bebida | Categoria::Sobremesa => (),
            _ => return Err("Categoria do Produto é inválida".to_string()),
        };
        assertion_concern::assert_argument_not_empty(
            self.nome.clone(), "Nome não pode ser vazio".to_string()
        );
        assertion_concern::assert_argument_not_empty(
            self.descricao.clone(), "Descrição não pode ser vazio".to_string()
        );
        assertion_concern::assert_argument_not_negative(
            self.preco.clone(), "Preço não pode ser negativo".to_string()
        );
        assertion_concern::assert_argument_date_format(
            self.data_criacao.clone(), "Data de criação não está no formato correto (YYYY-MM-DD)".to_string()
        );
        assertion_concern::assert_argument_date_format(
            self.data_atualizacao.clone(), "Data de atualização não está no formato correto (YYYY-MM-DD)".to_string()
        );
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

    pub fn data_criacao(&self) -> &String {
        &self.data_criacao
    }

    pub fn data_atualizacao(&self) -> &String {
        &self.data_atualizacao
    }

    // Setters
    pub fn set_nome(&mut self, nome: String) {
        assertion_concern::assert_argument_not_empty(
            nome.clone(), "Nome não pode ser vazio".to_string()
        );
        self.nome = nome;
    }

    pub fn set_foto(&mut self, foto: String) {
        self.foto = foto;
    }

    pub fn set_descricao(&mut self, descricao: String) {
        assertion_concern::assert_argument_not_empty(
            descricao.clone(), "Descrição não pode ser vazio".to_string()
        );
        self.descricao = descricao;
    }

    pub fn set_categoria(&mut self, categoria: Categoria) {
        self.categoria = categoria;
    }

    pub fn set_preco(&mut self, preco: f32) {
        assertion_concern::assert_argument_not_negative(
            preco.clone(), "Preço não pode ser negativo".to_string()
        );
        self.preco = preco;
    }

    pub fn set_ingredientes(&mut self, ingredientes: Ingredientes) {
        self.ingredientes = ingredientes;
    }

    pub fn set_data_criacao(&mut self, data_criacao: String) {
        assertion_concern::assert_argument_date_format(
            data_criacao.clone(), "Data de criação não está no formato correto (YYYY-MM-DD)".to_string()
        );
        self.data_criacao = data_criacao;
    }

    pub fn set_data_atualizacao(&mut self, data_atualizacao: String) {
        assertion_concern::assert_argument_date_format(
            data_atualizacao.clone(), "Data de atualização não está no formato correto (YYYY-MM-DD)".to_string()
        );
        self.data_atualizacao = data_atualizacao;
    }
}

// Unit Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::domain::value_objects::ingredientes::Ingredientes;

    #[test]
    fn test_produto_creation_valid() {
        let ingredientes = Ingredientes::new(
            vec!["Pão".to_string(), "Hambúrguer".to_string(), "Queijo".to_string()]
        ).unwrap();
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
    fn test_produto_validate_entity_empty_nome() {
        let ingredientes = Ingredientes::new(vec!["Pão".to_string(), "Hambúrguer".to_string(), "Queijo".to_string()]).unwrap();
        let mut produto = Produto::new(
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

        produto.nome = "".to_string();
        produto.validate_entity().unwrap();
    }

    #[test]
    #[should_panic(expected = "Preço não pode ser negativo")]
    fn test_produto_validate_entity_negative_preco() {
        let ingredientes = Ingredientes::new(vec!["Pão".to_string(), "Hambúrguer".to_string(), "Queijo".to_string()]).unwrap();
        let mut produto = Produto::new(
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

        produto.preco = -10.0;
        produto.validate_entity().unwrap();
    }

    #[test]
    fn test_produto_setters_valid() {
        let mut produto = Produto::new(
            1,
            "Cheeseburger".to_string(),
            "cheeseburger.png".to_string(),
            "O clássico pão, carne e queijo!".to_string(),
            Categoria::Lanche,
            9.99,
            Ingredientes::new(
                vec!["Pão".to_string(), "Hambúrguer".to_string(), "Queijo".to_string()]
            ).unwrap(),
            "2024-01-16".to_string(),
            "2024-01-16".to_string(),
        );
        produto.set_nome("Salada Burger".to_string());
        produto.set_foto("salada_burguer.png".to_string());
        produto.set_descricao("Delicioso hambúrguer com salada fresca!".to_string());
        produto.set_preco(10.99);
        produto.set_data_criacao("2024-02-17".to_string());
        produto.set_data_atualizacao("2024-02-18".to_string());
        assert_eq!(produto.nome(), "Salada Burger");
        assert_eq!(produto.foto(), "salada_burguer.png");
        assert_eq!(produto.descricao(), "Delicioso hambúrguer com salada fresca!");
        assert_eq!(produto.preco(), 10.99);
        assert_eq!(produto.data_criacao(), "2024-02-17");
        assert_eq!(produto.data_atualizacao(), "2024-02-18");
    }
    
    #[test]
    fn test_produto_set_categoria_valid() {
        let mut produto = Produto::new(
            1,
            "Cheeseburger".to_string(),
            "cheeseburger.png".to_string(),
            "O clássico pão, carne e queijo!".to_string(),
            Categoria::Lanche,
            9.99,
            Ingredientes::new(
                vec!["Pão".to_string(), "Hambúrguer".to_string(), "Queijo".to_string()]
            ).unwrap(),
            "2024-01-16".to_string(),
            "2024-01-16".to_string(),
        );
        produto.set_categoria(Categoria::Bebida);
        assert_eq!(produto.categoria(), &Categoria::Bebida);
    }

    #[test]
    #[should_panic(expected = "Nome não pode ser vazio")]
    fn test_produto_set_nome_empty() {
        let mut produto = Produto::new(
            1,
            "Cheeseburger".to_string(),
            "cheeseburger.png".to_string(),
            "O clássico pão, carne e queijo!".to_string(),
            Categoria::Lanche,
            9.99,
            Ingredientes::new(
                vec!["Pão".to_string(), "Hambúrguer".to_string(), "Queijo".to_string()]
            ).unwrap(),
            "2024-01-16".to_string(),
            "2024-01-16".to_string(),
        );
        produto.set_nome("".to_string());
    }

    #[test]
    #[should_panic(expected = "Preço não pode ser negativo")]
    fn test_produto_set_preco_negative() {
        let mut produto = Produto::new(
            1,
            "Cheeseburger".to_string(),
            "cheeseburger.png".to_string(),
            "O clássico pão, carne e queijo!".to_string(),
            Categoria::Lanche,
            9.99,
            Ingredientes::new(
                vec!["Pão".to_string(), "Hambúrguer".to_string(), "Queijo".to_string()]
            ).unwrap(),
            "2024-01-16".to_string(),
            "2024-01-16".to_string(),
        );
        produto.set_preco(-1.0);
    }

    #[test]
    #[should_panic(expected = "Data de criação não está no formato correto (YYYY-MM-DD)")]
    fn test_produto_set_data_criacao_invalid_format() {
        let mut produto = Produto::new(
            1,
            "Cheeseburger".to_string(),
            "cheeseburger.png".to_string(),
            "O clássico pão, carne e queijo!".to_string(),
            Categoria::Lanche,
            9.99,
            Ingredientes::new(
                vec!["Pão".to_string(), "Hambúrguer".to_string(), "Queijo".to_string()]
            ).unwrap(),
            "2024-01-16".to_string(),
            "2024-01-16".to_string(),
        );
        produto.set_data_criacao("17-01-2024".to_string());
    }
    #[test]
    #[should_panic(expected = "Data de atualização não está no formato correto (YYYY-MM-DD)")]
    fn test_produto_set_data_atualizacao_invalid_format() {
        let mut produto = Produto::new(
            1,
            "Cheeseburger".to_string(),
            "cheeseburger.png".to_string(),
            "O clássico pão, carne e queijo!".to_string(),
            Categoria::Lanche,
            9.99,
            Ingredientes::new(
                vec!["Pão".to_string(), "Hambúrguer".to_string(), "Queijo".to_string()]
            ).unwrap(),
            "2024-01-16".to_string(),
            "2024-01-16".to_string(),
        );
        produto.set_data_atualizacao("18-02-2024".to_string());
    }
}