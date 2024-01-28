use crate::core::domain::base::domain_error::DomainError;

pub enum StatusPagamento {
    Successo,
    Falha,
}

pub trait PagamentoNotificationHandler {
    fn handle_pagamento_notification(&self, id: usize, status: StatusPagamento);
}

pub trait PagamentoProcessor {
    fn process_pagamento(&self, id: usize) -> Result<(), DomainError>;
}