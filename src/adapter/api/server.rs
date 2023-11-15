use std::sync::Arc;

use crate::adapter::driven::infra::repositories::in_memory_user_repository::InMemoryUserRepository;
use crate::core::application::use_cases::user_use_case::UserUseCase;
use rocket::futures::lock::Mutex;
use rocket::response::Redirect;
use rocket_okapi::swagger_ui::*;
use rocket_okapi::settings::UrlObject;
use rocket;

use super::controllers::user_controller;

#[get("/")]
fn redirect_to_docs() -> Redirect {
    Redirect::to(uri!("/docs"))
}

#[rocket::main]
pub async fn main() -> Result<(), rocket::Error> {
    let user_repository = Arc::new(Mutex::new(InMemoryUserRepository::new()));
    let user_use_case = UserUseCase::new(user_repository);
    rocket::build()
    .mount("/", routes![redirect_to_docs])
    .mount(
        "/docs/",
        make_swagger_ui(&SwaggerUIConfig {
            urls: vec![UrlObject::new("Users", "/users/openapi.json")],
            ..Default::default()
        }),
    )
        .mount("/users", user_controller::routes())
        .manage(user_use_case)
        .configure(rocket::Config::figment().merge(("port", 3000)))
        .launch()
        .await?;

    Ok(())
}
