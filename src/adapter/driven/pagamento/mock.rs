use crate::core::domain::base::domain_error::DomainError;
use crate::core::application::ports::pagamento_port::{ StatusPagamento, PagamentoPort, ResultadoHandler };

pub struct MockPagamentoSuccesso;

#[async_trait]
impl PagamentoPort for MockPagamentoSuccesso {
    async fn processa_pagamento(
        &self,
        pedido_id: usize, 
        valor_pagamento: f32,
        resultado_handler: ResultadoHandler
    ) -> Result<usize, DomainError> {
        resultado_handler(StatusPagamento::Successo, pedido_id);
        Ok(pedido_id)
    }

    fn pagamento_status(
        &self,
        pagamento_id: usize
    ) -> Result<StatusPagamento, DomainError> {
        Ok(StatusPagamento::Successo)
    }
}
