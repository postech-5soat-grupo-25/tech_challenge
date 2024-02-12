use postgres_from_row::FromRow;
use std::sync::Arc;
use tokio_postgres::Client;
use rocket::futures::lock::Mutex;

use crate::core::domain::base::domain_error::DomainError;
use crate::core::domain::entities::cliente::Cliente;
use crate::core::domain::entities::pedido::{Pedido, Status};
use crate::core::domain::entities::produto::Produto;
use crate::core::domain::repositories::cliente_repository::ClienteRepository;
use crate::core::domain::repositories::pedido_repository::PedidoRepository;
use crate::core::domain::repositories::produto_repository::ProdutoRepository;

use super::super::postgres::table::Table;
use super::super::postgres::pedido::ProxyPedido;

const CREATE_PEDIDO: &str = "INSERT INTO pedido (cliente, lanche, acompanhamento, bebida, pagamento, status, data_criacao, data_atualizacao) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *";
const QUERY_PEDIDOS: &str = "SELECT * FROM pedido";
const QUERY_PEDIDO_BY_ID: &str = "SELECT * FROM pedido WHERE id = $1";
const QUERY_PEDIDOS_NOVOS: &str = "SELECT * FROM pedido where status = 0";
const SET_STATUS_PEDIDO: &str = "UPDATE pedido SET status = $2 WHERE id = $1";
const SET_PEDIDO_LANCHE: &str = "UPDATE pedido SET lanche = $2 WHERE id = $1";
const SET_PEDIDO_ACOMPANHAMENTO: &str = "UPDATE pedido SET acompanhamento = $2 WHERE id = $1";
const SET_PEDIDO_BEBIDA: &str = "UPDATE pedido SET bebida = $2 WHERE id = $1";
const SET_PEDIDO_PAGAMENTO: &str = "UPDATE pedido SET pagamento = $2 WHERE id = $1";

pub struct PostgresPedidoRepository {
    client: Client,
    tables: Vec<Table>,
    cliente_repository: Arc<Mutex<dyn ClienteRepository + Send + Sync>>,
    produto_repository: Arc<Mutex<dyn ProdutoRepository + Send + Sync>>,
}

impl PostgresPedidoRepository {
    pub async fn new(
        client: Client,
        tables: Vec<Table>,
        cliente_repository: Arc<Mutex<dyn ClienteRepository + Send + Sync>>,
        produto_repository: Arc<Mutex<dyn ProdutoRepository + Send + Sync>>,
    ) -> Self {
        let repo = PostgresPedidoRepository {
            client,
            tables,
            cliente_repository,
            produto_repository,
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

    async fn pedido_from_row(&self, pedido_row: &tokio_postgres::Row) -> Pedido {
        let cliente_repository_lock = self.cliente_repository.lock().await;
        let produto_repository_lock = self.produto_repository.lock().await;

        let pedido_aux: ProxyPedido = ProxyPedido::from_row(&pedido_row);
        
        let cliente_result: Result<Cliente, DomainError> = cliente_repository_lock
            .get_cliente_by_id(*pedido_aux.cliente_id())
            .await;
        let cliente: Cliente = cliente_result.unwrap();

        let lanche_result: Result<Produto, DomainError> = produto_repository_lock
            .get_produto_by_id(*pedido_aux.lanche_id())
            .await;
        let lanche: Produto = lanche_result.unwrap();

        let bebida_result: Result<Produto, DomainError> = produto_repository_lock
            .get_produto_by_id(*pedido_aux.bebida_id())
            .await;
        let bebida: Produto = bebida_result.unwrap();

        let acompanhamento_result: Result<Produto, DomainError> = produto_repository_lock
            .get_produto_by_id(*pedido_aux.acompanhamento_id() as usize)
            .await;
        let acompanhamento: Produto = acompanhamento_result.unwrap();

        Pedido::new(
            *pedido_aux.id(),
            Some(cliente),
            Some(lanche),
            Some(acompanhamento),
            Some(bebida),
            pedido_aux.pagamento().clone(),
            pedido_aux.status().clone(),
            pedido_aux.data_criacao().clone(),
            pedido_aux.data_atualizacao().clone(),
        )
    }
}

#[async_trait]
impl PedidoRepository for PostgresPedidoRepository {
    async fn lista_pedidos(&mut self) -> Result<Vec<Pedido>, DomainError> {
        let pedidos = self.client.query(QUERY_PEDIDOS, &[]).await.unwrap();
        let mut pedidos_vec = Vec::new();
        for pedido in pedidos {
            let pedido_aux = self.pedido_from_row(&pedido).await;
            pedidos_vec.push(pedido_aux.clone());
        }
        Ok(pedidos_vec)
    }

    async fn get_pedidos_novos(&self) -> Result<Vec<Pedido>, DomainError> {
        let cliente_repository_lock = self.cliente_repository.lock().await;
        let produto_repository_lock = self.produto_repository.lock().await;

        let pedidos = self.client.query(QUERY_PEDIDOS_NOVOS, &[]).await.unwrap();
        let mut pedidos_vec = Vec::new();
        for pedido in pedidos {
            let pedido_aux: ProxyPedido = ProxyPedido::from_row(&pedido);
            let cliente_result: Result<Cliente, DomainError> = cliente_repository_lock
                .get_cliente_by_id(*pedido_aux.id())
                .await;
            let cliente: Cliente = cliente_result.unwrap();
            let lanche_result: Result<Produto, DomainError> = produto_repository_lock
                .get_produto_by_id(*pedido_aux.cliente_id())
                .await;
            let lanche: Produto = lanche_result.unwrap();
            let bebida_result: Result<Produto, DomainError> = produto_repository_lock
                .get_produto_by_id(*pedido_aux.bebida_id())
                .await;
            let bebida: Produto = bebida_result.unwrap();
            let acompanhamento_result: Result<Produto, DomainError> = produto_repository_lock
                .get_produto_by_id(*pedido_aux.acompanhamento_id())
                .await;
            let acompanhamento: Produto = acompanhamento_result.unwrap();

            let pedido: Pedido = Pedido::new(
                *pedido_aux.id(),
                Some(cliente),
                Some(lanche),
                Some(acompanhamento),
                Some(bebida),
                pedido_aux.pagamento().clone(),
                pedido_aux.status().clone(),
                pedido_aux.data_criacao().clone(),
                pedido_aux.data_atualizacao().clone(),
            );

            pedidos_vec.push(pedido.clone());
        }
        Ok(pedidos_vec)
    }

    async fn atualiza_status(&mut self, id: usize, status: Status) -> Result<Pedido, DomainError> {
        let _id = id as i32;
        let updated_pedido = self
            .client
            .query(SET_STATUS_PEDIDO, &[&_id, &status.to_string()])
            .await
            .unwrap();

        let updated_pedido = updated_pedido.get(0);
        match updated_pedido {
            Some(pedido) => Ok(self.pedido_from_row(&pedido).await),
            None => Err(DomainError::NotFound),
        }
    }

    async fn create_pedido(&mut self, pedido: Pedido) -> Result<Pedido, DomainError> {
        let cliente_repository_lock = self.cliente_repository.lock().await;
        let produto_repository_lock = self.produto_repository.lock().await;

        let cliente_id = match pedido.cliente() {
            Some(cliente) => Some(cliente.id().clone() as i32),
            None => None,
        };
        let lanche_id = match pedido.lanche() {
            Some(lanche) => Some(lanche.id().clone() as i32),
            None => None,
        };
        let acompanhamento_id = match pedido.acompanhamento() {
            Some(acompanhamento) => Some(acompanhamento.id().clone() as i32),
            None => None,
        };
        let bebida_id = match pedido.bebida() {
            Some(bebida) => Some(bebida.id().clone() as i32),
            None => None,
        };

        let status = pedido.status();

        let new_pedido = self
            .client
            .query(
                CREATE_PEDIDO,
                &[
                    &cliente_id,
                    &lanche_id,
                    &acompanhamento_id,
                    &bebida_id,
                    &pedido.pagamento(),
                    &status.to_string(),
                    &pedido.data_criacao(),
                    &pedido.data_atualizacao(),
                ],
            )
            .await
            .unwrap();
        
        let new_pedido = new_pedido.get(0);
        match new_pedido {
            Some(pedido) => {
                let pedido_aux: ProxyPedido = ProxyPedido::from_row(&pedido);
                let cliente_result: Result<Cliente, DomainError> = cliente_repository_lock
                    .get_cliente_by_id(*pedido_aux.cliente_id())
                    .await;
                let cliente: Cliente = cliente_result.unwrap();
                let lanche_result: Result<Produto, DomainError> = produto_repository_lock
                    .get_produto_by_id(*pedido_aux.lanche_id())
                    .await;
                let lanche: Produto = lanche_result.unwrap();
                let bebida_result: Result<Produto, DomainError> = produto_repository_lock
                    .get_produto_by_id(*pedido_aux.bebida_id())
                    .await;
                let bebida: Produto = bebida_result.unwrap();
                let acompanhamento_result: Result<Produto, DomainError> = produto_repository_lock
                    .get_produto_by_id(*pedido_aux.acompanhamento_id())
                    .await;
                let acompanhamento: Produto = acompanhamento_result.unwrap();

                let pedido: Pedido = Pedido::new(
                    *pedido_aux.id(),
                    Some(cliente),
                    Some(lanche),
                    Some(acompanhamento),
                    Some(bebida),
                    pedido_aux.pagamento().clone(),
                    pedido_aux.status().clone(),
                    pedido_aux.data_criacao().clone(),
                    pedido_aux.data_atualizacao().clone(),
                );
                println!("Novo pedido cadastrado: {:?}", pedido);
                Ok(pedido)
            }
            None => Err(DomainError::Invalid("Pedido".to_string())),
        }
    }

    async fn get_pedido_by_id(&self, pedido_id: usize) -> Result<Pedido, DomainError> {
        let cliente_repository_lock = self.cliente_repository.lock().await;
        let produto_repository_lock = self.produto_repository.lock().await;

        let pedido_id = pedido_id as i32;
        let pedido = self
            .client
            .query_one(QUERY_PEDIDO_BY_ID, &[&pedido_id])
            .await;

        match pedido {
            Ok(pedido) => {
                let pedido_aux: ProxyPedido = ProxyPedido::from_row(&pedido);
                let cliente_result: Result<Cliente, DomainError> = cliente_repository_lock
                    .get_cliente_by_id(*pedido_aux.cliente_id())
                    .await;
                let cliente: Cliente = cliente_result.unwrap();
                let lanche_result: Result<Produto, DomainError> = produto_repository_lock
                    .get_produto_by_id(*pedido_aux.lanche_id())
                    .await;
                let lanche: Produto = lanche_result.unwrap();
                let bebida_result: Result<Produto, DomainError> = produto_repository_lock
                    .get_produto_by_id(*pedido_aux.bebida_id())
                    .await;
                let bebida: Produto = bebida_result.unwrap();
                let acompanhamento_result: Result<Produto, DomainError> = produto_repository_lock
                    .get_produto_by_id(*pedido_aux.acompanhamento_id())
                    .await;
                let acompanhamento: Produto = acompanhamento_result.unwrap();

                let pedido: Pedido = Pedido::new(
                    *pedido_aux.id(),
                    Some(cliente),
                    Some(lanche),
                    Some(acompanhamento),
                    Some(bebida),
                    pedido_aux.pagamento().clone(),
                    pedido_aux.status().clone(),
                    pedido_aux.data_criacao().clone(),
                    pedido_aux.data_atualizacao().clone(),
                );
                Ok(pedido)
            },
            Err(_) => Err(DomainError::NotFound),
        }
    }

    async fn cadastrar_lanche(
        &mut self,
        pedido_id: usize,
        lanche: Produto,
    ) -> Result<Pedido, DomainError> {
        let _pedido_id: i32 = pedido_id as i32;
        let _lanche_id = *lanche.id() as i32;

        let updated_pedido = self
            .client
            .query(SET_PEDIDO_LANCHE, &[&_pedido_id, &_lanche_id])
            .await
            .unwrap();

        let updated_pedido = updated_pedido.get(0);
        match updated_pedido {
            Some(pedido) => Ok(self.pedido_from_row(&pedido).await),
            None => Err(DomainError::NotFound),
        }
    }

    async fn cadastrar_acompanhamento(
        &mut self,
        pedido_id: usize,
        acompanhamento: Produto,
    ) -> Result<Pedido, DomainError> {
        let _pedido_id: i32 = pedido_id as i32;
        let _acompanhamento_id = *acompanhamento.id() as i32;

        let updated_pedido = self
            .client
            .query(SET_PEDIDO_ACOMPANHAMENTO, &[&_pedido_id, &_acompanhamento_id])
            .await
            .unwrap();

        let updated_pedido = updated_pedido.get(0);
        match updated_pedido {
            Some(pedido) => Ok(self.pedido_from_row(&pedido).await),
            None => Err(DomainError::NotFound),
        }
    }

    async fn cadastrar_bebida(
        &mut self,
        pedido_id: usize,
        bebida: Produto,
    ) -> Result<Pedido, DomainError> {
        let _pedido_id: i32 = pedido_id as i32;
        let _bebida_id = *bebida.id() as i32;

        let updated_pedido = self
            .client
            .query(SET_PEDIDO_BEBIDA, &[&_pedido_id, &_bebida_id])
            .await
            .unwrap();

        let updated_pedido = updated_pedido.get(0);
        match updated_pedido {
            Some(pedido) => Ok(self.pedido_from_row(&pedido).await),
            None => Err(DomainError::NotFound),
        }
    }

    async fn cadastrar_pagamento(
        &mut self,
        pedido_id: usize,
        pagamento: String,
    ) -> Result<Pedido, DomainError> {
        let _pedido_id: i32 = pedido_id as i32;

        let updated_pedido = self
            .client
            .query(SET_PEDIDO_PAGAMENTO, &[&_pedido_id, &pagamento])
            .await
            .unwrap();

        let updated_pedido = updated_pedido.get(0);
        match updated_pedido {
            Some(pedido) => Ok(self.pedido_from_row(&pedido).await),
            None => Err(DomainError::NotFound),
        }
    }

}
