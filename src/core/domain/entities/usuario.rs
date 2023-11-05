use crate::core::domain::base::aggregate_root::AggregateRoot;
use crate::core::domain::value_objects::{ cpf, endereco };
use crate::core::domain::base::assertion_concern;

#[derive(Clone)]
pub struct Usuario {
  nome: String,
  email: String,
  senha: String,
  cpf: cpf::Cpf,
  endereco: endereco::Endereco,
}

impl AggregateRoot for Usuario {}

impl Usuario {
  pub fn new(nome: String, email: String, senha: String, cpf: cpf::Cpf, endereco: endereco::Endereco) -> Self {
    Usuario { nome, email, senha, cpf, endereco }
  }

  fn validate_entity(&self) {
    assertion_concern::assert_argument_not_empty(self.nome.clone(), "Nome não pode ser vazio".to_string());
    assertion_concern::assert_argument_not_empty(self.email.clone(), "Email não pode ser vazio".to_string());
  }

  pub fn nome(&self) -> &String {
    &self.nome
  }

  pub fn email(&self) -> &String {
    &self.email
  }

  pub fn senha(&self) -> &String {
    &self.senha
  }

  pub fn cpf(&self) -> &cpf::Cpf {
    &self.cpf
  }

  pub fn endereco(&self) -> &endereco::Endereco {
    &self.endereco
  }
}