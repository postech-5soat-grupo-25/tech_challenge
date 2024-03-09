use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use postgres_from_row::FromRow;
use std::collections::HashMap;
use std::time::SystemTime;

use crate::entities::pagamento::Pagamento;

use super::table::{ColumnDefault, ColumnNullable, ColumnTypes};

pub fn get_pagamento_table_columns() -> HashMap<String, (ColumnTypes, ColumnNullable, ColumnDefault)>
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
        "id_pedido".to_string(),
        (
            ColumnTypes::Integer,
            ColumnNullable(true),
            ColumnDefault(None),
        ),
    );
    columns.insert(
        "estado".to_string(),
        (
            ColumnTypes::Text,
            ColumnNullable(false),
            ColumnDefault(None),
        ),
    );
    columns.insert(
        "metodo".to_string(),
        (
            ColumnTypes::Text,
            ColumnNullable(true),
            ColumnDefault(None),
        ),
    );
    columns.insert(
        "referencia".to_string(),
        (
            ColumnTypes::Text,
            ColumnNullable(true),
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
    columns
}

impl FromRow for Pagamento {
    fn from_row(row: &tokio_postgres::Row) -> Self {
        let id: i32 = row.get("id");
        let id_pedido: i32 = row.get("id_pedido");


        let data_criacao: std::time::SystemTime = row.get("data_criacao");
        let data_criacao: DateTime<Utc> = data_criacao.into();

        Pagamento::new(
            id as usize,
            id_pedido as usize,
            row.get("estado"),
            row.get("valor"),
            row.get("metodo"),
            row.get("referencia"),
            data_criacao.format("%Y-%m-%d %H:%M:%S%.3f%z").to_string(),
        )
    }

    fn try_from_row(row: &tokio_postgres::Row) -> Result<Self, tokio_postgres::Error> {
        let id: i32 = row.try_get("id")?;
        let id_pedido: i32 = row.try_get("id_pedido")?;


        let data_criacao: std::time::SystemTime = row.try_get("data_criacao")?;
        let data_criacao: DateTime<Utc> = data_criacao.into();

        Ok(Pagamento::new(
            id as usize,
            id_pedido as usize,
            row.try_get("estado")?,
            row.try_get("valor")?,
            row.try_get("metodo")?,
            row.try_get("referencia")?,
            data_criacao.format("%Y-%m-%d %H:%M:%S%.3f%z").to_string(),
        ))
    }
}
