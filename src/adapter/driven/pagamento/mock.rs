use crate::core::domain::base::domain_error::DomainError;
use crate::core::application::ports::pagamento_port::{ StatusPagamento, PagamentoPort, ResultadoHandler };

pub struct MockPagamentoSuccesso;

impl PagamentoPort for MockPagamentoSuccesso {
    fn  processa_pagamento(
        pedido_id: usize,
        valor_pagamento: f32,
        resultado_handler: ResultadoHandler
    ) -> Result<usize, DomainError> {
        resultado_handler(StatusPagamento::Successo, pedido_id);
        Ok(pedido_id)
    }

    fn pagamento_status(
        pagamento_id: usize
    ) -> Result<StatusPagamento, DomainError> {
        Ok(StatusPagamento::Successo)
    }
}