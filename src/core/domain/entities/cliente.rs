use schemars::JsonSchema;
use serde::{Serialize, Deserialize};

use crate::core::domain::base::domain_error::DomainError;
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

    pub fn validate_entity(&self) -> Result<(), DomainError> {
        assertion_concern::assert_argument_not_empty(self.nome.clone())?;
        assertion_concern::assert_argument_not_empty(self.email.clone())?;
        assertion_concern::assert_argument_date_format(self.data_criacao.clone())?;
        assertion_concern::assert_argument_date_format(self.data_atualizacao.clone())?;
        Ok(())
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
    pub fn set_nome(&mut self, nome: String) -> Result<(), DomainError> {
        assertion_concern::assert_argument_not_empty(nome.clone())?;
        self.nome = nome;
        Ok(())
    }

    pub fn set_email(&mut self, email: String) -> Result<(), DomainError> {
        assertion_concern::assert_argument_not_empty(email.clone())?;
        self.email = email;
        Ok(())
    }

    pub fn set_cpf(&mut self, cpf: Cpf) {
        self.cpf = cpf;
    }

    pub fn set_data_criacao(&mut self, data_criacao: String) -> Result<(), DomainError> {
        assertion_concern::assert_argument_date_format(data_criacao.clone())?;
        self.data_criacao = data_criacao;
        Ok(())
    }

    pub fn set_data_atualizacao(&mut self, data_atualizacao: String) -> Result<(), DomainError> {
        assertion_concern::assert_argument_date_format(data_atualizacao.clone())?;
        self.data_atualizacao = data_atualizacao;
        Ok(())
    }
}

// Unit Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::domain::value_objects::cpf::Cpf;

    fn create_valid_cliente() -> Cliente {
        Cliente::new(
            1,
            "Fulano da Silva".to_string(),
            "fulano.silva@exemplo.com".to_string(),
            Cpf::new("123.456.789-09".to_string()).unwrap(),
            "2024-01-17".to_string(),
            "2024-01-17".to_string(),
        )
    }

    #[test]
    fn test_cliente_creation_valid() {
        let cliente = create_valid_cliente();
        assert_eq!(cliente.id(), &1);
        assert_eq!(cliente.nome(), "Fulano da Silva");
        assert_eq!(cliente.email(), "fulano.silva@exemplo.com");
        assert_eq!(cliente.data_criacao(), "2024-01-17");
        assert_eq!(cliente.data_atualizacao(), "2024-01-17");
    }

    #[test]
    fn test_cliente_validate_entity_valid() {
        let cliente = create_valid_cliente();
        assert!(cliente.validate_entity().is_ok());
    }

    #[test]
    fn test_cliente_validate_entity_empty_nome() {
        let cliente = Cliente::new(
            1,
            "".to_string(),
            "fulano.silva@exemplo.com".to_string(),
            Cpf::new("123.456.789-09".to_string()).unwrap(),
            "2024-01-17".to_string(),
            "2024-01-17".to_string(),
        );

        let result = cliente.validate_entity();
        assert!(matches!(result, Err(DomainError::Empty)), "Esperado Err(DomainError::Empty), obtido {:?}", result);
    }

    #[test]
    fn test_cliente_validate_entity_empty_email() {
        let cliente = Cliente::new(
            1,
            "Fulano da Silva".to_string(),
            "".to_string(),
            Cpf::new("123.456.789-09".to_string()).unwrap(),
            "2024-01-17".to_string(),
            "2024-01-17".to_string(),
        );

        let result = cliente.validate_entity();
        assert!(matches!(result, Err(DomainError::Empty)), "Esperado Err(DomainError::Empty), obtido {:?}", result);
    }

    #[test]
    fn test_cliente_validate_entity_invalid_data_criacao() {
        let cliente = Cliente::new(
            1,
            "Fulano da Silva".to_string(),
            "fulano.silva@exemplo.com".to_string(),
            Cpf::new("123.456.789-09".to_string()).unwrap(),
            "17-01-2024".to_string(),
            "2024-01-17".to_string(),
        );

        let result = cliente.validate_entity();
        assert!(matches!(result, Err(DomainError::Invalid(_))), "Esperado Err(DomainError::Invalid), obtido {:?}", result);
    }

    #[test]
    fn test_cliente_validate_entity_invalid_data_atualizacao() {
        let cliente = Cliente::new(
            1,
            "Fulano da Silva".to_string(),
            "fulano.silva@exemplo.com".to_string(),
            Cpf::new("123.456.789-09".to_string()).unwrap(),
            "2024-01-17".to_string(),
            "18-02-2024".to_string(),
        );

        let result = cliente.validate_entity();
        assert!(matches!(result, Err(DomainError::Invalid(_))), "Esperado Err(DomainError::Invalid), obtido {:?}", result);
    }

    #[test]
    fn test_cliente_setters_valid() {
        let mut cliente = create_valid_cliente();
        let _ = cliente.set_nome("Ciclano da Silva".to_string());
        let _ = cliente.set_email("ciclano.silva@exemplo.com".to_string());
        let _ = cliente.set_data_criacao("2024-02-17".to_string());
        let _ = cliente.set_data_atualizacao("2024-02-18".to_string());
        assert_eq!(cliente.nome(), "Ciclano da Silva");
        assert_eq!(cliente.email(), "ciclano.silva@exemplo.com");
        assert_eq!(cliente.data_criacao(), "2024-02-17");
        assert_eq!(cliente.data_atualizacao(), "2024-02-18");
    }

    #[test]
    fn test_cliente_set_nome_empty() {
        let mut cliente = create_valid_cliente();
        let result = cliente.set_nome("".to_string());
        assert!(matches!(result, Err(DomainError::Empty)), "Esperado Err(DomainError::Empty), obtido {:?}", result);
    }

    #[test]
    fn test_cliente_set_email_empty() {
        let mut cliente = create_valid_cliente();
        let result = cliente.set_email("".to_string());
        assert!(matches!(result, Err(DomainError::Empty)), "Esperado Err(DomainError::Empty), obtido {:?}", result);
    }

    #[test]
    fn test_cliente_set_data_criacao_invalid_format() {
        let mut cliente = create_valid_cliente();
        let result = cliente.set_data_criacao("17-01-2024".to_string());
        assert!(matches!(result, Err(DomainError::Invalid(_))), "Esperado Err(DomainError::Invalid), obtido {:?}", result);
    }

    #[test]
    fn test_cliente_set_data_atualizacao_invalid_format() {
        let mut cliente = create_valid_cliente();
        let result = cliente.set_data_atualizacao("18-02-2024".to_string());
        assert!(matches!(result, Err(DomainError::Invalid(_))), "Esperado Err(DomainError::Invalid), obtido {:?}", result);
    }
}