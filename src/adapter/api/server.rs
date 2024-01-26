use std::net::{IpAddr, Ipv4Addr};
use std::sync::Arc;
use std::process;

use rocket::futures::lock::Mutex;
use rocket::response::Redirect;
use rocket_okapi::swagger_ui::*;
use rocket_okapi::settings::UrlObject;

use crate::adapter::driven::infra::{repositories, postgres};
use crate::core::domain::repositories::user_repository::UserRepository;
use crate::core::domain::repositories::cliente_repository::ClienteRepository;
use crate::core::domain::repositories::pedido_repository::PedidoRepository;
use repositories::in_memory_pedido_repository::InMemoryPedidoRepository;
use repositories::{in_memory_user_repository::InMemoryUserRepository, postgres_user_repository::PostgresUserRepository};
use repositories::{in_memory_cliente_repository::InMemoryClienteRepository, postgres_cliente_repository::PostgresClienteRepository};
use crate::adapter::api::config::{Config, Env};
use crate::core::application::use_cases::user_use_case::UserUseCase;
use crate::core::application::use_cases::cliente_use_case::ClienteUseCase;
use crate::core::application::use_cases::preparacao_e_entrega_use_case::PreparacaoeEntregaUseCase;

use super::controllers::{auth_controller, user_controller, cliente_controller, pedido_controller};
use super::error_handling::generic_catchers;

#[get("/")]
fn redirect_to_docs() -> Redirect {
    Redirect::to(uri!("/docs"))
}

#[rocket::main]
pub async fn main() -> Result<(), rocket::Error> {
    let config = Config::build();

    println!("Loading environment variables...");
    let user_repository: Arc<Mutex<dyn UserRepository + Sync + Send>> =
    if config.env == Env::Test {
        println!("Using in memory database");
        Arc::new(Mutex::new(InMemoryUserRepository::new()))
    } else {
        println!("Connecting to database: {}", config.db_url.clone());
        let postgres_connection_manager = postgres::PgConnectionManager::new(config.db_url.clone()).await.unwrap();
        let tables = postgres::get_tables();

        Arc::new(Mutex::new(PostgresUserRepository::new(postgres_connection_manager.client, tables).await))
    };
    let user_use_case = UserUseCase::new(user_repository);

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

    let pedido_repository: Arc<Mutex<dyn PedidoRepository + Sync + Send>> =
    // TODO
    // if config.env == Env::Test {
    //     println!("Using in memory database");
    //     Arc::new(Mutex::new(InMemoryPedidoRepository::new()))
    // } else {
    //     println!("MASSIVE FAILURE");
    //     process::exit(1);
    // };
    Arc::new(Mutex::new(InMemoryPedidoRepository::new()));

    let preparacao_e_entrega_use_case = PreparacaoeEntregaUseCase::new(pedido_repository);

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
                UrlObject::new("Users", "/users/openapi.json"),
                UrlObject::new("Clientes", "/clientes/openapi.json"),
                UrlObject::new("Pedido", "/pedido/openapi.json")
            ],
            ..Default::default()
        }),
    )
    .mount("/auth", auth_controller::routes())
    .mount("/users", user_controller::routes())
    .mount("/clientes", cliente_controller::routes())
    .mount("/pedido", pedido_controller::routes())
    .register("/users", user_controller::catchers())
    .register("/clientes", cliente_controller::catchers())
    .manage(user_use_case)
    .manage(cliente_use_case)
    .manage(preparacao_e_entrega_use_case)
    .configure(server_config)
    .launch()
    .await?;

    println!("Server running on {}", config.env.to_string());
    Ok(())
}