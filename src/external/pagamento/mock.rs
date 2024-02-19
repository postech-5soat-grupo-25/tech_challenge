use crate::base::domain_error::DomainError;
use crate::traits::pagamento_port::{ StatusPagamento, PagamentoPort };

pub struct MockPagamentoSuccesso {}

impl PagamentoPort for MockPagamentoSuccesso {
    fn processa_pagamento(
        &self,
        pedido_id: usize,
        valor_pagamento: f64
    ) -> Result<StatusPagamento, DomainError> {
        Ok(StatusPagamento::Successo)
    }

    fn pagamento_status(
        &self,
        pagamento_id: usize
    ) -> Result<StatusPagamento, DomainError> {
        Ok(StatusPagamento::Successo)
    }
}