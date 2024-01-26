use postgres_from_row::FromRow;
use tokio_postgres::Client;

use crate::adapter::driven::infra::postgres::pedidos;
use crate::core::domain::base::domain_error::DomainError;
use crate::core::domain::entities::pedido::Pedido;
use crate::core::domain::repositories::pedido_repository::PedidoRepository;
use crate::core::domain::value_objects::cpf::Cpf;

use super::super::postgres::table::Table;

const QUERY_PEDIDOS: &str = "SELECT * FROM pedidos";
const QUERY_PEDIDOS_NOVOS: &str = "SELECT * FROM pedidos where status = 0";
const SET_STATUS_PEDIDO: &str = "UPDATE pedidos SET status = $1 WHERE id = $2";
const DELETE_PEDIDO: &str = "DELETE FROM pedidos WHERE id = $1";

pub struct PostgresPedidoRepository {
    client: Client,
    tables: Vec<Table>,
}

impl PostgresPedidoRepository {
    pub async fn new(client: Client, tables: Vec<Table>) -> Self {
        let repo = PostgresPedidoRepository { client, tables };
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
impl PedidoRepository for PostgresPedidoRepository {
    async fn get_pedidos_novos(&self) -> Result<Vec<Pedido>, DomainError> {
        let pedidos = self.client.query(QUERY_PEDIDOS_NOVOS, &[]).await.unwrap();
        let mut pedidos_vec = Vec::new();
        for pedido in pedidos {
            pedidos_vec.push(Pedido::from_row(&pedido));
        }
        Ok(pedidos_vec)
    }

    async fn atualizar_status_pedido(&mut self, id: usize, status :String) -> Result<Pedido, DomainError> {
        let status_enum = Status::from_string(status);
        if (status_enum == Status::Invalido){
            return Err::<Pedido, _>(DomainError::Invalid("status".to_string()));
        }
        let = status_enum.to_index()
        let updated_pedido = self.client.query(SET_STATUS_PEDIDO, &[
            &index,
            &id,
        ]).await.unwrap();

        let updated_pedido = updated_pedido.get(0);
        match updated_user {
            Some(pedido) => {
                Ok(Pedido::from_row(pedido))
            },
            None => {
                Err(DomainError::NotFound)
            }
        }
    }


    // async fn get_pedidos(&self) -> Result<Vec<Pedido>, DomainError> {
    //     let pedidos = self.client.query(QUERY_PEDIDOS, &[]).await.unwrap();
    //     let mut pedidos_vec = Vec::new();
    //     for pedido in pedidos {
    //         pedidos_vec.push(Pedido::from_row(&pedido));
    //     }
    //     Ok(pedidos_vec)
    // }

    // async fn create_pedido(&mut self, pedido: Pedido) -> Result<Pedido, DomainError> {
    //     let new_pedido = self
    //         .client
    //         .query(
    //             CREATE_PEDIDO,
    //             &[
    //                 &pedido.cliente(),
    //                 &pedido.lanche(),
    //                 &pedido.acompanhamento(),
    //                 &pedido.pagamento(),
    //                 &pedido.status(),
    //                 &pedido.data_criacao(),
    //                 &pedido.data_atualizacao(),
    //             ],
    //         )
    //         .await
    //         .unwrap();
    //     let new_pedido = new_pedido.get(0);
    //     match new_pedido {
    //         Some(pedido) => {
    //             println!("Novo pedido cadastrado: {:?}", pedido);
    //             Ok(Pedido::from_row(pedido))
    //         }
    //         None => Err(DomainError::Invalid("Pedido".to_string())),
    //     }
    // }

    // async fn delete_pedido(&mut self, id: usize) -> Result<(), DomainError> {
    //     let deleted_pedido = self.client.query_one(DELETE_PEDIDO, &[&id]).await;
    //     match deleted_pedido {
    //         Ok(_) => Ok(()),
    //         _ => Err(DomainError::NotFound),
    //     }
    // }
}
