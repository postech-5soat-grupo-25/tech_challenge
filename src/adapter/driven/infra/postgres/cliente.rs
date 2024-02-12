use chrono::{DateTime, Utc};

use postgres_from_row::FromRow;
use std::collections::HashMap;

use crate::core::domain::entities::cliente::Cliente;
use crate::core::domain::value_objects::cpf::Cpf;

use super::table::{ColumnDefault, ColumnNullable, ColumnTypes};

pub fn get_cliente_table_columns() -> HashMap<String, (ColumnTypes, ColumnNullable, ColumnDefault)>
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
        "data_criacao".to_string(),
        (
            ColumnTypes::Timestamp,
            ColumnNullable(true),
            ColumnDefault(Some("CURRENT_TIMESTAMP".to_string())),
        ),
    );
    columns.insert(
        "data_atualizacao".to_string(),
        (
            ColumnTypes::Timestamp,
            ColumnNullable(true),
            ColumnDefault(Some("CURRENT_TIMESTAMP".to_string())),
        ),
    );

    columns
}

impl FromRow for Cliente {
    fn from_row(row: &tokio_postgres::Row) -> Self {
        let id: i32 = row.get("id");
        let data_criacao: std::time::SystemTime = row.get("data_criacao");
        let data_criacao: DateTime<Utc> = data_criacao.into();
        let data_atualizacao: std::time::SystemTime = row.get("data_atualizacao");
        let data_atualizacao: DateTime<Utc> = data_atualizacao.into();
        Cliente::new(
            id as usize,
            row.get("nome"),
            row.get("email"),
            Cpf::new(row.get("cpf")).unwrap(),
            data_criacao.format("%Y-%m-%d %H:%M:%S%.3f%z").to_string(),
            data_atualizacao
                .format("%Y-%m-%d %H:%M:%S%.3f%z")
                .to_string(),
        )
    }

    fn try_from_row(row: &tokio_postgres::Row) -> Result<Self, tokio_postgres::Error> {
        let id: i32 = row.try_get("id")?;
        let data_criacao: std::time::SystemTime = row.try_get("data_criacao")?;
        let data_criacao: DateTime<Utc> = data_criacao.into();
        let data_atualizacao: std::time::SystemTime = row.try_get("data_atualizacao")?;
        let data_atualizacao: DateTime<Utc> = data_atualizacao.into();
        Ok(Cliente::new(
            id as usize,
            row.try_get("nome")?,
            row.try_get("email")?,
            Cpf::new(row.try_get("cpf")?).unwrap(),
            data_criacao.format("%Y-%m-%d %H:%M:%S%.3f%z").to_string(),
            data_atualizacao
                .format("%Y-%m-%d %H:%M:%S%.3f%z")
                .to_string(),
        ))
    }
}
