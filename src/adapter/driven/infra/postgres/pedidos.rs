use postgres_from_row::FromRow;
use std::collections::HashMap;

use crate::core::domain::entities::pedido::Status;
use crate::core::domain::entities::pedido::PedidoFromRow;

use super::table::{ColumnDefault, ColumnNullable, ColumnTypes};

pub fn get_pedidos_table_columns() -> HashMap<String, (ColumnTypes, ColumnNullable, ColumnDefault)>
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
        "cliente".to_string(),
        (
            ColumnTypes::Integer,
            ColumnNullable(false),
            ColumnDefault(None),
        ),
    );
    columns.insert(
        "lanche".to_string(),
        (
            ColumnTypes::Integer,
            ColumnNullable(true),
            ColumnDefault(None),
        ),
    );
    columns.insert(
        "acompanhamento".to_string(),
        (
            ColumnTypes::Integer,
            ColumnNullable(true),
            ColumnDefault(None),
        ),
    );
    columns.insert(
        "bebida".to_string(),
        (
            ColumnTypes::Integer,
            ColumnNullable(true),
            ColumnDefault(None),
        ),
    );
    columns.insert(
        "pagamento".to_string(),
        (
            ColumnTypes::Text,
            ColumnNullable(true),
            ColumnDefault(None),
        ),
    );
    columns.insert(
        "status".to_string(),
        (
            ColumnTypes::Integer,
            ColumnNullable(false),
            ColumnDefault(None),
        ),
    );

    columns.insert(
        "data_criacao".to_string(),
        (
            ColumnTypes::Timestamp,
            ColumnNullable(true),
            ColumnDefault(Some("CURRENT_DATE".to_string())),
        ),
    );
    columns.insert(
        "data_atualizacao".to_string(),
        (
            ColumnTypes::Timestamp,
            ColumnNullable(true),
            ColumnDefault(Some("CURRENT_DATE".to_string())),
        ),
    );

    columns
}

impl FromRow for PedidoFromRow {
    fn from_row(row: &tokio_postgres::Row) -> Self {
        let id: i32 = row.get("id");
        let cliente_id: i32 = row.get("cliente");
        let lanche_id: i32 = row.get("lanche");
        let acompanhamento_id: i32 = row.get("acompanhamento");
        let bebida_id: i32 = row.get("bebida");
        let status: i32 = row.get("status");
        
        PedidoFromRow::new(
            id as usize,
            cliente_id,
            lanche_id,
            acompanhamento_id,
            bebida_id,
            row.get("pagamento"),
            Status::from_index(status as usize),
            row.get("data_criacao"),
            row.get("data_atualizacao"),
        )
    }

    fn try_from_row(row: &tokio_postgres::Row) -> Result<Self, tokio_postgres::Error> {
        let id: i32 = row.try_get("id")?;
        let cliente_id: i32 = row.try_get("cliente")?;
        let lanche_id: i32 = row.try_get("lanche")?;
        let acompanhamento_id: i32 = row.try_get("acompanhamento")?;
        let bebida_id: i32 = row.try_get("bebida")?;
        let status: i32 = row.try_get("status")?;

        Ok(PedidoFromRow::new(
            id as usize,
            cliente_id,
            lanche_id,
            acompanhamento_id,
            bebida_id,
            row.get("pagamento"),
            Status::from_index(status as usize),
            row.get("data_criacao"),
            row.get("data_atualizacao"),
        ))
    }
}
