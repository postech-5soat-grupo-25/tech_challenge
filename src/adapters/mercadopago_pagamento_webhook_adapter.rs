use crate::traits::pagamento_webhook_adapter::PagamentoWebhookAdapter;
use crate::entities::pagamento::Pagamento;
use crate::base::domain_error::DomainError;

use serde_json::Value;

#[derive(Clone)]
pub struct MercadoPagoPagamentoWebhookAdapter {
}

impl MercadoPagoPagamentoWebhookAdapter {
    pub fn new() -> Self {
        MercadoPagoPagamentoWebhookAdapter {}
    }
}

#[async_trait]
impl PagamentoWebhookAdapter for MercadoPagoPagamentoWebhookAdapter {
    fn processa_webhook(
        &self,
        data: Value,
        mut pagamento: Pagamento,
    ) -> Pagamento {
        if let Some(obj) = data.as_object() {
            if let Some(action) = obj.get("action") {
                // Check if the action attribute is a string and if it equals "payment.approved"
                if let Some(action_str) = action.as_str() {
                    if action_str == "payment.approved" {
                        pagamento.set_estado(String::from("aprovado"));
                    }
                }
            }
            if let Some(id) = obj.get("id") {
                // Check if the action attribute is a string and if it equals "payment.approved"
                if let Some(id_str) = id.as_str() {
                    pagamento.set_referencia(id_str.to_string());
                }
            }
        }
        pagamento
    }
}