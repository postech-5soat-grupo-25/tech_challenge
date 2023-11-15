
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
#[derive(Clone, Deserialize, Serialize, JsonSchema)]
pub struct Endereco {
  pub cep: String,
}

impl Endereco {
  pub fn new(cep: String) -> Self {
    // TODO: validar cep
    Endereco { cep }
  }
}