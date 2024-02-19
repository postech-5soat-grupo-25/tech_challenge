use postgres_from_row::FromRow;
use tokio_postgres::Client;

use crate::{
    base::domain_error::DomainError, 
    entities::cliente::Cliente,
    traits::cliente_repository::ClienteRepository, 
    entities::cpf::Cpf,
};

use crate::external::postgres::table::Table;

const CREATE_CLIENTE: &str = "INSERT INTO cliente (nome, email, cpf, data_criacao, data_atualizacao) VALUES ($1, $2, $3, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP) RETURNING *";
const QUERY_CLIENTE_BY_CPF: &str = "SELECT * FROM cliente WHERE cpf = $1";
const QUERY_CLIENTE_BY_ID: &str = "SELECT * FROM cliente WHERE id = $1";
const QUERY_CLIENTES: &str = "SELECT * FROM cliente";
const DELETE_CLIENTE: &str = "DELETE FROM cliente WHERE cpf = $1 RETURNING *";

pub struct PostgresClienteRepository {
    client: Client,
    tables: Vec<Table>,
}

impl PostgresClienteRepository {
    pub async fn new(client: Client, tables: Vec<Table>) -> Self {
        let repo = PostgresClienteRepository { client, tables };
        repo.check_for_tables().await;
        repo
    }

    async fn check_for_tables(&self) {
        for table in self.tables.iter() {
            let query = table.get_create_if_not_exists_query();
            self.client.execute(query.as_str(), &[]).await.unwrap();
        }
    }
}

#[async_trait]
impl ClienteRepository for PostgresClienteRepository {
    async fn get_clientes(&self) -> Result<Vec<Cliente>, DomainError> {
        let clientes = self.client.query(QUERY_CLIENTES, &[]).await.unwrap();
        let mut clientes_vec = Vec::new();
        for cliente in clientes {
            clientes_vec.push(Cliente::from_row(&cliente));
        }
        Ok(clientes_vec)
    }

    async fn get_cliente_by_cpf(&self, cpf: Cpf) -> Result<Cliente, DomainError> {
        let cliente = self.client.query_one(QUERY_CLIENTE_BY_CPF, &[&cpf.0]).await;
        match cliente {
            Ok(cliente) => Ok(Cliente::from_row(&cliente)),
            Err(_) => Err(DomainError::NotFound),
        }
    }

    async fn get_cliente_by_id(&self, id: usize) -> Result<Cliente, DomainError> {
        let id = id as i32;
        let cliente = self.client.query_one(QUERY_CLIENTE_BY_ID, &[&id]).await;
        match cliente {
            Ok(cliente) => Ok(Cliente::from_row(&cliente)),
            Err(_) => Err(DomainError::NotFound),
        }
    }

    async fn create_cliente(&mut self, cliente: Cliente) -> Result<Cliente, DomainError> {
        let new_cliente = self
            .client
            .query(
                CREATE_CLIENTE,
                &[&cliente.nome(), &cliente.email(), &cliente.cpf().0],
            )
            .await
            .unwrap();
        let new_cliente = new_cliente.get(0);
        match new_cliente {
            Some(cliente) => {
                println!("Novo cliente cadastrado: {:?}", cliente);
                Ok(Cliente::from_row(cliente))
            }
            None => Err(DomainError::Invalid("Cliente".to_string())),
        }
    }

    async fn delete_cliente(&mut self, cpf: Cpf) -> Result<(), DomainError> {
        let deleted_cliente = self.client.query_one(DELETE_CLIENTE, &[&cpf.0]).await;
        match deleted_cliente {
            Ok(_) => Ok(()),
            _ => Err(DomainError::NotFound),
        }
    }
}
