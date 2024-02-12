use mockall::*;

use schemars::JsonSchema;
use crate::core::domain::base::domain_error::DomainError;

#[derive(PartialEq, JsonSchema)]
pub enum StatusPagamento {
    Successo,
    Falha,
}

#[automock]
pub trait PagamentoPort: Send + Sync {
    fn processa_pagamento(
        &self,
        pedido_id: usize,
        valor_pagamento: f64,
    ) -> Result<StatusPagamento, DomainError>;

    fn pagamento_status(
        &self,
        pagamento_id: usize
    ) -> Result<StatusPagamento, DomainError>;
}
