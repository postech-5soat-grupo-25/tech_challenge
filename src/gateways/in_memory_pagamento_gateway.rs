use chrono::Utc;
use tokio::time::{sleep, Duration};

use crate::base::domain_error::DomainError;
use crate::entities::pagamento::Pagamento;
use crate::entities::cliente::Cliente;
use crate::entities::produto::{Produto,Categoria};

use crate::traits::pagamento_gateway::PagamentoGateway;

#[derive(Clone)]
pub struct InMemoryPagamentoRepository {
    _pagamentos: Vec<Pagamento>,
}

impl InMemoryPagamentoRepository {
    pub fn new() -> Self {
        let current_date = Utc::now().naive_utc().format("%Y-%m-%d").to_string();



        let pagamento = Pagamento::new(
            1,
            1,
            "pago".to_string(),
            "MercadoPago".to_string(),
            "1234".to_string(),
            current_date,
        );

        println!("Usando repositório em memória!");

        InMemoryPagamentoRepository {
            _pagamentos: vec![pagamento],
        }
    }
}


#[async_trait]
impl PagamentoGateway for InMemoryPagamentoRepository {
    async fn create_pagamento(&mut self, pagamento: Pagamento) -> Result<Pagamento, DomainError> {
        let pagamentos = &mut self._pagamentos;
        pagamentos.push(pagamento.clone());
        Ok(pagamento)
    }

    async fn get_pagamento_by_id_pedido(&mut self, id_pagamento: usize) -> Result<Pagamento, DomainError> {
        let id = id_pagamento as i32;
        let pagamentos = &mut self._pagamentos;
        sleep(Duration::from_secs(1)).await;
        for pagamento in &self._pagamentos {
            if pagamento.id_pedido().to_owned() == id as usize {
                return Ok(pagamento.clone());
            }
        }
        Err(DomainError::NotFound)
    }
    
    // async fn atualiza_status(&mut self, id: usize, status: Status) -> Result<Pagamento, DomainError> {
    //     let pagamentos = &mut self._pagamentos;
    //     if (status == Status::Invalido){
    //         return Err::<Pagamento, _>(DomainError::Invalid("status".to_string()));
    //     }
    //     for pagamento in pagamentos.iter_mut() {
    //         if *pagamento.id() == id {
    //             pagamento.set_status(status.clone());
    //             return Ok(pagamento.clone());
    //         }
    //     }
    //     Err(DomainError::NotFound)
    // }

    // async fn get_pagamento_by_id(&self, pagamento_id: usize) -> Result<Pagamento, DomainError> {
    //     let pagamentos = &self._pagamentos;
    //     for pagamento in pagamentos.iter() {
    //         if *pagamento.id() == pagamento_id {
    //             return Ok(pagamento.clone());
    //         }
    //     }
    //     Err(DomainError::NotFound)
    // }
}

unsafe impl Sync for InMemoryPagamentoRepository {}
unsafe impl Send for InMemoryPagamentoRepository {}
