#[derive(Debug)]
pub enum DomainError {
    AlreadyExists,
    Empty,
    NotFound,
    Invalid(String),
    NonPositive
}