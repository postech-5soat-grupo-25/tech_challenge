
pub mod table;
pub mod usuario;
pub mod cliente;
pub mod pedido;
pub mod produto;

use tokio_postgres::{NoTls, Error, Client};
use tokio;

use self::table::{Table, TablesNames};
use self::usuario::get_usuario_table_columns;
use self::cliente::get_cliente_table_columns;
use self::produto::get_produto_table_columns;
use self::pedido::get_pedido_table_columns;
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
      name: TablesNames::Produto,
      columns: get_produto_table_columns(),
    },
    Table {
      name: TablesNames::Pedido,
      columns: get_pedido_table_columns(),
    },
  ]
}
