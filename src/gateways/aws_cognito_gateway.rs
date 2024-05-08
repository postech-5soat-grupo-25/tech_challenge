use aws_config::from_env;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_cognitoidentityprovider::operation::list_users;
use aws_sdk_cognitoidentityprovider::{config::Region, meta::PKG_VERSION, Client};
use chrono::Utc;

use crate::{
    base::domain_error::DomainError, entities::cliente::Cliente, entities::cpf::Cpf,
    traits::cliente_gateway::ClienteGateway,
};

fn option_to_string(option: Option<&str>) -> String {
    match option {
        Some(value) => value.to_string(), // Convert &str to String
        None => String::new(),            // Return an empty String for None
    }
}

pub struct AwsCognitoRepository {
    client: Client,
    user_pool_id: String,
}

impl AwsCognitoRepository {
    pub async fn new(user_pool_id: String) -> Self {
        // TODO config the region correctly

        let region_provider = RegionProviderChain::default_provider(); // Default region provider chain

        let config = aws_config::from_env().region(region_provider).load().await;
        let client = Client::new(&config);
        AwsCognitoRepository {
            client,
            user_pool_id,
        }
    }
}

#[async_trait]
impl ClienteGateway for AwsCognitoRepository {
    async fn get_clientes(&self) -> Result<Vec<Cliente>, DomainError> {
        let response = self
            .client
            .list_users()
            .user_pool_id(&self.user_pool_id)
            .send()
            .await;

        let mut clientes: Vec<Cliente> = Vec::new();

        match response {
            Ok(response) => {
                println!("Estamos GIGANTESCOS");
                let users = response.users();

                for user in users {
                    let mut id = String::new();
                    let mut nome = String::new();
                    let mut email = String::new();
                    let mut cpf_string = String::new();
                    let mut data_criacao = String::new();
                    let mut data_atualizacao = String::new();

                    for attr in user.attributes() {
                        match attr.name() {
                            "id" => id =option_to_string(attr.value()),
                            "nome" => nome = option_to_string(attr.value()),
                            "email" => email = option_to_string(attr.value()),
                            "cpf" => cpf_string = option_to_string(attr.value()),
                            "data_criacao" => data_criacao = option_to_string(attr.value()),
                            "data_atualizacao" => data_atualizacao = option_to_string(attr.value()),
                            _ => {}
                        }
                    };

                    let cpf = Cpf::new(cpf_string.to_string());

                    match cpf {
                        Ok(cpf) => {
                            let cliente = Cliente::new(
                                user.username().unwrap_or_default().parse().unwrap_or(0),
                                nome,
                                email,
                                cpf,
                                data_criacao,
                                data_atualizacao,
                            );
        
                            clientes.push(cliente);
                        },
                        Err(err) => println!("Invalid CPF for user: {}", nome),
                    };
                }
                Ok(clientes)
            }
            Err(err) => {
                println!("Error during aws cognito request: {}", err);
                Err(DomainError::NotFound)
            }

        }
    }

    async fn get_cliente_by_cpf(&self, cpf: Cpf) -> Result<Cliente, DomainError> {
        // let request = self.client
        //     .list_users()
        //     .user_pool_id(&self.user_pool_id)
        //     .filter(format!("cpf = \"{}\"", cpf))
        //     .build();

        // let response = self.client.list_users(request).await?;
        // if let Some(user) = response.users().first() {
        //     let mut email = String::new();
        //     if let Some(attributes) = user.attributes() {
        //         for attr in attributes {
        //             if attr.name() == "email" {
        //                 email = attr.value().to_owned();
        //             }
        //         }
        //     }

        //     let cliente = Cliente {
        //         id: user.username().unwrap_or("").to_owned(),
        //         cpf: cpf.to_owned(),
        //         email,
        //     };

        //     Ok(cliente)
        // } else {
        //     Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "Cliente not found")))
        // }
        let _now = Utc::now().format("%Y-%m-%d %H:%M:%S%.3f%z").to_string();
        let cliente = Cliente::new(
            1,
            "Fulano da Silva".to_string(),
            "".to_string(),
            Cpf::new("123.456.789-09".to_string()).unwrap(),
            _now.clone(),
            _now,
        );

        Ok(cliente)
    }

    async fn get_cliente_by_id(&self, id: usize) -> Result<Cliente, DomainError> {
        // let request = self.client
        //     .admin_get_user()
        //     .user_pool_id(&self.user_pool_id)
        //     .username(id)
        //     .build();

        // let response = self.client.admin_get_user(request).await?;
        // let mut cpf = String::new();
        // let mut email = String::new();

        // for attr in response.user_attributes().unwrap_or_default() {
        //     if attr.name() == "cpf" {
        //         cpf = attr.value().to_owned();
        //     }
        //     if attr.name() == "email" {
        //         email = attr.value().to_owned();
        //     }
        // }

        // let cliente = Cliente {
        //     id: id.to_owned(),
        //     cpf,
        //     email,
        // };

        // Ok(cliente)
        let _now = Utc::now().format("%Y-%m-%d %H:%M:%S%.3f%z").to_string();
        let cliente = Cliente::new(
            1,
            "Fulano da Silva".to_string(),
            "".to_string(),
            Cpf::new("123.456.789-09".to_string()).unwrap(),
            _now.clone(),
            _now,
        );

        Ok(cliente)
    }

    async fn create_cliente(&mut self, cliente: Cliente) -> Result<Cliente, DomainError> {
        // let request = self.client
        //     .admin_create_user()
        //     .user_pool_id(&self.user_pool_id)
        //     .username(&cliente.cpf)
        //     .temporary_password("TempPassword123!")
        //     .user_attributes(
        //         AttributeType::builder()
        //             .name("email")
        //             .value(&cliente.email)
        //             .build(),
        //         AttributeType::builder()
        //             .name("cpf")
        //             .value(&cliente.cpf)
        //             .build(),
        //     )
        //     .build();

        // self.client.admin_create_user(request).await?;

        // println!("Cliente '{}' created successfully.", cliente.cpf);
        // Ok(())
        let _now = Utc::now().format("%Y-%m-%d %H:%M:%S%.3f%z").to_string();
        let cliente = Cliente::new(
            1,
            "Fulano da Silva".to_string(),
            "".to_string(),
            Cpf::new("123.456.789-09".to_string()).unwrap(),
            _now.clone(),
            _now,
        );

        Ok(cliente)
    }

    async fn delete_cliente(&mut self, cpf: Cpf) -> Result<(), DomainError> {
        // let request = self.client
        //     .admin_delete_user()
        //     .user_pool_id(&self.user_pool_id)
        //     .username(cpf)
        //     .build();

        // self.client.admin_delete_user(request).await?;
        // println!("Cliente '{}' deleted successfully.", cpf);
        // Ok(())

        Ok(())
    }
}
