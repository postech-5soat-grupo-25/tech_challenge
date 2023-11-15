#[derive(Debug)]
pub enum DomainError {
    AlreadyExists,
    Empty,
    NotFound,
}