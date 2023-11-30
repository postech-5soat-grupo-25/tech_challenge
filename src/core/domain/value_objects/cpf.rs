use serde::{Deserialize, Serialize, Serializer};
use schemars::JsonSchema;
#[derive(Clone, Deserialize, Debug)]
pub struct Cpf {
    pub numero: String,
}

impl Cpf {
    pub fn new(numero: String) -> Self {
        // TODO: validar cpf
        Cpf { numero }
    }
}

impl Serialize for Cpf {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.numero)
    }
}

impl JsonSchema for Cpf {
    fn schema_name() -> String {
        "Cpf".to_owned()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        String::json_schema(gen)
    }
}