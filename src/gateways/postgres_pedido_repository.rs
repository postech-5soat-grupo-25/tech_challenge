use bytes::BytesMut;
use postgres_from_row::FromRow;
use rocket::futures::lock::Mutex;
use std::error::Error;
use std::sync::Arc;
use tokio_postgres::types::{FromSql, ToSql, Type};
use tokio_postgres::Client;

use crate::base::domain_error::DomainError;
use crate::entities::cliente::Cliente;
use crate::entities::pedido::{Pedido, Status};
use crate::entities::produto::Produto;
use crate::traits::cliente_repository::ClienteRepository;
use crate::traits::pedido_repository::PedidoRepository;
use crate::traits::produto_repository::ProdutoRepository;

use crate::external::postgres::pedido::ProxyPedido;
use crate::external::postgres::table::Table;

const CREATE_PEDIDO: &str = "INSERT INTO pedido (cliente_id, lanche_id, acompanhamento_id, bebida_id, pagamento, status, data_criacao, data_atualizacao) VALUES ($1, $2, $3, $4, $5, $6, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP) RETURNING id, cliente_id, lanche_id, acompanhamento_id, bebida_id, pagamento, CAST(status AS VARCHAR), data_criacao, data_atualizacao";
const QUERY_PEDIDOS: &str = "SELECT id, cliente_id, lanche_id, acompanhamento_id, bebida_id, pagamento, CAST(status AS VARCHAR), data_criacao, data_atualizacao FROM pedido";
const QUERY_PEDIDO_BY_ID: &str = "SELECT id, cliente_id, lanche_id, acompanhamento_id, bebida_id, pagamento, CAST(status AS VARCHAR), data_criacao, data_atualizacao FROM pedido WHERE id = $1";
const QUERY_PEDIDOS_NOVOS: &str = "SELECT id, cliente_id, lanche_id, acompanhamento_id, bebida_id, pagamento, CAST(status AS VARCHAR), data_criacao, data_atualizacao FROM pedido WHERE status IN ('Recebido', 'EmPreparacao')";
const SET_PEDIDO_STATUS: &str = "UPDATE pedido SET status = $2, data_atualizacao = CURRENT_TIMESTAMP WHERE id = $1 RETURNING id, cliente_id, lanche_id, acompanhamento_id, bebida_id, pagamento, CAST(status AS VARCHAR), data_criacao, data_atualizacao";
const SET_PEDIDO_CLIENTE: &str = "UPDATE pedido SET cliente_id = $2 WHERE id = $1 RETURNING id, cliente_id, lanche_id, acompanhamento_id, bebida_id, pagamento, CAST(status AS VARCHAR), data_criacao, data_atualizacao";
const SET_PEDIDO_LANCHE: &str = "UPDATE pedido SET lanche_id = $2 WHERE id = $1 RETURNING id, cliente_id, lanche_id, acompanhamento_id, bebida_id, pagamento, CAST(status AS VARCHAR), data_criacao, data_atualizacao";
const SET_PEDIDO_ACOMPANHAMENTO: &str = "UPDATE pedido SET acompanhamento_id = $2 WHERE id = $1 RETURNING id, cliente_id, lanche_id, acompanhamento_id, bebida_id, pagamento, CAST(status AS VARCHAR), data_criacao, data_atualizacao";
const SET_PEDIDO_BEBIDA: &str = "UPDATE pedido SET bebida_id = $2 WHERE id = $1 RETURNING id, cliente_id, lanche_id, acompanhamento_id, bebida_id, pagamento, CAST(status AS VARCHAR), data_criacao, data_atualizacao";
const SET_PEDIDO_PAGAMENTO: &str = "UPDATE pedido SET pagamento = $2 WHERE id = $1 RETURNING id, cliente_id, lanche_id, acompanhamento_id, bebida_id, pagamento, CAST(status AS VARCHAR), data_criacao, data_atualizacao";

impl<'a> FromSql<'a> for Status {
    fn from_sql(
        _ty: &tokio_postgres::types::Type,
        raw: &'a [u8],
    ) -> Result<Self, Box<dyn Error + Sync + Send>> {
        let value = std::str::from_utf8(raw)?;

        match value {
            "Recebido" => Ok(Status::Recebido),
            "EmPreparacao" => Ok(Status::EmPreparacao),
            "Pronto" => Ok(Status::Pronto),
            "Pendente" => Ok(Status::Pendente),
            "Finalizado" => Ok(Status::Finalizado),
            "Cancelado" => Ok(Status::Cancelado),
            "Invalido" => Ok(Status::Invalido),
            _ => Err("Invalid Status value".into()),
        }
    }
    fn accepts(_ty: &tokio_postgres::types::Type) -> bool {
        true
    }
}

impl ToSql for Status {
    fn to_sql(
        &self,
        _ty: &Type,
        out: &mut BytesMut,
    ) -> Result<tokio_postgres::types::IsNull, Box<dyn std::error::Error + 'static + Send + Sync>>
    {
        match self {
            Status::Recebido => out.extend_from_slice(b"Recebido"),
            Status::EmPreparacao => out.extend_from_slice(b"EmPreparacao"),
            Status::Pronto => out.extend_from_slice(b"Pronto"),
            Status::Pendente => out.extend_from_slice(b"Pendente"),
            Status::Finalizado => out.extend_from_slice(b"Finalizado"),
            Status::Cancelado => out.extend_from_slice(b"Cancelado"),
            Status::Invalido => out.extend_from_slice(b"Invalido"),
        }
        Ok(tokio_postgres::types::IsNull::No)
    }

    fn accepts(_ty: &Type) -> bool {
        true
    }

    fn to_sql_checked(
        &self,
        ty: &Type,
        out: &mut BytesMut,
    ) -> Result<tokio_postgres::types::IsNull, Box<dyn std::error::Error + 'static + Send + Sync>>
    {
        self.to_sql(ty, out)
    }
}

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

    async fn pedido_from_proxy(&self, pedido_row: &tokio_postgres::Row) -> Pedido {
        let _pedido: ProxyPedido = ProxyPedido::from_row(&pedido_row);
    
        let cliente = if let Some(cliente_id) = _pedido.cliente_id() {
            let cliente_repo = self.cliente_repository.lock().await;
            cliente_repo.get_cliente_by_id(*cliente_id).await.ok()
        } else {
            None
        };
    
        let lanche = if let Some(lanche_id) = _pedido.lanche_id() {
            let produto_repo = self.produto_repository.lock().await;
            produto_repo.get_produto_by_id(*lanche_id).await.ok()
        } else {
            None
        };
    
        let bebida = if let Some(bebida_id) = _pedido.bebida_id() {
            let produto_repo = self.produto_repository.lock().await;
            produto_repo.get_produto_by_id(*bebida_id).await.ok()
        } else {
            None
        };
    
        let acompanhamento = if let Some(acompanhamento_id) = _pedido.acompanhamento_id() {
            let produto_repo = self.produto_repository.lock().await;
            produto_repo.get_produto_by_id(*acompanhamento_id).await.ok()
        } else {
            None
        };

        Pedido::new(
            *_pedido.id(),
            cliente,
            lanche,
            acompanhamento,
            bebida,
            _pedido.pagamento().clone(),
            _pedido.status().clone(),
            _pedido.data_criacao().clone(),
            _pedido.data_atualizacao().clone(),
        )
    }
}

#[async_trait]
impl PedidoRepository for PostgresPedidoRepository {
    async fn lista_pedidos(&mut self) -> Result<Vec<Pedido>, DomainError> {
        let pedidos = self.client.query(QUERY_PEDIDOS, &[]).await.unwrap();
        let mut pedidos_vec = Vec::new();
        for pedido in pedidos {
            let _pedido = self.pedido_from_proxy(&pedido).await;
            pedidos_vec.push(_pedido.clone());
        }
        Ok(pedidos_vec)
    }

    async fn get_pedidos_novos(&self) -> Result<Vec<Pedido>, DomainError> {
        let pedidos_rows = self.client.query(QUERY_PEDIDOS_NOVOS, &[]).await.unwrap();
        let mut pedidos_vec = Vec::new();

        for pedido_row in pedidos_rows {
            let pedido = self.pedido_from_proxy(&pedido_row).await;
            pedidos_vec.push(pedido);
        }

        Ok(pedidos_vec)
    }

    async fn atualiza_status(&mut self, id: usize, status: Status) -> Result<Pedido, DomainError> {
        let _id = id as i32;
        let updated_pedido = self
            .client
            .query(SET_PEDIDO_STATUS, &[&_id, &status])
            .await
            .unwrap();

        let updated_pedido = updated_pedido.get(0);
        match updated_pedido {
            Some(pedido) => Ok(self.pedido_from_proxy(&pedido).await),
            None => Err(DomainError::NotFound),
        }
    }

    async fn create_pedido(&mut self, pedido: Pedido) -> Result<Pedido, DomainError> {
        let cliente_id = pedido.cliente().map(|cliente| *cliente.id() as i32);
        let lanche_id = pedido.lanche().map(|lanche| *lanche.id() as i32);
        let acompanhamento_id = pedido
            .acompanhamento()
            .map(|acompanhamento| *acompanhamento.id() as i32);
        let bebida_id = pedido.bebida().map(|bebida| *bebida.id() as i32);

        let status = pedido.status();
        let new_pedido_row = self
            .client
            .query_one(
                CREATE_PEDIDO,
                &[
                    &cliente_id,
                    &lanche_id,
                    &acompanhamento_id,
                    &bebida_id,
                    &pedido.pagamento(),
                    &status,
                ],
            )
            .await;
        match new_pedido_row {
            Ok(row) => {
                let new_pedido = self.pedido_from_proxy(&row).await;
                println!("Novo pedido cadastrado: {:?}", new_pedido);
                Ok(new_pedido)
            }
            Err(_) => Err(DomainError::Invalid("Pedido".to_string())),
        }
    }

    async fn get_pedido_by_id(&self, pedido_id: usize) -> Result<Pedido, DomainError> {
        let pedido_id = pedido_id as i32;
        let pedido_row_result = self
            .client
            .query_opt(QUERY_PEDIDO_BY_ID, &[&pedido_id])
            .await;

        match pedido_row_result {
            Ok(Some(row)) => {
                let pedido = self.pedido_from_proxy(&row).await;
                Ok(pedido)
            }
            Ok(None) => Err(DomainError::NotFound),
            Err(_) => Err(DomainError::Invalid("Pedido".to_string())),
        }
    }

    async fn cadastrar_cliente(
        &mut self,
        pedido_id: usize,
        cliente: Cliente,
    ) -> Result<Pedido, DomainError> {
        let _pedido_id: i32 = pedido_id as i32;
        let _cliente_id = *cliente.id() as i32;

        let updated_pedido = self
            .client
            .query(SET_PEDIDO_CLIENTE, &[&_pedido_id, &_cliente_id])
            .await
            .unwrap();

        let updated_pedido = updated_pedido.get(0);
        match updated_pedido {
            Some(pedido) => Ok(self.pedido_from_proxy(&pedido).await),
            None => Err(DomainError::NotFound),
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
            Some(pedido) => Ok(self.pedido_from_proxy(&pedido).await),
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
            .query(
                SET_PEDIDO_ACOMPANHAMENTO,
                &[&_pedido_id, &_acompanhamento_id],
            )
            .await
            .unwrap();

        let updated_pedido = updated_pedido.get(0);
        match updated_pedido {
            Some(pedido) => Ok(self.pedido_from_proxy(&pedido).await),
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
            Some(pedido) => Ok(self.pedido_from_proxy(&pedido).await),
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
            Some(pedido) => Ok(self.pedido_from_proxy(&pedido).await),
            None => Err(DomainError::NotFound),
        }
    }
}
