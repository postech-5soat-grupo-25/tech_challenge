use crate::core::domain::base::aggregate_root::AggregateRoot;

pub struct RedeSocial {
  nome: String,
  url: String,
  token: String,
}

impl AggregateRoot for RedeSocial {}

impl RedeSocial {
  pub fn new(nome: String, url: String, token: String) -> Self {
    RedeSocial { nome, url, token }
  }

  pub fn nome(&self) -> &String {
    &self.nome
  }

  pub fn url(&self) -> &String {
    &self.url
  }

  pub fn token(&self) -> &String {
    &self.token
  }
}