use crate::{
    base::domain_error::DomainError,
    entities::pagamento::Pagamento,
    traits::authentication_adapter::AuthenticationAdapter,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[automock]
pub trait PagamentoWebhookAdapter: Send + Sync {
    fn processa_webhook(
        &self,
        data: json,
    ) -> Result<StatusPagamento, DomainError>;

    fn pagamento_status(
        &self,
        pagamento_id: usize
    ) -> Result<StatusPagamento, DomainError>;
}