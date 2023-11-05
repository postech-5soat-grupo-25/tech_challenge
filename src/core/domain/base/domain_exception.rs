pub struct DomainError {
    message: String,
    error_stack: Option<Vec<String>>,
}

impl DomainError {
  pub fn new(message: String, error_stack: Option<Vec<String>>) -> Self {
      DomainError { message, error_stack }
  }
}

pub type Result<T> = std::result::Result<T, DomainError>;