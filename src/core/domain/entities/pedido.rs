use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::core::domain::base::aggregate_root::AggregateRoot;
use crate::core::domain::entities::cliente::Cliente;
use crate::core::domain::entities::produto::Produto;
use crate::core::domain::base::assertion_concern;

#[derive(Clone, Serialize, Deserialize, Debug, JsonSchema)]
pub enum Status {
    Recebido,
    EmPreparacao,
    Pronto,
    Finalizado,
}

#[derive(Clone, Serialize, Deserialize, Debug, JsonSchema)]
pub struct Pedido {
    id: usize,
    cliente: Option<Cliente>,
    lanche: Option<Produto>,
    acompanhamento: Option<Produto>,
    bebida: Option<Produto>,
    pagamento: String,
    status: Status,
    data_criacao: String,
    data_atualizacao: String,
}

impl AggregateRoot for Pedido {}

impl Pedido {
    pub fn new(
        id: usize,
        cliente: Option<Cliente>,
        lanche: Option<Produto>,
        acompanhamento: Option<Produto>,
        bebida: Option<Produto>,
        pagamento: String,
        status: Status,
        data_criacao: String,
        data_atualizacao: String,
    ) -> Self { 
        Pedido {
            id,
            cliente,
            lanche,
            acompanhamento,
            bebida,
            pagamento,
            status,
            data_criacao,
            data_atualizacao,
        }
    }

    pub fn validate_entity(&self) -> Result<(), String> {
        if self.lanche.is_none() && self.acompanhamento.is_none() && self.bebida.is_none() {
            return Err("Pedido deve conter pelo menos um item entre Lanche, Acompanhamento ou Bebida".to_string());
        };
        match self.status {
            Status::Recebido | Status::EmPreparacao | Status::Pronto | Status::Finalizado => (),
            _ => return Err("Status do Pedido é inválido".to_string()),
        };
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

    pub fn cliente(&self) -> Option<&Cliente> {
        self.cliente.as_ref()
    }

    pub fn lanche(&self) -> Option<&Produto> {
        self.lanche.as_ref()
    }

    pub fn acompanhamento(&self) -> Option<&Produto> {
        self.acompanhamento.as_ref()
    }

    pub fn bebida(&self) -> Option<&Produto> {
        self.bebida.as_ref()
    }

    pub fn pagamento(&self) -> &String {
        &self.pagamento
    }

    pub fn status(&self) -> &Status {
        &self.status
    }

    pub fn data_criacao(&self) -> &String {
        &self.data_criacao
    }

    pub fn data_atualizacao(&self) -> &String {
        &self.data_atualizacao
    }

    // Setters
    pub fn set_cliente(&mut self, cliente: Option<Cliente>) {
        self.cliente = cliente;
    }

    pub fn set_lanche(&mut self, lanche: Option<Produto>) {
        self.lanche = lanche;
    }

    pub fn set_acompanhamento(&mut self, acompanhamento: Option<Produto>) {
        self.acompanhamento = acompanhamento;
    }

    pub fn set_bebida(&mut self, bebida: Option<Produto>) {
        self.bebida = bebida;
    }

    pub fn set_pagamento(&mut self, pagamento: String) {
        self.pagamento = pagamento;
    }

    pub fn set_status(&mut self, status: Status) {
        self.status = status;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::domain::value_objects::cpf::Cpf;
    use crate::core::domain::value_objects::ingredientes::Ingredientes;
    use crate::core::domain::value_objects::ingredientes::Categoria;

    #[test]
    fn test_pedido_creation_valid() {
        let cliente = Cliente::new(
            1,
            "Fulano da Silva".to_string(),
            "fulano.silva@exemplo.com".to_string(),
            Cpf::new("123.456.789-09".to_string()).unwrap(),
            "2024-01-17".to_string(),
            "2024-01-17".to_string(),
        );

        let lanche = Produto::new(
            1,
            "Cheeseburger".to_string(),
            "cheeseburger.png".to_string(),
            "O clássico pão, carne e queijo!".to_string(),
            Categoria::Lanche,
            9.99,
            Ingredientes::new(vec!["Pão".to_string(), "Hambúrguer".to_string(), "Queijo".to_string()]).unwrap(),
            "2024-01-17".to_string(),
            "2024-01-17".to_string(),
        );

        let pedido = Pedido::new(
            1,
            Some(cliente),
            Some(lanche),
            None,
            None,
            "Cartão de Crédito".to_string(),
            Status::Recebido,
            "2024-01-17".to_string(),
            "2024-01-17".to_string(),
        );

        assert_eq!(pedido.id(), &1);
        assert!(pedido.cliente().is_some());
        assert!(pedido.lanche().is_some());
        assert!(pedido.acompanhamento().is_none());
        assert!(pedido.bebida().is_none());
        assert_eq!(pedido.pagamento(), "Cartão de Crédito");
        assert_eq!(pedido.status(), &Status::Recebido);
        assert_eq!(pedido.data_criacao(), "2024-01-17");
        assert_eq!(pedido.data_atualizacao(), "2024-01-17");
    }

    #[test]
    fn test_pedido_validate_entity_valid() {
        let cliente = Cliente::new(
            1,
            "Fulano da Silva".to_string(),
            "fulano.silva@exemplo.com".to_string(),
            Cpf::new("123.456.789-09".to_string()).unwrap(),
            "2024-01-17".to_string(),
            "2024-01-17".to_string(),
        );

        let lanche = Produto::new(
            1,
            "Cheeseburger".to_string(),
            "cheeseburger.png".to_string(),
            "O clássico pão, carne e queijo!".to_string(),
            Categoria::Lanche,
            9.99,
            Ingredientes::new(vec!["Pão".to_string(), "Hambúrguer".to_string(), "Queijo".to_string()]).unwrap(),
            "2024-01-17".to_string(),
            "2024-01-17".to_string(),
        );

        let pedido = Pedido::new(
            1,
            Some(cliente),
            Some(lanche),
            None,
            None,
            "Mercado Pago".to_string(),
            Status::Recebido,
            "2024-01-17".to_string(),
            "2024-01-17".to_string(),
        );

        assert!(pedido.validate_entity().is_ok());
    }

    #[test]
    #[should_panic(expected = "Pedido deve conter pelo menos um item entre Lanche, Acompanhamento ou Bebida")]
    fn test_pedido_validate_entity_no_items() {
        let cliente = Cliente::new(
            1,
            "Fulano da Silva".to_string(),
            "fulano.silva@exemplo.com".to_string(),
            Cpf::new("123.456.789-09".to_string()).unwrap(),
            "2024-01-17".to_string(),
            "2024-01-17".to_string(),
        );

        let pedido = Pedido::new(
            1,
            Some(cliente),
            None,
            None,
            None,
            "Mercado Pago".to_string(),
            Status::Recebido,
            "2024-01-17".to_string(),
            "2024-01-17".to_string(),
        );

        pedido.validate_entity().unwrap();
    }

    #[test]
    #[should_panic(expected = "Data de criação não está no formato correto (YYYY-MM-DD)")]
    fn test_pedido_set_data_criacao_invalid_format() {
        let pedido = Pedido::new(
            1,
            Some(cliente),
            Some(lanche),
            None,
            None,
            "Cartão de Crédito".to_string(),
            Status::Recebido,
            "2024-01-17".to_string(),
            "2024-01-17".to_string(),
        );

        pedido.set_data_criacao("17-01-2024".to_string());
    }

    #[test]
    #[should_panic(expected = "Data de atualização não está no formato correto (YYYY-MM-DD)")]
    fn test_pedido_set_data_atualizacao_invalid_format() {
        let pedido = Pedido::new(
            1,
            Some(cliente),
            Some(lanche),
            None,
            None,
            "Cartão de Crédito".to_string(),
            Status::Recebido,
            "2024-01-17".to_string(),
            "2024-01-17".to_string(),
        );

        pedido.set_data_atualizacao("18-02-2024".to_string());
    }
}