use schemars::JsonSchema;
use serde::{Serialize, Deserialize};

use crate::core::domain::base::aggregate_root::AggregateRoot;
use crate::core::domain::value_objects::cpf::Cpf;
use crate::core::domain::base::assertion_concern;

#[derive(Clone, Serialize, Deserialize, Debug, JsonSchema)]
pub struct Cliente {
    id: usize,
    nome: String,
    email: String,
    cpf: Cpf,
    data_criacao: String,
    data_atualizacao: String,
}

impl AggregateRoot for Cliente {}

impl Cliente {
    pub fn new(
        id: usize,
        nome: String,
        email: String,
        cpf: Cpf,
        data_criacao: String,
        data_atualizacao: String,
    ) -> Self {
        Cliente {
            id,
            nome,
            email,
            cpf,
            data_criacao,
            data_atualizacao,
        }
    }

    // Getters
    pub fn id(&self) -> &usize {
        &self.id
    }

    pub fn nome(&self) -> &String {
        &self.nome
    }

    pub fn email(&self) -> &String {
        &self.email
    }

    pub fn cpf(&self) -> &Cpf {
        &self.cpf
    }

    pub fn data_criacao(&self) -> &String {
        &self.data_criacao
    }

    pub fn data_atualizacao(&self) -> &String {
        &self.data_atualizacao
    }

    // Setters
    pub fn set_nome(&mut self, nome: String) {
        assertion_concern::assert_argument_not_empty(nome.clone(), "Nome não pode ser vazio".to_string());
        self.nome = nome;
    }

    pub fn set_email(&mut self, email: String) {
        assertion_concern::assert_argument_not_empty(email.clone(), "Email não pode ser vazio".to_string());
        self.email = email;
    }

    pub fn set_cpf(&mut self, cpf: Cpf) {
        self.cpf = cpf;
    }

    pub fn set_data_criacao(&mut self, data_criacao: String) {
        assertion_concern::assert_argument_date_format(data_criacao.clone(), "Data de criação não está no formato correto (YYYY-MM-DD)".to_string());
        self.data_criacao = data_criacao;
    }

    pub fn set_data_atualizacao(&mut self, data_atualizacao: String) {
        assertion_concern::assert_argument_date_format(data_atualizacao.clone(), "Data de atualização não está no formato correto (YYYY-MM-DD)".to_string());
        self.data_atualizacao = data_atualizacao;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::domain::value_objects::cpf::Cpf;

    #[test]
    fn test_cliente_creation_valid() {
        let cpf = Cpf::new("123.456.789-09".to_string()).unwrap();
        let cliente = Cliente::new(
            1,
            "Fulano da Silva".to_string(),
            "fulano.silva@exemplo.com".to_string(),
            cpf,
            "2024-01-17".to_string(),
            "2024-01-17".to_string(),
        );

        assert_eq!(cliente.id(), &1);
        assert_eq!(cliente.nome(), "Fulano da Silva");
        assert_eq!(cliente.email(), "fulano.silva@exemplo.com");
        assert_eq!(cliente.data_criacao(), "2024-01-17");
        assert_eq!(cliente.data_atualizacao(), "2024-01-17");
    }

    #[test]
    fn test_cliente_setters_valid() {
        let mut cliente = Cliente::new(
            1,
            "Fulano da Silva".to_string(),
            "fulano.silva@exemplo.com".to_string(),
            Cpf::new("123.456.789-09".to_string()).unwrap(),
            "2024-01-17".to_string(),
            "2024-01-17".to_string(),
        );

        cliente.set_nome("Ciclano da Silva".to_string());
        cliente.set_email("ciclano.silva@exemplo.com".to_string());
        cliente.set_data_criacao("2024-02-17".to_string());
        cliente.set_data_atualizacao("2024-02-18".to_string());

        assert_eq!(cliente.nome(), "Ciclano da Silva");
        assert_eq!(cliente.email(), "ciclano.silva@exemplo.com");
        assert_eq!(cliente.data_criacao(), "2024-02-17");
        assert_eq!(cliente.data_atualizacao(), "2024-02-18");
    }

    #[test]
    #[should_panic(expected = "Nome não pode ser vazio")]
    fn test_cliente_set_nome_empty() {
        let mut cliente = Cliente::new(
            1,
            "Fulano da Silva".to_string(),
            "fulano.silva@exemplo.com".to_string(),
            Cpf::new("123.456.789-09".to_string()).unwrap(),
            "2024-01-17".to_string(),
            "2024-01-17".to_string(),
        );

        cliente.set_nome("".to_string());
    }

    #[test]
    #[should_panic(expected = "Email não pode ser vazio")]
    fn test_cliente_set_email_empty() {
        let mut cliente = Cliente::new(
            1,
            "Fulano da Silva".to_string(),
            "fulano.silva@exemplo.com".to_string(),
            Cpf::new("123.456.789-09".to_string()).unwrap(),
            "2024-01-17".to_string(),
            "2024-01-17".to_string(),
        );

        cliente.set_email("".to_string());
    }

    #[test]
    #[should_panic(expected = "Data de criação não está no formato correto (YYYY-MM-DD)")]
    fn test_cliente_set_data_criacao_invalid_format() {
        let mut cliente = Cliente::new(
            1,
            "Fulano da Silva".to_string(),
            "fulano.silva@exemplo.com".to_string(),
            Cpf::new("123.456.789-09".to_string()).unwrap(),
            "2024-01-17".to_string(),
            "2024-01-17".to_string(),
        );

        cliente.set_data_criacao("17-01-2024".to_string());
    }

    #[test]
    #[should_panic(expected = "Data de atualização não está no formato correto (YYYY-MM-DD)")]
    fn test_cliente_set_data_atualizacao_invalid_format() {
        let mut cliente = Cliente::new(
            1,
            "Fulano da Silva".to_string(),
            "fulano.silva@exemplo.com".to_string(),
            Cpf::new("123.456.789-09".to_string()).unwrap(),
            "2024-01-17".to_string(),
            "2024-01-17".to_string(),
        );

        cliente.set_data_atualizacao("18-02-2024".to_string());
    }
}