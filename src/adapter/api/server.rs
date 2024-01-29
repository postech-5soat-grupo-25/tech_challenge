use std::net::{IpAddr, Ipv4Addr};
use std::sync::Arc;
use std::process;

use rocket::futures::lock::Mutex;
use rocket::response::Redirect;
use rocket_okapi::swagger_ui::*;
use rocket_okapi::settings::UrlObject;

use crate::adapter::driven::infra::repositories::postgres_produto_repository::PostgresProdutoRepository;
use crate::adapter::driven::infra::{repositories, postgres};
use crate::core::domain::repositories::produto_repository::ProdutoRepository;
use crate::core::domain::repositories::usuario_repository::UsuarioRepository;
use crate::core::domain::repositories::cliente_repository::ClienteRepository;
use crate::core::domain::repositories::pedido_repository::PedidoRepository;
use repositories::{in_memory_pedido_repository::InMemoryPedidoRepository, postgres_pedido_repository::PostgresPedidoRepository};
use repositories::{in_memory_usuario_repository::InMemoryUsuarioRepository, postgres_usuario_repository::PostgresUsuarioRepository};
use repositories::{in_memory_cliente_repository::InMemoryClienteRepository, postgres_cliente_repository::PostgresClienteRepository};
use crate::adapter::api::config::{Config, Env};
use crate::core::application::use_cases::usuario_use_case::UsuarioUseCase;
use crate::core::application::use_cases::cliente_use_case::ClienteUseCase;
use crate::core::application::use_cases::preparacao_e_entrega_use_case::PreparacaoeEntregaUseCase;

use super::controllers::{auth_controller, usuario_controller, cliente_controller, pedido_controller, produto_controller};
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

    // Cloning cliente_repository to share ownership
    let cloned_cliente_repository = Arc::clone(&cliente_repository);

    let cliente_use_case = ClienteUseCase::new(cliente_repository);

    let postgres_connection_manager = postgres::PgConnectionManager::new(config.db_url.clone()).await.unwrap();
    let tables = postgres::get_tables();
    let produto_repository: Arc<Mutex<dyn ProdutoRepository + Sync + Send>> = 
        Arc::new(Mutex::new(PostgresProdutoRepository::new(postgres_connection_manager.client, tables).await));

    // Cloning produto_repository to share ownership
    let cloned_produto_repository = Arc::clone(&produto_repository);

    let pedido_repository: Arc<Mutex<dyn PedidoRepository + Sync + Send>> =
    if config.env == Env::Test {
        println!("Using in memory database");
        Arc::new(Mutex::new(InMemoryPedidoRepository::new()))
    } else {
        println!("Connecting to database: {}", config.db_url);
        let postgres_connection_manager = postgres::PgConnectionManager::new(config.db_url.clone()).await.unwrap();
        let tables = postgres::get_tables();

        Arc::new(Mutex::new(PostgresPedidoRepository::new(postgres_connection_manager.client, tables, cloned_cliente_repository, cloned_produto_repository).await))
    };

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
                UrlObject::new("Usuarios", "/usuarios/openapi.json"),
                UrlObject::new("Clientes", "/clientes/openapi.json"),
                UrlObject::new("Pedido", "/pedido/openapi.json"),
            ],
            ..Default::default()
        }),
    )
    .mount("/auth", auth_controller::routes())
    .mount("/usuarios", usuario_controller::routes())
    .mount("/clientes", cliente_controller::routes())
    .mount("/pedido", pedido_controller::routes())
    .mount("/produto", produto_controller::routes())
    .register("/usuarios", usuario_controller::catchers())
    .register("/clientes", cliente_controller::catchers())
    .manage(usuario_use_case)
    .manage(cliente_use_case)
    .manage(preparacao_e_entrega_use_case)
    .configure(server_config)
    .launch()
    .await?;

    println!("Server running on {}", config.env.to_string());
    Ok(())
}