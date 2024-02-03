use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use postgres_from_row::FromRow;
use std::collections::HashMap;
use std::io::prelude::*;
use std::str::FromStr;

use crate::core::domain::entities::produto::Categoria;
use crate::core::domain::entities::produto::Produto;
use crate::core::domain::value_objects::ingredientes::Ingredientes;
use tokio_postgres::error::Error as PgError;
use tokio_postgres::types::{FromSql, Type};

use super::table::{ColumnDefault, ColumnNullable, ColumnTypes};

pub fn get_produtos_table_columns() -> HashMap<String, (ColumnTypes, ColumnNullable, ColumnDefault)>
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
        "foto".to_string(),
        (
            ColumnTypes::Text,
            ColumnNullable(false),
            ColumnDefault(None),
        ),
    );
    columns.insert(
        "descricao".to_string(),
        (
            ColumnTypes::Text,
            ColumnNullable(false),
            ColumnDefault(None),
        ),
    );
    columns.insert(
        "categoria".to_string(),
        (
            ColumnTypes::ENUM(
                "categoria_type".to_string(),
                vec![
                    "lanche".to_string(),
                    "bebida".to_string(),
                    "acompanhamento".to_string(),
                    "sobremesa".to_string(),
                ],
            ),
            ColumnNullable(false),
            ColumnDefault(None),
        ),
    );
    columns.insert(
        "preco".to_string(),
        (
            ColumnTypes::Float,
            ColumnNullable(false),
            ColumnDefault(None),
        ),
    );
    columns.insert(
        "ingredientes".to_string(),
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

impl FromRow for Produto {
    fn from_row(row: &tokio_postgres::Row) -> Self {
        let id: i32 = row.get("id");
        let preco: f64 = row.get("preco");

        let lista_ingredientes: Vec<String> = row.get("ingredientes");
        let ingredientes = match Ingredientes::new(lista_ingredientes) {
            Ok(ing) => ing,
            Err(e) => panic!("Failed to create Ingredientes: {:?}", e),
        };

        let categoria: Categoria = row.get("categoria");
        //let categoria = Categoria::from_str(&categoria_str.to_lowercase()).unwrap_or_else(|_| panic!("Invalid Categoria: {}", categoria_str));
        let data_criacao =  row.get("data_criacao");
        let data_atualizacao =  row.get("data_atualizacao");

        Produto::new(
            id as usize,
            row.get("nome"),
            row.get("foto"),
            row.get("descricao"),
            categoria,
            preco,
            ingredientes,
            data_criacao,
            data_atualizacao,
        )
    }

    fn try_from_row(row: &tokio_postgres::Row) -> Result<Self, tokio_postgres::Error> {
        let id: i32 = row.try_get("id")?;
        let preco: f64 = row.try_get("preco")?;

        let lista_ingredientes_json: tokio_postgres::types::Json<Vec<String>> =
            row.get("ingredientes");
        let lista_ingredientes = lista_ingredientes_json.0;
        let ingredientes = match Ingredientes::new(lista_ingredientes) {
            Ok(ing) => ing,
            Err(e) => panic!("Failed to create Ingredientes: {:?}", e),
        };

        let categoria_str: String = row.get("categoria");
        let categoria = Categoria::from_str(&categoria_str)
            .unwrap_or_else(|_| panic!("Invalid Categoria: {}", categoria_str));

        Ok(Produto::new(
            id as usize,
            row.try_get("nome")?,
            row.try_get("foto")?,
            row.try_get("descricao")?,
            categoria,
            preco,
            ingredientes,
            row.get("created_at"),
            row.get("updated_at"),
        ))
    }
}
