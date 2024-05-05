use crate::entities::cpf;
use crate::{
    base::domain_error::DomainError, entities::cliente::Cliente, entities::cpf::Cpf,
    traits::cliente_gateway::ClienteGateway,
};

use aws_config::meta::region::RegionProviderChain;
use aws_config::BehaviorVersion;
use aws_sdk_cognitoidentityprovider::error::DisplayErrorContext;
use aws_sdk_cognitoidentityprovider::{config::Region, meta::PKG_VERSION, Client};
use chrono::Utc;

static USER_POOL_ID: &str = "us-east-1_9KnwgXBoZ";

fn extract_digits(input: &str) -> String {
    input.chars().filter(|c| c.is_digit(10)).collect()
}

async fn get_client() -> Client {
    let region_provider = RegionProviderChain::default_provider().or_else(Region::new("us-west-2"));

    let shared_config = aws_config::defaults(BehaviorVersion::latest())
        .region(region_provider)
        .load()
        .await;
    let client = Client::new(&shared_config);

    client.set_user_pool_mfa_config().user_pool_id(USER_POOL_ID);

    client
}
pub struct CognitoClienteGateway {
    client: Client,
}

impl CognitoClienteGateway {
    pub async fn new() -> Self {
        let cognito_client = get_client().await;
        let gateway = CognitoClienteGateway {
            client: cognito_client,
        };
        gateway
    }
}

#[async_trait]
impl ClienteGateway for CognitoClienteGateway {
    async fn get_clientes(&self) -> Result<Vec<Cliente>, DomainError> {
        Ok(vec![])
    }

    async fn get_cliente_by_cpf(&self, cpf: Cpf) -> Result<Cliente, DomainError> {
        let cpf_digits = extract_digits(&cpf.clone().0);
        let cliente = self
            .client
            .admin_get_user()
            .username(cpf_digits.clone())
            .send()
            .await;

        println!("{:?}", cliente);

        match cliente {
            Ok(cliente) => {
                let user_attributes = cliente.user_attributes.unwrap();
                let user_name = user_attributes
                    .iter()
                    .find(|attr| attr.name == "name")
                    .unwrap()
                    .value
                    .clone()
                    .unwrap_or("No name".to_string());

                let user_email = user_attributes
                    .iter()
                    .find(|attr| attr.name == "email")
                    .unwrap()
                    .value
                    .clone()
                    .unwrap_or("No email".to_string());

                Ok(Cliente::new(
                    cpf_digits.clone().parse::<usize>().unwrap(),
                    user_name,
                    user_email,
                    cpf.clone(),
                    Utc::now().format("%Y-%m-%d %H:%M:%S%.3f%z").to_string(),
                    Utc::now().format("%Y-%m-%d %H:%M:%S%.3f%z").to_string(),
                ))
            }
            Err(e) => {
                let error = e.to_string();
                Err(DomainError::Invalid("Cliente não encontrado".to_string()))
            }
        }
    }

    async fn get_cliente_by_id(&self, id: usize) -> Result<Cliente, DomainError> {
        Ok(Cliente::new(
            1,
            "Fulano da Silva".to_string(),
            "fulano.silva@exemplo.com".to_string(),
            Cpf::new("123.456.789-09".to_string()).unwrap(),
            Utc::now().format("%Y-%m-%d %H:%M:%S%.3f%z").to_string(),
            Utc::now().format("%Y-%m-%d %H:%M:%S%.3f%z").to_string(),
        ))
    }

    async fn create_cliente(&mut self, cliente: Cliente) -> Result<Cliente, DomainError> {
        Ok(cliente)
    }

    async fn delete_cliente(&mut self, cpf: Cpf) -> Result<(), DomainError> {
        Ok(())
    }
}
