
pub mod users;
pub mod product;
pub mod table;

use std::collections::HashMap;

use tokio_postgres::{NoTls, Error, Client};
use rocket::tokio;

use self::table::{Table, TablesNames};
use self::users::get_users_table_columns;
use self::product::get_products_table_columns;

pub struct PgConnectionManager {
  pub client: Client,
}

impl PgConnectionManager {
  pub async fn new(db_url: String) -> Result<Self, Error> {
    let (client, connection)  = tokio_postgres::connect(&db_url, NoTls).await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    Ok(PgConnectionManager { client })
  }
}

pub fn get_tables() -> Vec<Table> {
  vec![
    Table {
      name: TablesNames::Users,
      columns: get_users_table_columns(),
      enums: HashMap::new(),
    },
    Table {
      name: TablesNames::Products,
      columns: get_products_table_columns(),
      enums: {
        let mut enums = HashMap::new();
        enums.insert("categoria_type".to_string(), vec!["lanche".to_string(), "bebida".to_string(), "acompanhamento".to_string(), "sobremesa".to_string()]);
        enums
    }, // ENUMs for the Products table
    },
  ]
}
