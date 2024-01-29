use std::sync::Arc;

use rocket::futures::lock::Mutex;

use crate::core::domain::base::domain_error::DomainError;
use crate::core::domain::entities::pedido::Pedido;

use crate::core::domain::repositories::pedido_repository::PedidoRepository;

#[derive(Clone)]
pub struct PreparacaoeEntregaUseCase {
    pedido_repository: Arc<Mutex<dyn PedidoRepository + Sync + Send>>,
}

impl PreparacaoeEntregaUseCase {
    pub fn new(pedido_repository: Arc<Mutex<dyn PedidoRepository + Sync + Send>>) -> Self {
        PreparacaoeEntregaUseCase { pedido_repository }
    }

    pub async fn get_pedidos_novos(&self) -> Result<Vec<Pedido>, DomainError> {
        let pedido_repository = self.pedido_repository.lock().await;
        pedido_repository.get_pedidos_novos().await
    }

    pub async fn atualiza_status(&self,  id: usize, status : String) -> Result<Pedido, DomainError> {
        let mut pedido_repository = self.pedido_repository.lock().await;
        pedido_repository.atualiza_status(id, status).await
    }
}

unsafe impl Send for PreparacaoeEntregaUseCase {}
unsafe impl Sync for PreparacaoeEntregaUseCase {}