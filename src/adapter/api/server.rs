use rocket;
use crate::adapter::driven::infra::repositories::user_repository::UserRepository;
use crate::core::application::use_cases::user_use_case::UserUseCase;

use super::controllers::usuarios_controller;

pub fn main() -> Result<(), rocket::error::LaunchError> {
    let user_repository = UserRepository::new();
    let user_use_case = UserUseCase::new(Box::new(user_repository));
    rocket::ignite()
        .mount("/users", usuarios_controller::routes())
        .manage(user_use_case)
        .launch();

    Ok(())
}