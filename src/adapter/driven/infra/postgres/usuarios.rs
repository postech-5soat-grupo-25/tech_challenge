use postgres_from_row::FromRow;
use std::collections::HashMap;

use crate::core::domain::entities::usuario::{Status, Tipo, Usuario};
use crate::core::domain::value_objects::cpf::Cpf;

use super::table::{ColumnDefault, ColumnNullable, ColumnTypes};

pub fn get_usuarios_table_columns() -> HashMap<String, (ColumnTypes, ColumnNullable, ColumnDefault)>
{
    let mut columns = HashMap::new();
    columns.insert(
        "id".to_string(),
        (
            ColumnTypes::Index,
            ColumnNullable(false),
            ColumnDefault(None),
        ),
    );
    columns.insert(
        "nome".to_string(),
        (
            ColumnTypes::Text,
            ColumnNullable(false),
            ColumnDefault(None),
        ),
    );
    columns.insert(
        "email".to_string(),
        (
            ColumnTypes::Text,
            ColumnNullable(false),
            ColumnDefault(None),
        ),
    );
    columns.insert(
        "cpf".to_string(),
        (
            ColumnTypes::Text,
            ColumnNullable(false),
            ColumnDefault(None),
        ),
    );
    columns.insert(
        "senha".to_string(),
        (
            ColumnTypes::Text,
            ColumnNullable(false),
            ColumnDefault(None),
        ),
    );
    columns.insert(
        "tipo".to_string(),
        (
            ColumnTypes::Text,
            ColumnNullable(false),
            ColumnDefault(None),
        ),
    );
    columns.insert(
        "status".to_string(),
        (
            ColumnTypes::Text,
            ColumnNullable(false),
            ColumnDefault(None),
        ),
    );
    columns.insert(
        "data_criacao".to_string(),
        (
            ColumnTypes::Date,
            ColumnNullable(true),
            ColumnDefault(Some("CURRENT_DATE".to_string())),
        ),
    );
    columns.insert(
        "data_atualizacao".to_string(),
        (
            ColumnTypes::Date,
            ColumnNullable(true),
            ColumnDefault(Some("CURRENT_DATE".to_string())),
        ),
    );

    columns
}

impl FromRow for Usuario {
    fn from_row(row: &tokio_postgres::Row) -> Self {
        let id: i32 = row.get("id");

        Usuario::new(
            id as usize,
            row.get("nome"),
            row.get("email"),
            Cpf::new(row.get("cpf")).unwrap(),
            row.get("senha"),
            row.get::<_, &str>("tipo").parse::<Tipo>().unwrap(),
            row.get::<_, &str>("status").parse::<Status>().unwrap(),
            row.get("data_criacao"),
            row.get("data_atualizacao"),
        )
    }

    fn try_from_row(row: &tokio_postgres::Row) -> Result<Self, tokio_postgres::Error> {
        let id: i32 = row.try_get("id")?;
        Ok(Usuario::new(
            id as usize,
            row.try_get("nome")?,
            row.try_get("email")?,
            Cpf::new(row.try_get("cpf")?).unwrap(),
            row.try_get("senha")?,
            (row.try_get::<_, &str>("tipo")?).parse::<Tipo>().unwrap(),
            (row.try_get::<_, &str>("status")?).parse::<Status>().unwrap(),
            row.try_get("data_criacao")?,
            row.try_get("data_atualizacao")?,
        ))
    }
}
