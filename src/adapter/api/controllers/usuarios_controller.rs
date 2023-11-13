use rocket::State;
use serde_json;
use crate::core::application::use_cases::user_use_case::UserUseCase;

#[get("/")]
pub async fn get_users(state: &State<UserUseCase>) -> Result<String, ()> {
    let users = state.get_users().await;
    match users {
        Ok(users) => {
            Ok(serde_json::to_string(&users).unwrap())
        },
        Err(error) => {
            match error {
                _ => {
                  println!("Erro ao buscar usuários");
                  Ok(String::from("Erro ao buscar usuários"))
                }
            }
        }
    }
}

#[get("/<id>")]
pub async fn get_user(state: &State<UserUseCase>, id: i32) -> Result<String, ()> {
    let user = state.get_user_by_id(id).await;
    match user {
        Ok(user) => {
            Ok(serde_json::to_string(&user).unwrap())
        },
        Err(error) => {
            match error {
                _ => {
                  println!("Erro ao buscar usuários");
                  Ok(String::from("Erro ao buscar usuários"))
                }
            }
        }
    }
}

pub fn routes() -> Vec<rocket::Route> {
    routes![get_users, get_user]
}