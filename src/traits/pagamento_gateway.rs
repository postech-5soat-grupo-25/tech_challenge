use mockall::*;

use crate::base::domain_error::DomainError;
use crate::entities::pagamento::Pagamento;

#[automock]
#[async_trait]
pub trait PagamentoGateway {
    async fn create_pagamento(&mut self, pagamento: Pagamento) -> Result<Pagamento, DomainError>;

    // async fn get_pagamento_by_id(&self, id: usize) -> Result<Pagamento, DomainError>;

    // async fn update_pagamento(&mut self, pagamento: Pagamento) -> Result<Pagamento, DomainError>;
}
