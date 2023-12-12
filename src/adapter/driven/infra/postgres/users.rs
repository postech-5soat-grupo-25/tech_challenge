use std::collections::HashMap;

use postgres_from_row::FromRow;

use crate::core::domain::{entities::usuario::Usuario, value_objects::{cpf::Cpf, endereco::{Endereco, self}}};

use super::table::{ColumnTypes, ColumnNullable, ColumnDefault};


pub fn get_users_table_columns() -> HashMap<String, (ColumnTypes, ColumnNullable, ColumnDefault)> {
  let mut columns = HashMap::new();
  columns.insert("id".to_string(), (ColumnTypes::Index, ColumnNullable(false), ColumnDefault(None)));
  columns.insert("nome".to_string(), (ColumnTypes::Text, ColumnNullable(false), ColumnDefault(None)));
  columns.insert("email".to_string(), (ColumnTypes::Text, ColumnNullable(false), ColumnDefault(None)));
  columns.insert("senha".to_string(), (ColumnTypes::Text, ColumnNullable(false), ColumnDefault(None)));
  columns.insert("cpf".to_string(), (ColumnTypes::Text, ColumnNullable(false), ColumnDefault(None)));
  columns.insert("endereco".to_string(), (ColumnTypes::JSON, ColumnNullable(false), ColumnDefault(None)));
  columns.insert("created_at".to_string(), (ColumnTypes::Timestamp, ColumnNullable(true), ColumnDefault(Some("CURRENT_TIMESTAMP".to_string()))));
  columns.insert("updated_at".to_string(), (ColumnTypes::Timestamp, ColumnNullable(true), ColumnDefault(Some("CURRENT_TIMESTAMP".to_string()))));

  columns
}

impl FromRow for Usuario {
  fn from_row(row: &tokio_postgres::Row) -> Self {
    let id: i32 = row.get("id");
    let endereco_json: tokio_postgres::types::Json<Endereco> = row.get("endereco");
    let endereco = endereco_json.0;

    Usuario::new(
      id as usize,
      row.get("nome"),
      row.get("email"),
      row.get("senha"),
      Cpf::new(row.get("cpf")).unwrap(),
      endereco,
    )
  }

  fn try_from_row(row: &tokio_postgres::Row) -> Result<Self, tokio_postgres::Error> {
    let id: i32 = row.try_get("id")?;
    Ok(Usuario::new(
      id as usize,
      row.try_get("nome")?,
      row.try_get("email")?,
      row.try_get("senha")?,
      Cpf::new(row.try_get("cpf")?).unwrap(),
      serde_json::from_str::<Endereco>(row.try_get("endereco")?).unwrap(),
    ))
  }
}