use std::sync::Arc;
use rocket::futures::lock::Mutex;
use crate::core::{application::ports::pagamento_port::PagamentoPort, domain::{base::domain_error::DomainError, entities::{cliente::Cliente, pedido::{Pedido, Status}}, repositories::{cliente_repository::ClienteRepository, pedido_repository::PedidoRepository, produto_repository::ProdutoRepository, usuario_repository::UsuarioRepository}}};

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

    pub async fn novo_pedido(&self, cliente: Cliente) -> Result<Pedido, DomainError> {
      let pedido = Pedido::new(
        0,
        cliente,
        None,
        None,
        None,
        String::from(""),
        Status::Pendente,
        String::from(""),
        String::from(""),
      );

      self.pedido_repository.lock().await.create_pedido(pedido.clone()).await?;

      Ok(pedido)
    }
}

unsafe impl Send for PedidosEPagamentosUseCase {}
unsafe impl Sync for PedidosEPagamentosUseCase {}