use crate::{core::domain::{repositories::user_repository::UserRepository, base::domain_error::DomainError, entities::usuario::Usuario, value_objects::{cpf::Cpf, endereco::{Endereco, self}}}, adapter::driven::infra::postgres::users};
use postgres_from_row::FromRow;
use tokio_postgres::Client;

use super::super::postgres::table::Table;
pub struct PostgresUserRepository {
  client: Client,
  tables: Vec<Table>,
}

const CREATE_USER: &str = "INSERT INTO users (nome, email, senha, cpf, endereco) VALUES ($1, $2, $3, $4, $5) RETURNING *";
const QUERY_USER_BY_CPF: &str = "SELECT * FROM users WHERE cpf = $1";
const QUERY_USER_BY_ID: &str = "SELECT * FROM users WHERE id = $1";
const QUERY_USERS: &str = "SELECT * FROM users";
const UPDATE_USER: &str = "UPDATE users SET nome = $1, email = $2, senha = $3, cpf = $4, endereco = $5 WHERE id = $6 RETURNING *";
const DELETE_USER: &str = "DELETE FROM users WHERE id = $1";

impl PostgresUserRepository {
  pub async fn new(client: Client, tables: Vec<Table>) -> Self {
    let mut repo = PostgresUserRepository { client, tables };

    repo.check_for_tables().await;
    repo.check_for_admin_user().await;

    repo
  }

  async fn check_for_tables(&self) {
    for table in self.tables.iter() {
      let query = table.get_create_if_not_exists_query();
      self.client.execute(query.as_str(), &[]).await.unwrap();
    };
  }

  async fn check_for_admin_user(&mut self) {
    let admin_cpf = Cpf::new("000.000.000-00".to_string()).unwrap();
    let admin_user = self.get_user_by_cpf(admin_cpf).await;
    match admin_user {
      Ok(user) => {
        println!("Admin user found: {:?}", user);
      },
      _ => {
        println!("Admin user not found, creating...");
        let admin_user = Usuario::new(
          1,
          "Albert Dias Moreira".to_string(),
          "contato@albert-dm.dev".to_string(),
          "melhor_projeto".to_string(),
          Cpf::new("000.000.000-00".to_string()).unwrap(),
          Endereco { cep: "00000-000".to_string() }
        );
        self.create_user(admin_user).await.unwrap();
      }
    }
  }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
  async fn get_users(&self) -> Result<Vec<Usuario>, DomainError>{
    let users = self.client.query(QUERY_USERS, &[]).await.unwrap();
    let mut users_vec = Vec::new();
    for user in users {
      users_vec.push(Usuario::from_row(&user));
    }
    Ok(users_vec)
  }

  async fn get_user_by_id(&self, id: usize) -> Result<Usuario, DomainError>{
    let id = id as i32;
    let user = self.client.query_one(QUERY_USER_BY_ID, &[&id]).await;
    match user {
      Ok(user) => {
        Ok(Usuario::from_row(&user))
      },
      Err(_) => {
        Err(DomainError::NotFound)
      }
    }
  }

  async fn get_user_by_cpf(&self, cpf: Cpf) -> Result<Usuario, DomainError> {
    let user = self.client.query_one(QUERY_USER_BY_CPF, &[&cpf.0]).await;
    match user {
      Ok(user) => {
        Ok(Usuario::from_row(&user))
      },
      Err(_) => {
        Err(DomainError::NotFound)
      }
    }
  }

  async fn create_user(&mut self, user: Usuario) -> Result<Usuario, DomainError> {
    let endereco = user.endereco();
    let endereco_json = tokio_postgres::types::Json(endereco);
    let new_user = self.client.query(CREATE_USER, &[
      &user.nome(),
      &user.email(),
      &user.senha(),
      &user.cpf().0,
      &endereco_json,
    ]).await.unwrap();
    let new_user = new_user.get(0);
    match new_user {
      Some(user) => {
        println!("New user created: {:?}", user);
        Ok(Usuario::from_row(user))
      },
      None => {
        println!("Error creating new user");
        Err(DomainError::Invalid("Usuário".to_string()))
      }
    }

  }

  async fn update_user(&mut self, new_user_data: Usuario) -> Result<Usuario, DomainError> {
    let endereco = new_user_data.endereco();
    let endereco_json = tokio_postgres::types::Json(endereco);
    let id = new_user_data.id().clone() as i32;

    let updated_user = self.client.query(UPDATE_USER, &[
      &new_user_data.nome(),
      &new_user_data.email(),
      &new_user_data.senha(),
      &new_user_data.cpf().0,
      &endereco_json,
      &id,
    ]).await.unwrap();
    let updated_user = updated_user.get(0);
    match updated_user {
      Some(user) => {
        Ok(Usuario::from_row(user))
      },
      None => {
        Err(DomainError::NotFound)
      }
    }
  }

  async fn delete_user(&mut self, id: usize) -> Result<(), DomainError> {
    let id = id as i32;
    let deleted_user = self.client.query_one(DELETE_USER, &[&id]).await;
    match deleted_user {
      Ok(_) => {
        Ok(())
      },
      _ => {
        Err(DomainError::NotFound)
      }
    }
  }
}