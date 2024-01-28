use std::thread;
use std::time::Duration;

use crate::core::application::ports::pagamento_port;
use crate::core::domain::base::domain_error::DomainError;

pub struct MockPagamentoProcessor<T: pagamento_port::PagamentoNotificationHandler> {
    notification_handler: T,
}

impl<T: pagamento_port::PagamentoNotificationHandler> MockPagamentoProcessor<T> {
    pub fn new(notification_handler: T) -> Self {
        MockPagamentoProcessor {
            notification_handler,
        }
    }
}

impl<T: pagamento_port::PagamentoNotificationHandler + Send + 'static + Clone>
    pagamento_port::PagamentoProcessor for MockPagamentoProcessor<T>
{
    fn process_pagamento(&self, id: usize) -> Result<(), DomainError> {
        let notification_handler = self.notification_handler.clone();
        // Simulando chamada assíncrona do webhook com uma nova thread
        thread::spawn(move || {
            // Simulando delay de rede
            thread::sleep(Duration::from_secs(2));
            // Simulando notificação do webhook
            notification_handler
                .handle_pagamento_notification(id, pagamento_port::StatusPagamento::Successo);
        });
        Ok(())
    }
}
