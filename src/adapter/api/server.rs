use std::net::{IpAddr, Ipv4Addr};
use std::sync::Arc;



use crate::adapter::driven::infra::{repositories, postgres};
use crate::core::application::use_cases::product_use_case::ProductUseCase;
use crate::core::domain::repositories::user_repository::UserRepository;
use crate::core::domain::repositories::product_repository::ProductRepository;
use repositories::{in_memory_user_repository::InMemoryUserRepository, postgres_user_repository::PostgresUserRepository, postgres_product_repository::PostgresProductRepository};
use crate::adapter::api::config::{Config, Env};
use crate::core::application::use_cases::user_use_case::UserUseCase;
use rocket::futures::lock::Mutex;
use rocket::response::Redirect;
use rocket_okapi::swagger_ui::*;
use rocket_okapi::settings::UrlObject;

use super::controllers::{user_controller, auth_controller};
use super::error_handling::generic_catchers;

#[get("/")]
fn redirect_to_docs() -> Redirect {
    Redirect::to(uri!("/docs"))
}

#[rocket::main]
pub async fn main() -> Result<(), rocket::Error> {
    let config = Config::build();

    println!("Loading environment variables...");
    let postgres_connection_manager = if config.env != Env::Test {
        println!("Connecting to database: {}", config.db_url);
        let client = Arc::new(postgres::PgConnectionManager::new(config.db_url).await.unwrap().client);
        Some(client)
    } else {
        None
    };

    let tables = postgres::get_tables();

    let user_repository: Arc<Mutex<dyn UserRepository + Sync + Send>> =
    if config.env == Env::Test {
        println!("Using in memory database");
        Arc::new(Mutex::new(InMemoryUserRepository::new()))
    } else {
        Arc::new(Mutex::new(PostgresUserRepository::new(Arc::clone(&postgres_connection_manager.as_ref().unwrap()), tables.clone()).await))
    };

    let product_repository: Arc<Mutex<dyn ProductRepository + Sync + Send>> = Arc::new(Mutex::new(PostgresProductRepository::new(Arc::clone(&postgres_connection_manager.as_ref().unwrap()), tables).await));

    let user_use_case = UserUseCase::new(user_repository);

    let server_config = rocket::Config::figment()
        .merge(("address", IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0))))
        .merge(("port", 3000));

    rocket::build()
    .mount("/", routes![redirect_to_docs])
    .register("/", generic_catchers())
    .mount(
        "/docs/",
        make_swagger_ui(&SwaggerUIConfig {
            urls: vec![
                UrlObject::new("Auth", "/auth/openapi.json"),
                UrlObject::new("Users", "/users/openapi.json")
            ],
            ..Default::default()
        }),
    )
    .mount("/auth", auth_controller::routes())
    .mount("/users", user_controller::routes())
    .register("/users", user_controller::catchers())
    .manage(user_use_case)
    .configure(server_config)
    .launch()
    .await?;

    println!("Server running on {}", config.env.to_string());
    Ok(())
}