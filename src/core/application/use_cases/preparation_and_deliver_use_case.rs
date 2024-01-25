use std::sync::Arc;

use rocket::futures::lock::Mutex;
use rocket::http::Status;
use schemars::JsonSchema;
use serde::Deserialize;

use crate::core::domain::base::domain_error::DomainError;
use crate::core::domain::entities::pedido::Pedido;

use crate::core::domain::repositories::pedido_repository::PedidoRepository;

#[derive(Clone)]
pub struct PreparationAndDeliverUseCase {
    pedido_repository: Arc<Mutex<dyn PedidoRepository + Sync + Send>>,
}

impl PreparationAndDeliverUseCase {
    pub fn new(pedido_repository: Arc<Mutex<dyn PedidoRepository + Sync + Send>>) -> Self {
        PreparationAndDeliverUseCase { pedido_repository }
    }

    pub async fn get_pedidos_novos(&self) -> Result<Vec<Pedido>, DomainError> {
        let pedido_repository = self.pedido_repository.lock().await;
        pedido_repository.get_pedidos_novos().await
    }

    pub async fn set_pedido_em_preparacao(&self,  id: usize) -> Result<Pedido, DomainError> {
        let mut pedido_repository = self.pedido_repository.lock().await;
        pedido_repository.set_pedido_em_preparacao(id).await
    }

    pub async fn set_pedido_pronto(&self,  id: usize) -> Result<Pedido, DomainError> {
        let mut pedido_repository = self.pedido_repository.lock().await;
        pedido_repository.set_pedido_pronto(id).await
    }

    pub async fn set_pedido_finalizado(&self,  id: usize) -> Result<Pedido, DomainError> {
        let mut pedido_repository = self.pedido_repository.lock().await;
        pedido_repository.set_pedido_finalizado(id).await
    }

    pub async fn set_pedido_cancelado(&self,  id: usize) -> Result<Pedido, DomainError> {
        let mut pedido_repository = self.pedido_repository.lock().await;
        pedido_repository.set_pedido_cancelado(id).await
    }
}

unsafe impl Send for PreparationAndDeliverUseCase {}
unsafe impl Sync for PreparationAndDeliverUseCase {}