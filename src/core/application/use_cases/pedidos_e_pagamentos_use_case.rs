use std::sync::Arc;
use rocket::futures::{lock::Mutex, TryFutureExt};
use chrono::Utc;
use crate::core::{
  application::ports::pagamento_port::{
    PagamentoPort,
    ResultadoHandler,
    StatusPagamento
  },
  domain::{
    base::domain_error::DomainError,
    entities::{
      pedido::{Pedido, Status},
      produto::{Produto, Categoria},
    },
    repositories::{
      cliente_repository::ClienteRepository,
      pedido_repository::PedidoRepository,
      produto_repository::ProdutoRepository,
    },
  }
};


pub struct CreatePedidoInput {
  cliente_id: Option<usize>,
}

#[derive(Clone)]
pub struct PedidosEPagamentosUseCase {
    pedido_repository: Arc<Mutex<dyn PedidoRepository + Sync + Send>>,
    cliente_repository: Arc<Mutex<dyn ClienteRepository + Sync + Send>>,
    produto_repository: Arc<Mutex<dyn ProdutoRepository + Sync + Send>>,
    pagamento_adapter: Arc<Mutex<dyn PagamentoPort + Sync + Send>>,
}

impl PedidosEPagamentosUseCase {
    pub fn new(
      pedido_repository: Arc<Mutex<dyn PedidoRepository + Sync + Send>>,
      cliente_repository: Arc<Mutex<dyn ClienteRepository + Sync + Send>>,
      produto_repository: Arc<Mutex<dyn ProdutoRepository + Sync + Send>>,
      pagamento_adapter: Arc<Mutex<dyn PagamentoPort + Sync + Send>>,
    ) -> Self {
      PedidosEPagamentosUseCase {
        pedido_repository,
        cliente_repository,
        produto_repository,
        pagamento_adapter,
      }
    }

    pub async fn seleciona_pedido_por_id(&self, id: usize) -> Result<Pedido, DomainError> {
      let pedido_repository = self.pedido_repository.lock().await;
      pedido_repository.get_pedido_by_id(id).await
  }

    pub async fn novo_pedido(
      &self,
      pedido_input: CreatePedidoInput,
    ) -> Result<Pedido, DomainError> {

      let cliente = match pedido_input.cliente_id {
          Some(id) => {
              let mut cliente_repository = self.cliente_repository.lock().await;
              Some(cliente_repository.get_cliente_by_id(id).await?)
          },
          None => None,
      };

      let pedido = Pedido::new(
        0,
        cliente,
        None,
        None,
        None,
        String::from(""),
        Status::Pendente,
        Utc::now().naive_utc().date().to_string(),
        Utc::now().naive_utc().date().to_string(),
      );


      let pedido = self.pedido_repository.lock().await.create_pedido(pedido.clone()).await?;

      Ok(pedido)
    }

    pub async fn lista_lanches(&self) -> Result<Vec<Produto>, DomainError> {
      let produtos_repository = self.produto_repository.lock().await;
      produtos_repository.get_produtos_by_categoria(Categoria::Lanche).await
    }

    pub async fn adicionar_lanche_com_personalizacao(&self, pedido_id: usize, lanche_id: usize) -> Result<Pedido, DomainError> {
      let mut pedido_repository = self.pedido_repository.lock().await;
      let mut produto_repository = self.produto_repository.lock().await;
      let lanche = produto_repository.get_produto_by_id(lanche_id).await?;
      pedido_repository.cadastrar_lanche(pedido_id, lanche).await
    }

    pub async fn lista_acompanhamentos(&self) -> Result<Vec<Produto>, DomainError> {
      let produtos_repository = self.produto_repository.lock().await;
      produtos_repository.get_produtos_by_categoria(Categoria::Acompanhamento).await
    }

    pub async fn adicionar_acompanhamento(&self, pedido_id: usize, acompanhamento_id: usize) -> Result<Pedido, DomainError> {
        let mut pedido_repository = self.pedido_repository.lock().await;
        let mut produto_repository = self.produto_repository.lock().await;
        let acompanhamento = produto_repository.get_produto_by_id(acompanhamento_id).await?;
        pedido_repository.cadastrar_acompanhamento(pedido_id, acompanhamento).await

    }

    pub async fn lista_bebidas(&self) -> Result<Vec<Produto>, DomainError> {
      let produtos_repository = self.produto_repository.lock().await;
      produtos_repository.get_produtos_by_categoria(Categoria::Bebida).await
    }

    pub async fn adicionar_bebida(&self, pedido_id: usize, bebida_id: usize) -> Result<Pedido, DomainError> {
      let mut pedido_repository = self.pedido_repository.lock().await;
      let mut produto_repository = self.produto_repository.lock().await;
      let bebida = produto_repository.get_produto_by_id(bebida_id).await?;
      pedido_repository.cadastrar_bebida(pedido_id, bebida).await
    }

    pub async fn realizar_pagamento_do_pedido(&self, pedido_id: usize) -> Result<Pedido, DomainError> {
      let mut pedido_repository = self.pedido_repository.lock().await;
      let pagamento_adapter = self.pagamento_adapter.lock().await;

      let pedido = pedido_repository.get_pedido_by_id(pedido_id).await?;
      let total_pedido = pedido.get_total_valor_pedido();

      let resultado_handler: ResultadoHandler = Box::new(move |status, pedido_id| {
        if status == StatusPagamento::Successo {
          pedido_repository.atualiza_status(pedido_id, Status::Recebido).unwrap();
        }
      });

      let pagamento_id: usize = pagamento_adapter.processa_pagamento(pedido_id, total_pedido, resultado_handler).await?;
      pedido_repository.cadastrar_pagamento(pedido_id, pagamento_id).await
    }
}

unsafe impl Send for PedidosEPagamentosUseCase {}
unsafe impl Sync for PedidosEPagamentosUseCase {}