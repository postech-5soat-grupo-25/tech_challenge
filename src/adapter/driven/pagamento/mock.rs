use crate::core::domain::base::domain_error::DomainError;
use crate::core::application::ports::pagamento_port::{ StatusPagamento, PagamentoPort };

pub struct MockPagamentoSuccesso {}

impl PagamentoPort for MockPagamentoSuccesso {
    fn processa_pagamento(
        &self,
        pedido_id: usize,
        valor_pagamento: f32
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