use std::thread;
use std::time::Duration;

use crate::core::application::interfaces::payment;
use crate::core::domain::base::domain_error::DomainError;

pub struct MockPaymentProcessor<T: payment::PaymentNotificationHandler> {
    notification_handler: T,
}

impl<T: payment::PaymentNotificationHandler> MockPaymentProcessor<T> {
    pub fn new(notification_handler: T) -> Self {
        MockPaymentProcessor { notification_handler }
    }
}

impl<T: payment::PaymentNotificationHandler + Send + 'static + Clone> payment::PaymentProcessor for MockPaymentProcessor<T> {
    fn process_payment(&self, id: usize) -> Result<(), DomainError> {
        let notification_handler = self.notification_handler.clone();
        // Simulando chamada assíncrona do webhook com uma nova thread
        thread::spawn(move || {
            // Simulando delay de rede
            thread::sleep(Duration::from_secs(2));
            // Simulando notificação do webhook
            notification_handler.handle_payment_notification(id, payment::PaymentStatus::Success);
        });
        Ok(())
    }
}