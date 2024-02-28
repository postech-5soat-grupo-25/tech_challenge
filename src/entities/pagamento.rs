use chrono::Utc;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    base::{
        assertion_concern,
        domain_error::DomainError,
    },
};

#[derive(Clone, Serialize, Deserialize, Debug, JsonSchema)]
pub struct Pagamento {
    id: usize,
    id_pedido: usize,
    estado: String,
    metodo: String,
    referencia_pagamento: String,
    data_criacao: String,
}

impl Pagamento {
    pub fn new(
        id: usize,
        id_pedido: usize,
        estado: String,
        metodo: String,
        referencia_pagamento: String,
        data_criacao: String,
    ) -> Self {
        Pagamento {
            id,
            id_pedido,
            estado,
            metodo,
            referencia_pagamento,
            data_criacao,
        }
    }

    pub fn validate_entity(&self) -> Result<(), DomainError> {
        assertion_concern::assert_argument_not_empty(self.metodo.clone())?;
        assertion_concern::assert_argument_timestamp_format(self.data_criacao.clone())?;
        Ok(())
    }

    // Getters
    pub fn id(&self) -> &usize {
        &self.id
    }

    pub fn id_pedido(&self) -> &usize {
        &self.id_pedido
    }

    pub fn estado(&self) -> &String {
        &self.estado
    }

    pub fn metodo(&self) -> &String {
        &self.metodo
    }

    pub fn referencia_pagamento(&self) -> &String {
        &self.referencia_pagamento
    }


    pub fn data_criacao(&self) -> &String {
        &self.data_criacao
    }

    // Setters
    pub fn set_estado(&mut self, estado: String) -> Result<(), DomainError> {
        assertion_concern::assert_argument_not_empty(estado.clone())?;
        self.estado = estado;
        Ok(())
    }
    
    pub fn set_metodo(&mut self, metodo: String) -> Result<(), DomainError> {
        assertion_concern::assert_argument_not_empty(metodo.clone())?;
        self.metodo = metodo;
        Ok(())
    }

    pub fn set_referencia_pagamento(&mut self, referencia_pagamento: String) {
        self.referencia_pagamento = referencia_pagamento;
    }

}

// Unit Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn create_valid_pagamento() -> Pagamento {
        let _now = Utc::now().format("%Y-%m-%d %H:%M:%S%.3f%z").to_string();
        Pagamento::new(
            1,
            1,
            "MercadoPago".to_string(),
            "pendente".to_string(),
            "aaabbbccc".to_string(),
            _now.clone(),
        )
    }

    #[test]
    fn test_pagamento_creation_valid() {
        let pagamento = create_valid_pagamento();
        assert_eq!(pagamento.id(), &1);
        assert_eq!(pagamento.id_pedido(), &1);
        assert_eq!(pagamento.estado(), "pendente");
        assert_eq!(pagamento.metodo(), "MercadoPago");
        assert_eq!(pagamento.referencia_pagamento(), "aaabbbccc");

    }

    #[test]
    fn test_pagamento_validate_entity_valid() {
        let pagamento = create_valid_pagamento();
        assert!(pagamento.validate_entity().is_ok());
    }

    #[test]
    fn test_pagamento_validate_entity_empty_metodo() {
        let _now = Utc::now().format("%Y-%m-%d %H:%M:%S%.3f%z").to_string();
        let pagamento = Pagamento::new(
            1,
            1,
            "pendente".to_string(),
            "".to_string(),
            "aaabbbccc".to_string(),
            _now.clone(),
        );
        let result = pagamento.validate_entity();
        assert!(
            matches!(result, Err(DomainError::Empty)),
            "Esperado Err(DomainError::Empty), obtido {:?}",
            result
        );
    }

    
    #[test]
    fn test_pagamento_setters_valid() {
        let mut pagamento = create_valid_pagamento();
        let _ = pagamento.set_estado("aprovado".to_string());
        let _ = pagamento.set_metodo("PIX".to_string());
        let _ = pagamento.set_referencia_pagamento("dddeeefff".to_string());
        assert_eq!(pagamento.estado(), "aprovado");
        assert_eq!(pagamento.metodo(), "PIX");
        assert_eq!(pagamento.referencia_pagamento(), "dddeeefff");
    }

}