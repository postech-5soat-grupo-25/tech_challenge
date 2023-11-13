use serde::{Deserialize, Serialize};
#[derive(Clone, Deserialize, Serialize)]
pub struct Endereco {
  pub cep: String,
}