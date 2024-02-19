use chrono::Utc;
use rocket::tokio::time::{sleep, Duration};

use crate::base::domain_error::DomainError;
use crate::entities::usuario::Usuario;
use crate::traits::usuario_repository::UsuarioRepository;
use crate::entities::cpf::Cpf;

#[derive(Clone)]
pub struct InMemoryUsuarioRepository {
    _usuarios: Vec<Usuario>,
}

impl InMemoryUsuarioRepository {
    pub fn new() -> Self {
        let _id = 0;
        let _now = Utc::now().format("%Y-%m-%d %H:%M:%S%.3f%z").to_string();
        let usuario = Usuario::new(
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

        println!("Usando repositório em memória!");

        InMemoryUsuarioRepository {
            _usuarios: vec![usuario],
        }
    }
}

#[async_trait]
impl UsuarioRepository for InMemoryUsuarioRepository {
    async fn get_usuarios(&self) -> Result<Vec<Usuario>, DomainError> {
        let usuarios = self._usuarios.clone();
        sleep(Duration::from_secs(1)).await;
        Ok(usuarios)
    }

    async fn get_usuario_by_id(&self, id: usize) -> Result<Usuario, DomainError> {
        sleep(Duration::from_secs(1)).await;
        for usuario in &self._usuarios {
            if usuario.id().to_owned() == id {
                return Ok(usuario.clone());
            }
        }
        Err(DomainError::NotFound)
    }

    async fn get_usuario_by_cpf(&self, cpf: Cpf) -> Result<Usuario, DomainError> {
        sleep(Duration::from_secs(1)).await;
        for usuario in &self._usuarios {
            if usuario.cpf().to_owned() == cpf {
                return Ok(usuario.clone());
            }
        }
        Err(DomainError::NotFound)
    }

    async fn create_usuario(&mut self, usuario: Usuario) -> Result<Usuario, DomainError> {
        sleep(Duration::from_secs(1)).await;
        let existing_usuario = self.get_usuario_by_id(usuario.id().to_owned()).await;

        if existing_usuario.is_ok() {
            return Err(DomainError::AlreadyExists);
        }

        let mut usuario_list = self._usuarios.clone();
        usuario_list.push(usuario.clone());

        self._usuarios = usuario_list;

        Ok(usuario.clone())
    }

    async fn update_usuario(&mut self, dados_usuario_atualizado: Usuario) -> Result<Usuario, DomainError> {
        sleep(Duration::from_secs(1)).await;
        let mut usuario_list = self._usuarios.clone();
        for usuario in &mut usuario_list.iter_mut() {
        if usuario.id() == dados_usuario_atualizado.id() {
            *usuario = dados_usuario_atualizado.clone();
            return Ok(usuario.clone());
        }
        }
        Err(DomainError::NotFound)
    }

    async fn delete_usuario(&mut self, cpf: Cpf) -> Result<(), DomainError> {
        let usuario_list = &mut self._usuarios;
        for (index, usuario) in usuario_list.iter_mut().enumerate() {
            if usuario.cpf().to_owned() == cpf {
                usuario_list.remove(index);
                return Ok(());
            }
        }
        Err(DomainError::NotFound)
    }
}

unsafe impl Sync for InMemoryUsuarioRepository {}
unsafe impl Send for InMemoryUsuarioRepository {}
