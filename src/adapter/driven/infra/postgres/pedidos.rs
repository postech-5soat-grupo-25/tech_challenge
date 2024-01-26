use postgres_from_row::FromRow;
use std::collections::HashMap;

use crate::core::domain::entities::pedido::Pedido;
use crate::core::domain::entities::cliente::Cliente;
use crate::core::domain::entities::produto::Produto;
use crate::core::domain::entities::pedido::Status;

use crate::core::domain::entities::produto::Categoria;
use crate::core::domain::value_objects::ingredientes::Ingredientes;
use crate::core::domain::value_objects::cpf::Cpf;

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

impl FromRow for Pedido {
    fn from_row(row: &tokio_postgres::Row) -> Self {
        let id: i32 = row.get("id");
        let cliente_id: i32 = row.get("cliente");
        let lanche_id: i32 = row.get("lanche");
        let acompanhamento_id: i32 = row.get("acompanhamento");
        let bebida_id: i32 = row.get("bebida");
        let status: i32 = row.get("status");

        let p = Produto::new(
            1,
            "Cheeseburger".to_string(),
            "cheeseburger.png".to_string(),
            "O clássico pão, carne e queijo!".to_string(),
            Categoria::Lanche,
            9.99,
            Ingredientes::new(
            vec!["Pão".to_string(), "Hambúrguer".to_string(), "Queijo".to_string()]).unwrap(),
            "2024-01-16".to_string(),
            "2024-01-16".to_string(),
        );

        let cliente = Cliente::new(
            1,
            "Fulano da Silva".to_string(),
            "fulano.silva@exemplo.com".to_string(),
            Cpf::new("123.456.789-09".to_string()).unwrap(),
            "2024-01-17".to_string(),
            "2024-01-17".to_string(),
        );

        let lanche = p.clone();
        let acompanhamento = p.clone();
        let bebida = p.clone();

        
        Pedido::new(
            id as usize,
            Some(cliente),
            Some(lanche),
            Some(acompanhamento),
            Some(bebida),
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

        let cliente = Cliente::new(
            1,
            "Fulano da Silva".to_string(),
            "fulano.silva@exemplo.com".to_string(),
            Cpf::new("123.456.789-09".to_string()).unwrap(),
            "2024-01-17".to_string(),
            "2024-01-17".to_string(),
        );

        let p = Produto::new(
            1,
            "Cheeseburger".to_string(),
            "cheeseburger.png".to_string(),
            "O clássico pão, carne e queijo!".to_string(),
            Categoria::Lanche,
            9.99,
            Ingredientes::new(
            vec!["Pão".to_string(), "Hambúrguer".to_string(), "Queijo".to_string()]).unwrap(),
            "2024-01-16".to_string(),
            "2024-01-16".to_string(),
        );

        let lanche = p.clone();
        let acompanhamento = p.clone();
        let bebida = p.clone();
        // TODO create a way to get pedidos by id (lanche acompanhamento, bebida) and cliente by id
        
        Ok(Pedido::new(
            id as usize,
            Some(cliente),
            Some(lanche),
            Some(acompanhamento),
            Some(bebida),
            row.try_get("pagamento")?,
            Status::from_index(status as usize),
            row.try_get("data_criacao")?,
            row.try_get("data_atualizacao")?,
        ))
    }
}
