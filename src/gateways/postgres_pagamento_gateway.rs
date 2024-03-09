use bytes::BytesMut;
use postgres_from_row::FromRow;
use std::error::Error;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_postgres::types::{FromSql, ToSql, Type};
use tokio_postgres::Client;

use crate::base::domain_error::DomainError;
use crate::entities::cliente::Cliente;
use crate::entities::pagamento::Pagamento;
use crate::entities::pedido::Pedido;
use crate::entities::produto::Produto;
use crate::traits::cliente_gateway::ClienteGateway;
use crate::traits::pagamento_gateway::PagamentoGateway;
use crate::traits::produto_gateway::ProdutoGateway;
use crate::traits::pedido_gateway::PedidoGateway;

use crate::external::postgres::table::Table;


const CREATE_PAGAMENTO: &str = "INSERT INTO pagamento (id_pedido, estado, valor, metodo, referencia, data_criacao) VALUES ($1, $2, $3, $4, $5, CURRENT_TIMESTAMP) RETURNING id, id_pedido, estado, valor, metodo, referencia, data_criacao";
const QUERY_PAGAMENTO_BY_ID_PEDIDO: &str = "SELECT * FROM pagamento WHERE id_pedido = $1 order by data_criacao DESC limit 1";
const UPDATE_PAGAMENTO: &str = "UPDATE pagamento SET id_pedido = $2, estado = $3, valor = $4, metodo = $5, referencia = $6 WHERE id = $1 RETURNING id, id_pedido, estado, metodo, valor, referencia, data_criacao";


pub struct PostgresPagamentoRepository {
    client: Client,
    tables: Vec<Table>,
    pedido_repository: Arc<Mutex<dyn PedidoGateway + Send + Sync>>,
}

impl PostgresPagamentoRepository {
    pub async fn new(
        client: Client,
        tables: Vec<Table>,
        pedido_repository: Arc<Mutex<dyn PedidoGateway + Send + Sync>>,
    ) -> Self {
        let repo = PostgresPagamentoRepository {
            client,
            tables,
            pedido_repository,
        };
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
impl PagamentoGateway for PostgresPagamentoRepository {
    async fn create_pagamento(
        &mut self,
        pagamento: Pagamento
    ) -> Result<Pagamento, DomainError> {
        let _id_pedido = *pagamento.id_pedido() as i32;
        let new_pagamento = self
            .client
            .query(
                CREATE_PAGAMENTO,
                &[
                    &_id_pedido,
                    &String::from("pendente"),
                    &pagamento.valor(),
                    &pagamento.metodo(),
                    &pagamento.referencia(),
                ],
            )
            .await
            .unwrap();
        let new_pagamento = new_pagamento.get(0);
        match new_pagamento {
            Some(pagamento) => Ok(Pagamento::from_row(pagamento)),
            None => Err(DomainError::Invalid("Pagamento".to_string())),
        }
    }

    async fn get_pagamento_by_id_pedido(&mut self, id_pedido: usize) -> Result<Pagamento, DomainError> {
        let _id_pedido = id_pedido as i32;

        let pagamento_row = self.client.query(QUERY_PAGAMENTO_BY_ID_PEDIDO, &[&_id_pedido]).await.unwrap();
        let pagamento = pagamento_row.get(0);
        match pagamento {
            Some(pagamento) => Ok(Pagamento::from_row(&pagamento)),
            None => Err(DomainError::NotFound),
        }
    }

    async fn update_pagamento(
        &mut self,
        pagamento: Pagamento
    ) -> Result<Pagamento, DomainError> {
        let _id = *pagamento.id() as i32;
        let _id_pedido = *pagamento.id_pedido() as i32;
        let update_pagamento_row = self
            .client
            .query_one(
                UPDATE_PAGAMENTO,
                &[
                    &_id,
                    &_id_pedido,
                    &pagamento.estado(),
                    &pagamento.valor(),
                    &pagamento.metodo(),
                    &pagamento.referencia(),
                ],
            )
            .await;
        match update_pagamento_row {
            Ok(row) => {
                let updated_pagamento: Pagamento = Pagamento::from_row(&row);
                Ok(updated_pagamento)
            }
            Err(_) => Err(DomainError::Invalid("Pagamento".to_string())),
        }
    }

    // async fn atualiza_status(&mut self, id: usize, status: Status) -> Result<Pagamento, DomainError> {
    //     let _id = id as i32;
    //     let updated_pedido = self
    //         .client
    //         .query(SET_PEDIDO_STATUS, &[&_id, &status])
    //         .await
    //         .unwrap();

    //     let updated_pedido = updated_pedido.get(0);
    //     match updated_pedido {
    //         Some(pedido) => Ok(self.pedido_from_proxy(&pedido).await),
    //         None => Err(DomainError::NotFound),
    //     }
    // }


}
