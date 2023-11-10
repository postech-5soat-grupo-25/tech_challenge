use rocket::State;
use crate::core::application::use_cases::user_use_case::UserUseCase;

#[get("/")]
pub fn get_users(state: State<UserUseCase>) -> Result<&'static str, ()> {
    let users = state.get_users();
    match users {
        Ok(users) => {
            for user in users {
                println!("Usuario: {}", user.nome);
            }
            Ok("Usuario encontrado")
        },
        Err(error) => {
            match error {
                _ => {
                  println!("Erro ao buscar usuários");
                  Ok("Erro ao buscar usuários")
                }
            }
        }
    }
}

pub fn routes() -> Vec<rocket::Route> {
    routes![get_users]
}