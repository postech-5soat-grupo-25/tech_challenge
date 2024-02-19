use chrono::Utc;
use rocket::tokio::time::{sleep, Duration};

use crate::base::domain_error::DomainError;
use crate::entities::cliente::Cliente;
use crate::traits::cliente_repository::ClienteRepository;
use crate::entities::cpf::Cpf;

#[derive(Clone)]
pub struct InMemoryClienteRepository {
    _clientes: Vec<Cliente>,
}

impl InMemoryClienteRepository {
    pub fn new() -> Self {
        let _id = 0;
        let _now = Utc::now().format("%Y-%m-%d %H:%M:%S%.3f%z").to_string();
        let cliente = Cliente::new(
            _id,
            "Fulano da Silva".to_string(),
            "fulano.silva@exemplo.com".to_string(),
            Cpf::new("000.000.000-00".to_string()).unwrap(),
            _now.clone(),
            _now,
        );

        println!("Usando repositório em memória!");

        InMemoryClienteRepository {
            _clientes: vec![cliente],
        }
    }
}

#[async_trait]
impl ClienteRepository for InMemoryClienteRepository {
    async fn get_clientes(&self) -> Result<Vec<Cliente>, DomainError> {
        let clientes = self._clientes.clone();
        sleep(Duration::from_secs(1)).await;
        Ok(clientes)
    }

    async fn get_cliente_by_cpf(&self, cpf: Cpf) -> Result<Cliente, DomainError> {
        sleep(Duration::from_secs(1)).await;
        for cliente in &self._clientes {
            if cliente.cpf().to_owned() == cpf {
                return Ok(cliente.clone());
            }
        }
        Err(DomainError::NotFound)
    }

    async fn get_cliente_by_id(&self, id: usize) -> Result<Cliente, DomainError> {
        let id = id as i32;
        sleep(Duration::from_secs(1)).await;
        for cliente in &self._clientes {
            if cliente.id().to_owned() == id as usize {
                return Ok(cliente.clone());
            }
        }
        Err(DomainError::NotFound)
    }

    async fn create_cliente(&mut self, cliente: Cliente) -> Result<Cliente, DomainError> {
        sleep(Duration::from_secs(1)).await;
        let existing_cliente = self.get_cliente_by_cpf(cliente.cpf().to_owned()).await;

        if existing_cliente.is_ok() {
            return Err(DomainError::AlreadyExists);
        }

        let mut cliente_list = self._clientes.clone();
        cliente_list.push(cliente.clone());

        self._clientes = cliente_list;

        Ok(cliente.clone())
    }

    async fn delete_cliente(&mut self, cpf: Cpf) -> Result<(), DomainError> {
        let cliente_list = &mut self._clientes;
        for (index, cliente) in cliente_list.iter_mut().enumerate() {
            if cliente.cpf().to_owned() == cpf {
                cliente_list.remove(index);
                return Ok(());
            }
        }
        Err(DomainError::NotFound)
    }
}

unsafe impl Sync for InMemoryClienteRepository {}
unsafe impl Send for InMemoryClienteRepository {}
