use std::sync::Arc;

use rocket::futures::lock::Mutex;
use schemars::JsonSchema;
use serde::Deserialize;

use crate::core::domain::base::domain_error::DomainError;
use crate::core::domain::entities::pedido::Pedido;
use crate::core::domain::repositories::order_repository::OrderRepository;
use crate::core::domain::value_objects::cpf::Cpf;
use crate::core::domain::value_objects::endereco::Endereco;

// TODO: onde faço a validação dos dados? [ ] UseCase [ ] Repository [ ] Entity
#[derive(Clone, Debug, Deserialize, JsonSchema)]
pub struct CreateOrderInput {
    nome: String,
    email: String,
    senha: String,
    endereco: String,
    cpf: String,
}

#[derive(Clone, Debug, Deserialize, JsonSchema)]
pub struct UpdateOrderInput {
    nome: Option<String>,
    email: Option<String>,
    endereco: Option<String>,
}

#[derive(Clone)]
pub struct OrderUseCase {
    order_repository: Arc<Mutex<dyn OrderRepository + Sync + Send>>,
}

impl OrderUseCase {
    pub fn new(order_repository: Arc<Mutex<dyn OrderRepository + Sync + Send>>) -> Self {
        OrderUseCase { order_repository }
    }

    pub async fn get_orders(&self) -> Result<Vec<Pedido>, DomainError> {
        let order_repository = self.order_repository.lock().await;
        order_repository.get_orders().await
    }

    pub async fn get_order_by_id(&self, id: usize) -> Result<Pedido, DomainError> {
        let order_repository = self.order_repository.lock().await;
        order_repository.get_order_by_id(id).await
    }

    pub async fn get_order_by_cpf(&self, user_id: usize) -> Result<Pedido, DomainError> {
        let order_repository = self.order_repository.lock().await;
        order_repository.get_order_by_user(user_id).await
    }

    pub async fn create_order(&self, order: CreateOrderInput) -> Result<Pedido, DomainError> {
        let mut order_repository = self.order_repository.lock().await;
        let new_id = 0;
        let valid_cpf = Cpf::new(order.cpf.clone())?;
        let valid_endereco = Endereco::new(order.endereco.clone());

        let order = order_repository
            .create_order(Pedido::new(new_id, order.nome, order.email, order.senha, valid_cpf, valid_endereco))
            .await?;

        Ok(order.clone())
    }

    pub async fn update_order_info(&self, id: usize, fields_to_update: UpdateOrderInput) -> Result<Pedido, DomainError> {
        let mut order_repository = self.order_repository.lock().await;
        let mut order = order_repository.get_order_by_id(id).await?;
        if let Some(nome) = fields_to_update.nome {
            order.set_nome(nome);
        }
        if let Some(email) = fields_to_update.email {
            order.set_email(email);
        }
        if let Some(endereco) = fields_to_update.endereco {
            let valid_endereco = Endereco::new(endereco);
            order.set_endereco(valid_endereco);
        }
        order_repository.update_order(order).await
    }

    pub async fn delete_order(&self, id: usize) -> Result<(), DomainError> {
        let mut order_repository = self.order_repository.lock().await;
        order_repository.delete_order(id).await?;
        Ok(())
    }
}

unsafe impl Send for OrderUseCase {}
unsafe impl Sync for OrderUseCase {}
