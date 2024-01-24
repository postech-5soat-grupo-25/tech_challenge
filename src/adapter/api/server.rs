use std::net::{IpAddr, Ipv4Addr};
use std::sync::Arc;

use rocket::futures::lock::Mutex;
use rocket::response::Redirect;
use rocket_okapi::swagger_ui::*;
use rocket_okapi::settings::UrlObject;

use crate::adapter::driven::infra::{repositories, postgres};
use crate::core::domain::repositories::usuario_repository::UsuarioRepository;
use crate::core::domain::repositories::cliente_repository::ClienteRepository;
use repositories::{in_memory_usuario_repository::InMemoryUsuarioRepository, postgres_usuario_repository::PostgresUsuarioRepository};
use repositories::{in_memory_cliente_repository::InMemoryClienteRepository, postgres_cliente_repository::PostgresClienteRepository};
use crate::adapter::api::config::{Config, Env};
use crate::core::application::use_cases::usuario_use_case::UsuarioUseCase;
use crate::core::application::use_cases::cliente_use_case::ClienteUseCase;

use super::controllers::{auth_controller, usuario_controller, cliente_controller};
use super::error_handling::generic_catchers;

#[get("/")]
fn redirect_to_docs() -> Redirect {
    Redirect::to(uri!("/docs"))
}

#[rocket::main]
pub async fn main() -> Result<(), rocket::Error> {
    let config = Config::build();

    println!("Loading environment variables...");
    let usuario_repository: Arc<Mutex<dyn UsuarioRepository + Sync + Send>> =
    if config.env == Env::Test {
        println!("Using in memory database");
        Arc::new(Mutex::new(InMemoryUsuarioRepository::new()))
    } else {
        println!("Connecting to database: {}", config.db_url.clone());
        let postgres_connection_manager = postgres::PgConnectionManager::new(config.db_url.clone()).await.unwrap();
        let tables = postgres::get_tables();

        Arc::new(Mutex::new(PostgresUsuarioRepository::new(postgres_connection_manager.client, tables).await))
    };
    let usuario_use_case = UsuarioUseCase::new(usuario_repository);

    let cliente_repository: Arc<Mutex<dyn ClienteRepository + Sync + Send>> =
    if config.env == Env::Test {
        println!("Using in memory database");
        Arc::new(Mutex::new(InMemoryClienteRepository::new()))
    } else {
        println!("Connecting to database: {}", config.db_url);
        let postgres_connection_manager = postgres::PgConnectionManager::new(config.db_url.clone()).await.unwrap();
        let tables = postgres::get_tables();

        Arc::new(Mutex::new(PostgresClienteRepository::new(postgres_connection_manager.client, tables).await))
    };

    let cliente_use_case = ClienteUseCase::new(cliente_repository);

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
                UrlObject::new("Usuarios", "/usuarios/openapi.json"),
                UrlObject::new("Clientes", "/clientes/openapi.json")
            ],
            ..Default::default()
        }),
    )
    .mount("/auth", auth_controller::routes())
    .mount("/usuarios", usuario_controller::routes())
    .mount("/clientes", cliente_controller::routes())
    .register("/usuarios", usuario_controller::catchers())
    .register("/clientes", cliente_controller::catchers())
    .manage(usuario_use_case)
    .manage(cliente_use_case)
    .configure(server_config)
    .launch()
    .await?;

    println!("Server running on {}", config.env.to_string());
    Ok(())
}