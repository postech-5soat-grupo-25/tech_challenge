use std::sync::Arc;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use crate::base::domain_error::DomainError;
use crate::entities::cpf::Cpf;
use crate::traits::authentication_adapter::AuthenticationAdapter;
use crate::use_cases::gerenciamento_de_usuarios_use_case::UsuarioUseCase;
use crate::traits::usuario_repository::UsuarioRepository;
use crate::entities::usuario::Usuario;

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct LoginInput {
    cpf: String,
    senha: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct AuthenticationResponse {
    token: String,
    usuario: Usuario,
}

pub struct AuthController {
    usuario_use_case: UsuarioUseCase,
    authentication_adapter: Arc<dyn AuthenticationAdapter + Sync + Send>,
}

impl AuthController {
    pub fn new(
            usuario_repository: Arc<Mutex<dyn UsuarioRepository + Sync + Send>>,
            authentication_adapter: Arc<dyn AuthenticationAdapter + Sync + Send>
        ) -> AuthController {
        let usuario_use_case = UsuarioUseCase::new(usuario_repository);
        AuthController { usuario_use_case, authentication_adapter }
    }

    pub async fn login(&self, login_input: LoginInput) -> Result<AuthenticationResponse, DomainError> {
        let cpf = Cpf::new(login_input.cpf.clone())?;
        let usuario = self.usuario_use_case.get_usuario_by_cpf(cpf).await;
        match usuario {
            Ok(usuario) => {
                if !usuario.validate_senha(&login_input.senha) {
                    return Err(DomainError::Invalid("Senha inválida".to_string()));
                }
                let token = &self.authentication_adapter.get_token(usuario.clone()).await?;
                Ok(AuthenticationResponse {
                    token: token.clone(),
                    usuario,
                })
            }
            Err(_) => return Err(DomainError::Invalid("Usuário não encontrado".to_string())),
        }
    }
}