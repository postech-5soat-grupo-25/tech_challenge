use rocket;
use crate::adapter::driven::infra::repositories::in_memory_user_repository::InMemoryUserRepository;
use crate::core::application::use_cases::user_use_case::UserUseCase;

use super::controllers::usuarios_controller;


#[rocket::main]
pub async fn main() -> Result<(), rocket::Error> {
    let user_repository = InMemoryUserRepository::new();
    let user_use_case = UserUseCase::new(Box::new(user_repository));
    rocket::build()
        .mount("/users", usuarios_controller::routes())
        .manage(user_use_case)
        .launch()
        .await?;

    Ok(())
}