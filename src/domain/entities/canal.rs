// use crate::domain::base::domain_exception::{DomainError, Result};
use crate::domain::base::aggregate_root::AggregateRoot;

#[derive(Clone)]
pub struct Canal {
  nome: String,
}

impl AggregateRoot for Canal {}

impl Canal {
  pub fn new(nome: String) -> Self {
    Canal { nome }
  }

  pub fn nome(&self) -> &String {
    &self.nome
  }
}

