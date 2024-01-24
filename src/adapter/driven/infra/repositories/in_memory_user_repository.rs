
use rocket::tokio::time::{sleep, Duration};

use crate::core::domain::base::domain_error::DomainError;
use crate::core::domain::entities::usuario::Usuario;
use crate::core::domain::value_objects::cpf::Cpf;
use crate::core::domain::value_objects::endereco::Endereco;
use crate ::core::domain::repositories::usuario_repository::UsuarioRepository;

#[derive(Clone)]
pub struct InMemoryUserRepository {
  _users: Vec<Usuario>,
}

impl InMemoryUserRepository {
  pub fn new() -> Self {
    let user = Usuario::new(
      1,
      "Albert Dias Moreira".to_string(),
      "contato@albert-dm.dev".to_string(),
      "melhor_projeto".to_string(),
      Cpf::new("000.000.000-00".to_string()).unwrap(),
      Endereco { cep: "00000-000".to_string() }
    );

    println!("Usando repositório em memória!");

    InMemoryUserRepository {
      _users: vec![user],
    }
  }
}

#[async_trait]
impl UsuarioRepository for InMemoryUserRepository {
  async fn get_usuarios(&self) -> Result<Vec<Usuario>, DomainError> {
    let users = self._users.clone();
    sleep(Duration::from_secs(1)).await;
    Ok(users)
  }

  async fn get_usuario_by_id(&self, id: usize) -> Result<Usuario, DomainError> {
    sleep(Duration::from_secs(1)).await;
    for user in &self._users {
      if user.id().to_owned() == id {
        return Ok(user.clone());
      }
    }
    Err(DomainError::NotFound)
  }

  async fn get_usuario_by_cpf(&self, cpf: Cpf) -> Result<Usuario, DomainError> {
    sleep(Duration::from_secs(1)).await;
    for user in &self._users {
      if user.cpf().to_owned() == cpf {
        return Ok(user.clone());
      }
    }
    Err(DomainError::NotFound)
  }

  async fn create_usuario(&mut self, user: Usuario) -> Result<Usuario, DomainError> {
    sleep(Duration::from_secs(1)).await;
    let existing_user = self.get_usuario_by_id(user.id().to_owned()).await;

    if existing_user.is_ok() {
      return Err(DomainError::AlreadyExists);
    }

    let mut user_list = self._users.clone();
    user_list.push(user.clone());

    self._users = user_list;


    Ok(user.clone())
  }

  async fn update_usuario(&mut self, new_user_data: Usuario) -> Result<Usuario, DomainError> {
    let user_list = &mut self._users;
    for user in &mut user_list.iter_mut() {
      if user.id() == new_user_data.id() {
        *user = new_user_data.clone();
        return Ok(user.clone());
      }
    }
    Err(DomainError::NotFound)
  }

  async fn delete_usuario(&mut self, id: usize) -> Result<(), DomainError> {
    let user_list = &mut self._users;
    for (index, user) in user_list.iter_mut().enumerate() {
      if user.id().to_owned() == id {
        user_list.remove(index);
        return Ok(());
      }
    }
    Err(DomainError::NotFound)
  }
}

unsafe impl Sync for InMemoryUserRepository {}

unsafe impl Send for InMemoryUserRepository {}