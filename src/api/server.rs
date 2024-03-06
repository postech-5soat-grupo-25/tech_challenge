use rocket::response::Redirect;
use rocket_okapi::settings::UrlObject;
use rocket_okapi::swagger_ui::*;
use tokio::sync::Mutex;
use std::net::{IpAddr, Ipv4Addr};
use std::sync::Arc;

use super::error_handling::generic_catchers;
use super::routes::{
    auth_route, cliente_route, pedido_route, produto_route, usuario_route,
};
use crate::api::config::{Config, Env};
use crate::external::pagamento::mock::MockPagamentoSuccesso;
use crate::external::postgres;
use crate::adapters::jwt_authentication_adapter::JWTAuthenticationAdapter;
use crate::gateways::{
    in_memory_cliente_gateway::InMemoryClienteRepository,
    in_memory_pedido_gateway::InMemoryPedidoRepository,
    in_memory_pagamento_gateway::InMemoryPagamentoRepository,
    in_memory_usuario_gateway::InMemoryUsuarioRepository,
    postgres_cliente_gateway::PostgresClienteRepository,
    postgres_pedido_gateway::PostgresPedidoRepository,
    postgres_pagamento_gateway::PostgresPagamentoRepository,
    postgres_usuario_gateway::PostgresUsuarioGateway,
    postgres_produto_gateway::PostgresProdutoRepository,
};
use crate::traits::authentication_adapter::AuthenticationAdapter;
use crate::traits::pagamento_adapter::PagamentoAdapter;
use crate::traits::{
    cliente_gateway::ClienteGateway, pedido_gateway::PedidoGateway,
    produto_gateway::ProdutoGateway, usuario_gateway::UsuarioGateway,
    pagamento_gateway::PagamentoGateway
};

#[get("/")]
fn redirect_to_docs() -> Redirect {
    Redirect::to(uri!("/docs"))
}

#[rocket::main]
pub async fn main() -> Result<(), rocket::Error> {
    let config = Config::build();

    let jwt_authentication_adapter: Arc<dyn AuthenticationAdapter + Sync + Send>  = Arc::new(JWTAuthenticationAdapter::new(
        config.secret.clone(),
    ));

    println!("Loading environment variables...");
    let usuario_repository: Arc<Mutex<dyn UsuarioGateway + Sync + Send>> = if config.env
        == Env::Test
    {
        println!("Using in memory database");
        Arc::new(Mutex::new(InMemoryUsuarioRepository::new()))
    } else {
        println!("Connecting to database: {}", config.db_url.clone());
        let postgres_connection_manager = postgres::PgConnectionManager::new(config.db_url.clone())
            .await
            .unwrap();
        let tables = postgres::get_tables();

        Arc::new(Mutex::new(
            PostgresUsuarioGateway::new(postgres_connection_manager.client, tables).await,
        ))
    };

    let cliente_repository: Arc<Mutex<dyn ClienteGateway + Sync + Send>> = if config.env
        == Env::Test
    {
        println!("Using in memory database");
        Arc::new(Mutex::new(InMemoryClienteRepository::new()))
    } else {
        println!("Connecting to database: {}", config.db_url);
        let postgres_connection_manager = postgres::PgConnectionManager::new(config.db_url.clone())
            .await
            .unwrap();
        let tables = postgres::get_tables();

        Arc::new(Mutex::new(
            PostgresClienteRepository::new(postgres_connection_manager.client, tables).await,
        ))
    };

    //let pagamento_adapter: Arc<Mutex<dyn PagamentoAdapter + Sync + Send>> = Arc::new(Mutex::new(MockPagamentoSuccesso {}));

    // Cloning cliente_repository to share ownership
    let cloned_cliente_repository = Arc::clone(&cliente_repository);

    let postgres_connection_manager = postgres::PgConnectionManager::new(config.db_url.clone())
        .await
        .unwrap();
    let tables = postgres::get_tables();
    let produto_repository: Arc<Mutex<dyn ProdutoGateway + Sync + Send>> = Arc::new(Mutex::new(
        PostgresProdutoRepository::new(postgres_connection_manager.client, tables).await,
    ));

    // Cloning produto_repository to share ownership
    let cloned_produto_repository = Arc::clone(&produto_repository);

    let pedido_repository: Arc<Mutex<dyn PedidoGateway + Sync + Send>> = if config.env
        == Env::Test
    {
        println!("Using in memory database");
        Arc::new(Mutex::new(InMemoryPedidoRepository::new()))
    } else {
        println!("Connecting to database: {}", config.db_url);
        let postgres_connection_manager = postgres::PgConnectionManager::new(config.db_url.clone())
            .await
            .unwrap();
        let tables = postgres::get_tables();

        Arc::new(Mutex::new(
            PostgresPedidoRepository::new(
                postgres_connection_manager.client,
                tables,
                cloned_cliente_repository,
                cloned_produto_repository,
            )
            .await,
        ))
    };

    let cloned_pedido_repository = Arc::clone(&pedido_repository);

    let pagamento_repository: Arc<Mutex<dyn PagamentoGateway + Sync + Send>> = if config.env
        == Env::Test
    {
        println!("Using in memory database");
        Arc::new(Mutex::new(InMemoryPagamentoRepository::new()))
    } else {
        println!("Connecting to database: {}", config.db_url.clone());
        let postgres_connection_manager = postgres::PgConnectionManager::new(config.db_url.clone())
            .await
            .unwrap();
        let tables = postgres::get_tables();

        Arc::new(Mutex::new(
            PostgresPagamentoRepository::new(postgres_connection_manager.client, tables, cloned_pedido_repository).await,
        ))
    };

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
                    UrlObject::new("Produtos", "/produtos/openapi.json"),
                    UrlObject::new("Pedidos", "/pedidos/openapi.json"),
                ],
                ..Default::default()
            }),
        )
        .mount("/auth", auth_route::routes())
        .mount("/usuarios", usuario_route::routes())
        .mount("/clientes", cliente_route::routes())
        .mount("/produtos", produto_route::routes())
        .mount("/pedidos", pedido_route::routes())
        .register("/usuarios", usuario_route::catchers())
        .register("/clientes", cliente_route::catchers())
        .register("/produtos", produto_route::catchers())
        .register("/pedidos", pedido_route::catchers())
        .manage(jwt_authentication_adapter)
        .manage(usuario_repository)
        .manage(cliente_repository)
        .manage(produto_repository)
        .manage(pedido_repository)
        .manage(pagamento_repository)
        .configure(server_config)
        .launch()
        .await?;

    println!("Server running on {}", config.env.to_string());
    Ok(())
}
