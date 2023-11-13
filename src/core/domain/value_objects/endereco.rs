use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
#[derive(Clone, Deserialize, Serialize, JsonSchema)]
pub struct Endereco {
  pub cep: String,
}