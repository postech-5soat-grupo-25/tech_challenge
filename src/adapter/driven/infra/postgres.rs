
pub mod users;
pub mod table;

use tokio_postgres::{NoTls, Error, Client};
use rocket::tokio;

use self::table::{Table, TablesNames};
use self::users::get_users_table_columns;
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
    },
  ]
}
