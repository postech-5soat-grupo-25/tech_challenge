use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::core::domain::base::aggregate_root::AggregateRoot;
use crate::core::domain::value_objects::{ cpf, endereco };
use crate::core::domain::base::assertion_concern;

#[derive(Clone, Serialize, Deserialize, Debug, JsonSchema)]
pub struct Usuario {
  id: usize,
  nome: String,
  email: String,
  #[serde(skip_serializing)]
  senha: String,
  cpf: cpf::Cpf,
  endereco: endereco::Endereco,
}

impl AggregateRoot for Usuario {}

impl Usuario {
  pub fn new(id: usize, nome: String, email: String, senha: String, cpf: cpf::Cpf, endereco: endereco::Endereco) -> Self {
    Usuario {id, nome, email, senha, cpf, endereco }
  }

  fn validate_entity(&self) {
    assertion_concern::assert_argument_not_empty(self.nome.clone(), "Nome não pode ser vazio".to_string());
    assertion_concern::assert_argument_not_empty(self.email.clone(), "Email não pode ser vazio".to_string());
  }

  pub fn id(&self) -> &usize {
    &self.id
  }

  pub fn nome(&self) -> &String {
    &self.nome
  }

  pub fn set_nome(&mut self, nome: String) {
    self.nome = nome;
  }

  pub fn email(&self) -> &String {
    &self.email
  }

  pub fn set_email(&mut self, email: String) {
    self.email = email;
  }

  pub fn senha(&self) -> &String {
    &self.senha
  }

  pub fn validate_senha(&self, senha: &String) -> bool {
    &self.senha == senha
  }

  pub fn cpf(&self) -> &cpf::Cpf {
    &self.cpf
  }

  pub fn endereco(&self) -> &endereco::Endereco {
    &self.endereco
  }

  pub fn set_endereco(&mut self, endereco: endereco::Endereco) {
    self.endereco = endereco;
  }
}