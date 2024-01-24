use postgres_from_row::FromRow;
use tokio_postgres::Client;

use crate::adapter::driven::infra::postgres::clientes;
use crate::core::domain::base::domain_error::DomainError;
use crate::core::domain::entities::cliente::Cliente;
use crate::core::domain::repositories::cliente_repository::ClienteRepository;
use crate::core::domain::value_objects::cpf::Cpf;

use super::super::postgres::table::Table;

const CREATE_CLIENTE: &str = "INSERT INTO clientes (nome, email, cpf, data_criacao, data_atualizacao) VALUES ($1, $2, $3, $4, $5) RETURNING *";
const QUERY_CLIENTE_BY_CPF: &str = "SELECT * FROM clientes WHERE cpf = $1";
const QUERY_CLIENTES: &str = "SELECT * FROM clientes";
const DELETE_CLIENTE: &str = "DELETE FROM clientes WHERE id = $1";

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

    async fn create_cliente(&mut self, cliente: Cliente) -> Result<Cliente, DomainError> {
        let new_cliente = self
            .client
            .query(
                CREATE_CLIENTE,
                &[
                    &cliente.nome(),
                    &cliente.email(),
                    &cliente.cpf().0,
                    &cliente.data_criacao(),
                    &cliente.data_atualizacao(),
                ],
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

    async fn delete_cliente(&mut self, id: usize) -> Result<(), DomainError> {
        let id = id as i32;
        let deleted_cliente = self.client.query_one(DELETE_CLIENTE, &[&id]).await;
        match deleted_cliente {
            Ok(_) => Ok(()),
            _ => Err(DomainError::NotFound),
        }
    }
}
