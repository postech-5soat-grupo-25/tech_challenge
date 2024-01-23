use crate::core::application::interfaces::payment_processor;
// ::{PaymentStatus, PaymentNotificationHandler, PaymentProcessor};
use crate::core::domain::base::domain_error::DomainError;
use std::thread;
use std::time::Duration;

pub struct MockPaymentProcessor<T: payment_processor::PaymentNotificationHandler> {
    notification_handler: T,
}

impl<T: payment_processor::PaymentNotificationHandler> MockPaymentProcessor<T> {
    pub fn new(notification_handler: T) -> Self {
        MockPaymentProcessor { notification_handler }
    }
}

impl<T: payment_processor::PaymentNotificationHandler + Send + 'static + Clone> payment_processor::PaymentProcessor for MockPaymentProcessor<T> {
    fn process_payment(&self, id: usize) -> Result<(), DomainError> {
        let notification_handler = self.notification_handler.clone();
        // Simulate asynchronous webhook call with a new thread
        thread::spawn(move || {
            // Simulate network delay
            thread::sleep(Duration::from_secs(2));
            // Simulate webhook notification
            notification_handler.handle_payment_notification(id, payment_processor::PaymentStatus::Success);
        });

        Ok(())
    }
}