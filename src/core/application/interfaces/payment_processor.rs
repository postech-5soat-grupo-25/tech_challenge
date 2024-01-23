use crate::core::domain::base::domain_error::DomainError;

pub enum PaymentStatus {
    Success,
    Failure,
}

pub trait PaymentNotificationHandler {
    fn handle_payment_notification(&self, id: usize, status: PaymentStatus);
}

pub trait PaymentProcessor {
    fn process_payment(&self, id: usize) -> Result<(), DomainError>;
}