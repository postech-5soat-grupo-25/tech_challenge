use serde::{Deserialize, Serialize, Serializer};
use schemars::JsonSchema;
#[derive(Clone, Deserialize, JsonSchema)]
pub struct Cpf {
    pub numero: String,
}

impl Serialize for Cpf {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.numero)
    }
}