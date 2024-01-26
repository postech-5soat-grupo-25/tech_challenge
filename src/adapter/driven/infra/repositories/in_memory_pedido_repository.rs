use chrono::Utc;
use rocket::tokio::time::{sleep, Duration};

use crate::core::domain::base::domain_error::DomainError;
use crate::core::domain::entities::pedido::{Pedido, Status};
use crate::core::domain::entities::cliente::Cliente;
use crate::core::domain::entities::produto::{Produto,Categoria};

use crate::core::domain::value_objects::cpf::Cpf;
use crate::core::domain::value_objects::ingredientes::Ingredientes;
use crate::core::domain::repositories::pedido_repository::PedidoRepository;

#[derive(Clone)]
pub struct InMemoryPedidoRepository {
    _pedidos: Vec<Pedido>,
}

impl InMemoryPedidoRepository {
    pub fn new() -> Self {
        let current_date = Utc::now().naive_utc().format("%Y-%m-%d").to_string();

        let cliente = Cliente::new(
            1,
            "Fulano da Silva".to_string(),
            "fulano.silva@exemplo.com".to_string(),
            Cpf::new("123.456.789-09".to_string()).unwrap(),
            "2024-01-17".to_string(),
            "2024-01-17".to_string(),
        );

        let lanche = Produto::new(
            1,
            "Cheeseburger".to_string(),
            "cheeseburger.png".to_string(),
            "O clássico pão, carne e queijo!".to_string(),
            Categoria::Lanche,
            9.99,
            Ingredientes::new(vec!["Pão".to_string(), "Hambúrguer".to_string(), "Queijo".to_string()]).unwrap(),
            "2024-01-17".to_string(),
            "2024-01-17".to_string(),
        );
        
        let pedido = Pedido::new(
            1,
            Some(cliente),
            Some(lanche),
            None,
            None,
            "mercadopago".to_string(),
            Status::Recebido,
            current_date.clone(),
            current_date,
        );

        println!("Usando repositório em memória!");

        InMemoryPedidoRepository {
            _pedidos: vec![pedido],
        }
    }
}

async fn get_status_by_string(status : String) -> Status {
    let mut status_enum : Status = Status::Recebido;
    match status.as_str() {
        "recebido" => status_enum = Status::Recebido,
        "em_preparacao" => status_enum = Status::EmPreparacao,
        "pronto" => status_enum = Status::Pronto,
        "finalizado" => status_enum = Status::Finalizado,
        "set_pedido_cancelado" => status_enum = Status::Cancelado,
        &_ => status_enum = Status::Invalido,
    };
    return status_enum.clone();
}


#[async_trait]
impl PedidoRepository for InMemoryPedidoRepository {
    async fn get_pedidos_novos(&self) -> Result<Vec<Pedido>, DomainError> {
        let mut pedidos : Vec<Pedido> = Vec::new();
        for pedido in &self._pedidos {
            if (*pedido.status() == Status::Recebido){
                pedidos.push(pedido.clone());
            }
        }
        sleep(Duration::from_secs(1)).await;
        Ok(pedidos)
    }


    async fn atualizar_status_pedido(&mut self, id: usize, status :String) -> Result<Pedido, DomainError> {
        let pedidos = &mut self._pedidos;
        let status_enum = Status::from_string(status);
        if (status_enum == Status::Invalido){
            return Err::<Pedido, _>(DomainError::Invalid("status".to_string()));
        }
        for pedido in pedidos.iter_mut() {
            if *pedido.id() == id {
                pedido.set_status(status_enum.clone());
                return Ok(pedido.clone());
            }
        }
        
        Err(DomainError::NotFound)
    }

}

unsafe impl Sync for InMemoryPedidoRepository {}
unsafe impl Send for InMemoryPedidoRepository {}
