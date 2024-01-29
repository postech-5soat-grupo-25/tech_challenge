
pub mod table;
pub mod usuarios;
pub mod clientes;
pub mod pedidos;
pub mod produtos;

use tokio_postgres::{NoTls, Error, Client};
use rocket::tokio;

use self::table::{Table, TablesNames};
use self::usuarios::get_usuario_table_columns;
use self::clientes::get_cliente_table_columns;
use self::pedidos::get_pedidos_table_columns;
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
      name: TablesNames::Usuario,
      columns: get_usuario_table_columns(),
    },
    Table {
      name: TablesNames::Cliente,
      columns: get_cliente_table_columns(),
    },
    Table {
      name: TablesNames::Pedido,
      columns: get_pedidos_table_columns(),
    },
    Table {
      name: TablesNames::Produtos,
      columns: get_pedidos_table_columns(),
    },
  ]
}
