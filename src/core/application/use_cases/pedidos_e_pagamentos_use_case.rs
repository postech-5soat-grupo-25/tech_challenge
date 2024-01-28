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
      usuario_repository::UsuarioRepository,
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
    usuario_repository: Arc<Mutex<dyn UsuarioRepository + Sync + Send>>,
    produto_repository: Arc<Mutex<dyn ProdutoRepository + Sync + Send>>,
    pagammento_adapter: Arc<Mutex<dyn PagamentoPort + Sync + Send>>,
}

impl PedidosEPagamentosUseCase {
    pub fn new(
      pedido_repository: Arc<Mutex<dyn PedidoRepository + Sync + Send>>,
      cliente_repository: Arc<Mutex<dyn ClienteRepository + Sync + Send>>,
      usuario_repository: Arc<Mutex<dyn UsuarioRepository + Sync + Send>>,
      produto_repository: Arc<Mutex<dyn ProdutoRepository + Sync + Send>>,
      pagammento_adapter: Arc<Mutex<dyn PagamentoPort + Sync + Send>>,
    ) -> Self {
      PedidosEPagamentosUseCase {
        pedido_repository,
        cliente_repository,
        usuario_repository,
        produto_repository,
        pagammento_adapter,
      }
    }

    async fn verify_order_components(&self, produto_id: Option<usize>) -> Result<Option<Produto>, DomainError> {
        match produto_id {
            Some(id) => {
                let produto_repository = self.produto_repository.lock().await;
                Ok(Some(produto_repository.get_produto_by_id(id).await?))
            },
            None => Ok(None),
        }
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

    pub async fn adicionar_lanche_com_personalizacao(&self, pedido_id: usize, lanche: Produto) -> Result<Pedido, DomainError> {
      let mut pedido_repository = self.pedido_repository.lock().await;

      let mut produtos_repository = self.produto_repository.lock().await;
      let produto = produtos_repository.create_produto(lanche).await?;

      pedido_repository.cadastrar_lanche(pedido_id, produto).await

    }

    pub async fn lista_acompanhamentos(&self) -> Result<Vec<Produto>, DomainError> {
      let produtos_repository = self.produto_repository.lock().await;
      produtos_repository.get_produtos_by_categoria(Categoria::Acompanhamento).await

    }

    pub async fn adicionar_acompanhamento(&self, pedido_id: usize, acompanhamento: Produto) -> Result<Pedido, DomainError> {
        let mut pedido_repository = self.pedido_repository.lock().await;
        pedido_repository.cadastrar_acompanhamento(pedido_id, acompanhamento).await

    }

    pub async fn lista_bebidas(&self) -> Result<Vec<Produto>, DomainError> {
      let produtos_repository = self.produto_repository.lock().await;
      produtos_repository.get_produtos_by_categoria(Categoria::Bebida).await
    }

    pub async fn adicionar_bebida(&self, pedido_id: usize, bebida: Produto) -> Result<Pedido, DomainError> {
      let mut pedido_repository = self.pedido_repository.lock().await;
      pedido_repository.cadastrar_bebida(pedido_id, bebida).await
    }

    pub async fn realizar_pagamento_do_pedido(&self, pedido_id: usize, pagamento: String) -> Result<Pedido, DomainError> {
      let mut pedido_repository = self.pedido_repository.lock().await;
      pedido_repository.cadastrar_pagamento(pedido_id, pagamento).await
    }
}

unsafe impl Send for PedidosEPagamentosUseCase {}
unsafe impl Sync for PedidosEPagamentosUseCase {}