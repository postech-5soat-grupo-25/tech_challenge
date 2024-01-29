use std::sync::Arc;
use rocket::futures::lock::Mutex;
use chrono::Utc;
use crate::core::{
  application::ports::pagamento_port::PagamentoPort, 
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
      user_repository::UserRepository,
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
    user_repository: Arc<Mutex<dyn UserRepository + Sync + Send>>,
    produto_repository: Arc<Mutex<dyn ProdutoRepository + Sync + Send>>,
    pagammento_adapter: Arc<Mutex<dyn PagamentoPort + Sync + Send>>,
}

impl PedidosEPagamentosUseCase {
    pub fn new(
      pedido_repository: Arc<Mutex<dyn PedidoRepository + Sync + Send>>,
      cliente_repository: Arc<Mutex<dyn ClienteRepository + Sync + Send>>,
      user_repository: Arc<Mutex<dyn UserRepository + Sync + Send>>,
      produto_repository: Arc<Mutex<dyn ProdutoRepository + Sync + Send>>,
      pagammento_adapter: Arc<Mutex<dyn PagamentoPort + Sync + Send>>,
    ) -> Self {
      PedidosEPagamentosUseCase {
        pedido_repository,
        cliente_repository,
        user_repository,
        produto_repository,
        pagammento_adapter,
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


      self.pedido_repository.lock().await.create_pedido(pedido.clone()).await?;

      Ok(pedido)
    }

    pub async fn lista_lanches(&self) -> Result<Vec<Produto>, DomainError> {
      let produtos_repository = self.produto_repository.lock().await;
      produtos_repository.get_produtos_by_categoria(Categoria::Lanche).await
    }

    pub async fn adicionar_lanche_com_personalizacao(&self, pedido_id: usize, lanche_id: usize) -> Result<Pedido, DomainError> {
      let mut pedido_repository = self.pedido_repository.lock().await;
      pedido_repository.cadastrar_lanche(pedido_id, lanche_id).await

    }

    pub async fn lista_acompanhamentos(&self) -> Result<Vec<Produto>, DomainError> {
      let produtos_repository = self.produto_repository.lock().await;
      produtos_repository.get_produtos_by_categoria(Categoria::Acompanhamento).await

    }

    pub async fn adicionar_acompanhamento(&self, pedido_id: usize, acompanhamento_id: usize) -> Result<Pedido, DomainError> {
        let mut pedido_repository = self.pedido_repository.lock().await;
        pedido_repository.cadastrar_acompanhamento(pedido_id, acompanhamento_id).await

    }

    pub async fn lista_bebidas(&self) -> Result<Vec<Produto>, DomainError> {
      let produtos_repository = self.produto_repository.lock().await;
      produtos_repository.get_produtos_by_categoria(Categoria::Bebida).await
    }

    pub async fn adicionar_bebida(&self, pedido_id: usize, bebida_id: usize) -> Result<Pedido, DomainError> {
      let mut pedido_repository = self.pedido_repository.lock().await;
      pedido_repository.cadastrar_bebida(pedido_id, bebida_id).await
    }

    pub async fn realizar_pagamento_do_pedido(&self, pedido_id: usize, pagamento: String) -> Result<Pedido, DomainError> {
      let mut pedido_repository = self.pedido_repository.lock().await;
      pedido_repository.cadastrar_pagamento(pedido_id, pagamento).await
    }
}

unsafe impl Send for PedidosEPagamentosUseCase {}
unsafe impl Sync for PedidosEPagamentosUseCase {}