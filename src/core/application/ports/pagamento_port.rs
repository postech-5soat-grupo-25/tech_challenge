use crate::core::domain::base::domain_error::DomainError;

pub type ResultadoHandler = fn(status: StatusPagamento, pedido_id: usize) -> ();

pub enum StatusPagamento {
    Successo,
    Falha,
}

pub trait PagamentoNotificationHandler {
    fn handle_pagamento_notification(&self, id: usize, status: StatusPagamento);
}

pub trait PagamentoPort {
    fn processa_pagamento(
        pedido_id: usize, 
        valor_pagamento: f32,
        resultado_handler: ResultadoHandler
    ) -> Result<usize, DomainError>;

    fn pagamento_status(
        pagamento_id: usize
    ) -> Result<StatusPagamento, DomainError>;
}