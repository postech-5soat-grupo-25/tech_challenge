use rocket::tokio::time::{sleep, Duration};

use crate::core::domain::base::domain_error::DomainError;
use crate::core::domain::entities::cliente::Cliente;
use crate::core::domain::repositories::cliente_repository::ClienteRepository;
use crate::core::domain::value_objects::cpf::Cpf;

#[derive(Clone)]
pub struct InMemoryClienteRepository {
    _clientes: Vec<Cliente>,
}

impl InMemoryClienteRepository {
    pub fn new() -> Self {
        let cliente = Cliente::new(
            1,
            "Fulano da Silva".to_string(),
            "fulano.silva@exemplo.com".to_string(),
            Cpf::new("000.000.000-00".to_string()).unwrap(),
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
