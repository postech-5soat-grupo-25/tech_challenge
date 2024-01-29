use crate::core::domain::base::domain_error::DomainError;

pub type ResultadoHandler = Box<dyn Fn(StatusPagamento, usize) + Send>;

#[derive(PartialEq)]
pub enum StatusPagamento {
    Successo,
    Falha,
}

pub trait PagamentoNotificationHandler {
    fn handle_pagamento_notification(&self, id: usize, status: StatusPagamento);
}
#[async_trait]
pub trait PagamentoPort {
    async fn processa_pagamento(
        &self,
        pedido_id: usize,
        valor_pagamento: f32,
        resultado_handler: ResultadoHandler,
    ) -> Result<usize, DomainError>;

    fn pagamento_status(&self, pagamento_id: usize) -> Result<StatusPagamento, DomainError>;
}
