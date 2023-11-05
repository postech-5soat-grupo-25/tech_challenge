
use crate::core::domain::base::domain_exception::Result;
use crate::core::domain::entities::usuario::Usuario;
use crate::core::domain::value_objects::cpf::Cpf;
use crate::core::domain::value_objects::endereco::Endereco;
use crate ::core::domain::repositories::user_repository::UserRepositoryInterface;
pub struct UserRepository {
  _users: Vec<Usuario>,
}

impl UserRepository {
  fn new() -> Self {
    let user = Usuario::new(
      "Albert Dias Moreira".to_string(),
      "contato@albert-dm.dev".to_string(),
      "melhor_projeto".to_string(),
      Cpf { numero: "000.000.000-00".to_string() },
      Endereco { cep: "00000-000".to_string() }
    );
    UserRepository {
      _users: vec![user],
    }
  }
}

impl UserRepositoryInterface for UserRepository {
  fn get_users(&self) -> Result<Vec<Usuario>> {
    Ok(self._users.clone())
  }
}