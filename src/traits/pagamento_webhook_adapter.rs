use crate::{
    base::domain_error::DomainError,
    entities::pagamento::Pagamento,
    traits::authentication_adapter::AuthenticationAdapter,
};


use serde::{Deserialize, Serialize};
use serde_json::Value;

pub trait PagamentoWebhookAdapter: Send + Sync {
    fn processa_webhook(
        &self,
        data: Value,
        pagamento: Pagamento,
    ) -> Pagamento;
}