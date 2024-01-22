use std::sync::Arc;

use rocket::futures::lock::Mutex;
use schemars::JsonSchema;
use serde::Deserialize;

use crate::core::domain::base::domain_error::DomainError;
use crate::core::domain::entities::pedido::{ Pedido, Status };
use crate::core::domain::repositories::order_repository::OrderRepository;


impl PreparationAndDeliverUseCase {
    pub fn new(order_repository: Arc<Mutex<dyn OrderRepository + Sync + Send>>) -> Self {
        PreparationAndDeliverUseCase { order_repository }
    }

    pub async fn get_new_orders(&self) -> Result<Vec<Pedido>, DomainError> {
        let order_repository = self.order_repository.lock().await;
        order_repository.get_new_orders().await
    }

    pub async fn set_order_to_ready(&self,  id: usize) -> Result<Json<Usuario>, Status> {
        let mut order_repository = self.order_repository.lock().await;
        let mut order = order_repository.get_order_by_id(id).await?;
        order.set_status(Status::Pronto)
        order_repository.update_order(order).await
    }

    pub async fn set_order_to_finished(&self,  id: usize) -> Result<Json<Usuario>, Status> {
        let mut order_repository = self.order_repository.lock().await;
        let mut order = order_repository.get_order_by_id(id).await?;
        order.set_status(Status::Finalizado)
        order_repository.update_order(order).await
    }

    pub async fn set_order_to_canceled(&self,  id: usize) -> Result<Json<Usuario>, Status> {
        let mut order_repository = self.order_repository.lock().await;
        let mut order = order_repository.get_order_by_id(id).await?;
        order.set_status(Status::Cancelado)
        order_repository.update_order(order).await
    }
}

unsafe impl Send for PreparationAndDeliverUseCase {}
unsafe impl Sync for PreparationAndDeliverUseCase {}