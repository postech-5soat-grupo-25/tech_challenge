use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::core::domain::base::domain_error::DomainError;

#[derive(Clone, Serialize, Deserialize, Debug, JsonSchema)]
pub struct Ingredientes(Vec<String>);

impl Ingredientes {
    pub fn new(ingredientes: Vec<String>) -> Result<Self, DomainError> {
        return Ok(Ingredientes(ingredientes));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ingredientes_valid() {
        let ingredientes = Ingredientes::new(vec![
            "Pão".to_string(),
            "Hambúrguer".to_string(),
            "Queijo".to_string(),
        ]);
        assert!(ingredientes.is_ok());
    }

    #[test]
    fn test_ingredientes_empty() {
        let ingredientes = Ingredientes::new(vec![]);
        assert!(ingredientes.is_err());
    }

    #[test]
    fn test_ingredientes_single_item() {
        let ingredientes = Ingredientes::new(vec!["Queijo".to_string()]);
        assert!(ingredientes.is_ok());
    }
}
