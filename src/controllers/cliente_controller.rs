use tokio::sync::Mutex;
use std::sync::Arc;

use crate::base::domain_error::DomainError;
use crate::traits::cliente_repository::ClienteRepository;
use crate::use_cases::gerenciamento_de_clientes_use_case::{ClienteUseCase, CreateClienteInput};
use crate::entities::cliente::Cliente;
use crate::entities::cpf::Cpf;

pub struct ClienteController {
    cliente_use_case: ClienteUseCase,
}

impl ClienteController {
    pub fn new(cliente_repository: Arc<Mutex<dyn ClienteRepository + Sync + Send>> ) -> ClienteController {
        let cliente_use_case = ClienteUseCase::new(cliente_repository);
        ClienteController {
            cliente_use_case
        }
    }

    pub async fn lista_clientes(&self) -> Result<Vec<Cliente>, DomainError> {
        self.cliente_use_case.get_clientes().await
    }

    pub async fn busca_cliente_por_cpf(
        &self,
        cpf: Cpf,
    ) -> Result<Cliente, DomainError> {
        self.cliente_use_case.get_cliente_by_cpf(cpf).await
    }

    pub async fn cadastro_cliente(
        &self,
        cliente_input: CreateClienteInput,
    ) -> Result<Cliente, DomainError> {
        self.cliente_use_case.create_cliente(cliente_input).await
    }

}
