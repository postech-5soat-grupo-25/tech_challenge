use std::sync::Arc;

use rocket::futures::lock::Mutex;

use crate::core::domain::base::domain_error::DomainError;
use crate::core::domain::entities::pedido::{Pedido, Status};

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

    pub async fn atualiza_status(&self,  id: usize, status : Status) -> Result<Pedido, DomainError> {
        let mut pedido_repository = self.pedido_repository.lock().await;
        pedido_repository.atualiza_status(id, status).await
    }
}

unsafe impl Send for PreparacaoeEntregaUseCase {}
unsafe impl Sync for PreparacaoeEntregaUseCase {}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;
    use crate::core::domain::entities::pedido::Pedido;
    use crate::core::domain::repositories::pedido_repository::MockPedidoRepository;
    use rocket::futures::lock::Mutex;
    use std::sync::Arc;
    use mockall::predicate::*;

    #[tokio::test]
    async fn test_get_pedidos_novos() {
        let mut mock = MockPedidoRepository::new();

        let returned_pedido = Pedido::new(
            1,
            None,
            None,
            None,
            None,
            "id_pagamento".to_string(),
            Status::Recebido,
            "2021-10-10".to_string(),
            "2021-10-10".to_string()
        );

        let expected_pedido = returned_pedido.clone();

        mock.expect_get_pedidos_novos()
            .times(1)
            .returning(move || Ok(vec![returned_pedido.clone()]));

        let use_case = PreparacaoeEntregaUseCase::new(Arc::new(Mutex::new(mock)));
        let result = use_case.get_pedidos_novos().await;
        assert_eq!(result.unwrap()[0].id(), expected_pedido.id());
    }

    #[tokio::test]
    async fn test_atualiza_status() {
        let mut mock = MockPedidoRepository::new();

        let returned_pedido = Pedido::new(
            1,
            None,
            None,
            None,
            None,
            "id_pagamento".to_string(),
            Status::EmPreparacao,
            "2021-10-10".to_string(),
            "2021-10-10".to_string()
        );

        mock.expect_atualiza_status()
            .times(1)
            .with(eq(1), eq(Status::EmPreparacao))
            .returning(move |_, _| Ok(returned_pedido.clone()));

        let use_case = PreparacaoeEntregaUseCase::new(Arc::new(Mutex::new(mock)));
        let result = use_case.atualiza_status(1, Status::EmPreparacao).await;
        assert_eq!(result.unwrap().status().to_owned(), Status::EmPreparacao);
    }
}
