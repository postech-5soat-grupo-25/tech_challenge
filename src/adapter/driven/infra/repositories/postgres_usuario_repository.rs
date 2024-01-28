use chrono::Utc;
use postgres_from_row::FromRow;
use tokio_postgres::Client;

use crate::core::domain::base::domain_error::DomainError;
use crate::core::domain::entities::usuario::Usuario;
use crate::core::domain::repositories::usuario_repository::UsuarioRepository;
use crate::core::domain::value_objects::cpf::Cpf;

use super::super::postgres::table::Table;
pub struct PostgresUsuarioRepository {
    client: Client,
    tables: Vec<Table>,
}

const CREATE_USUARIO: &str = "INSERT INTO usuario (nome, email, cpf, senha, tipo, status) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *";
const UPDATE_USUARIO: &str = "UPDATE usuario SET nome = $1, email = $2, cpf = $3, senha = $4, tipo = $5, status = $6 WHERE id = $7 RETURNING *";
const QUERY_USUARIO_BY_CPF: &str = "SELECT * FROM usuario WHERE cpf = $1";
const QUERY_USUARIO_BY_ID: &str = "SELECT * FROM usuario WHERE id = $1";
const QUERY_USUARIOS: &str = "SELECT * FROM usuario";
const DELETE_USUARIO: &str = "DELETE FROM usuario WHERE cpf = $1";

impl PostgresUsuarioRepository {
    pub async fn new(client: Client, tables: Vec<Table>) -> Self {
        let mut repo = PostgresUsuarioRepository { client, tables };

        repo.check_for_tables().await;
        repo.check_for_usuario_admin().await;

        repo
    }

    async fn check_for_tables(&self) {
        for table in self.tables.iter() {
            let query = table.get_create_if_not_exists_query();
            self.client.execute(query.as_str(), &[]).await.unwrap();
        }
    }

    async fn check_for_usuario_admin(&mut self) {
        let admin_cpf = Cpf::new("000.000.000-00".to_string()).unwrap();
        let usuario_admin = self.get_usuario_by_cpf(admin_cpf).await;
        match usuario_admin {
            Ok(usuario) => {
                println!("Usuário Admin encontrado: {:?}", usuario);
            }
            _ => {
                println!("Usuário Admin não encontrado. Criando...");
                let _id = 0;
                let _now = Utc::now().format("%Y-%m-%d %H:%M:%S%.3f%z").to_string();
                let usuario_admin = Usuario::new(
                    _id,
                    "Administrador".to_string(),
                    "admin@fastfood.com.br".to_string(),
                    Cpf::new("000.000.000-00".to_string()).unwrap(),
                    "melhor_projeto".to_string(),
                    "Admin".parse().unwrap(),
                    "Ativo".parse().unwrap(),
                    _now.clone(),
                    _now,
                );
                self.create_usuario(usuario_admin).await.unwrap();
            }
        }
    }
}

#[async_trait]
impl UsuarioRepository for PostgresUsuarioRepository {
    async fn get_usuarios(&self) -> Result<Vec<Usuario>, DomainError> {
        let usuarios = self.client.query(QUERY_USUARIOS, &[]).await.unwrap();
        let mut usuarios_vec = Vec::new();
        for usuario in usuarios {
            usuarios_vec.push(Usuario::from_row(&usuario));
        }
        Ok(usuarios_vec)
    }

    async fn get_usuario_by_id(&self, id: usize) -> Result<Usuario, DomainError> {
        let id = id as i32;
        let usuario = self.client.query_one(QUERY_USUARIO_BY_ID, &[&id]).await;
        match usuario {
            Ok(usuario) => Ok(Usuario::from_row(&usuario)),
            Err(_) => Err(DomainError::NotFound),
        }
    }

    async fn get_usuario_by_cpf(&self, cpf: Cpf) -> Result<Usuario, DomainError> {
        let usuario = self.client.query_one(QUERY_USUARIO_BY_CPF, &[&cpf.0]).await;
        match usuario {
            Ok(usuario) => Ok(Usuario::from_row(&usuario)),
            Err(_) => Err(DomainError::NotFound),
        }
    }

    async fn create_usuario(&mut self, usuario: Usuario) -> Result<Usuario, DomainError> {
        let new_usuario = self
            .client
            .query(
                CREATE_USUARIO,
                &[
                    &usuario.nome(),
                    &usuario.email(),
                    &usuario.cpf().0,
                    &usuario.senha(),
                    &usuario.tipo().to_string(),
                    &usuario.status().to_string(),
                ],
            )
            .await
            .unwrap();
        let new_usuario = new_usuario.get(0);
        match new_usuario {
            Some(usuario) => {
                println!("Novo usuário cadastrado: {:?}", usuario);
                Ok(Usuario::from_row(usuario))
            }
            None => Err(DomainError::Invalid("Usuário".to_string())),
        }
    }

    async fn update_usuario(&mut self, dados_usuario_atualizado: Usuario) -> Result<Usuario, DomainError> {
        let id = dados_usuario_atualizado.id().clone() as i32;
        let usuario_atualizado = self.client.query(UPDATE_USUARIO, &[
          &dados_usuario_atualizado.nome(),
          &dados_usuario_atualizado.email(),
          &dados_usuario_atualizado.cpf().0,
          &dados_usuario_atualizado.senha(),
          &dados_usuario_atualizado.tipo().to_string(),
          &dados_usuario_atualizado.status().to_string(),
          &id,
        ]).await.unwrap();
        let usuario_atualizado = usuario_atualizado.get(0);
        match usuario_atualizado {
          Some(user) => {
            Ok(Usuario::from_row(user))
          },
          None => {
            Err(DomainError::NotFound)
          }
        }
      }

    async fn delete_usuario(&mut self, cpf: Cpf) -> Result<(), DomainError> {
        let deleted_usuario = self.client.query_one(DELETE_USUARIO, &[&cpf.0]).await;
        match deleted_usuario {
            Ok(_) => Ok(()),
            _ => Err(DomainError::NotFound),
        }
    }
}
