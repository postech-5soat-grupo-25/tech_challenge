use crate::core::{
    application::ports::pagamento_port::{PagamentoPort, StatusPagamento},
    domain::{
        base::domain_error::DomainError,
        entities::{
            pedido::{Pedido, Status},
            produto::{Categoria, Produto},
        },
        repositories::{
            cliente_repository::ClienteRepository, 
            pedido_repository::PedidoRepository,
            produto_repository::ProdutoRepository,
        },
    },
};
use chrono::Utc;
use rocket::futures::lock::Mutex;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone, Debug, Deserialize, JsonSchema)]
pub struct CreatePedidoInput {
    cliente_id: Option<usize>,
    lanche_id: Option<usize>,
    acompanhamento_id: Option<usize>,
    bebida_id: Option<usize>,
}

#[derive(Clone)]
pub struct PedidosEPagamentosUseCase {
    pedido_repository: Arc<Mutex<dyn PedidoRepository + Sync + Send>>,
    cliente_repository: Arc<Mutex<dyn ClienteRepository + Sync + Send>>,
    produto_repository: Arc<Mutex<dyn ProdutoRepository + Sync + Send>>,
    pagamento_adapter: Arc<Mutex<dyn PagamentoPort + Sync + Send>>,
}

impl PedidosEPagamentosUseCase {
    pub fn new(
        pedido_repository: Arc<Mutex<dyn PedidoRepository + Sync + Send>>,
        cliente_repository: Arc<Mutex<dyn ClienteRepository + Sync + Send>>,
        produto_repository: Arc<Mutex<dyn ProdutoRepository + Sync + Send>>,
        pagamento_adapter: Arc<Mutex<dyn PagamentoPort + Sync + Send>>,
    ) -> Self {
        PedidosEPagamentosUseCase {
            pedido_repository,
            cliente_repository,
            produto_repository,
            pagamento_adapter,
        }
    }

    pub async fn lista_pedidos(&self) -> Result<Vec<Pedido>, DomainError> {
        let mut pedido_repository = self.pedido_repository.lock().await;
        pedido_repository.lista_pedidos().await
    }

    pub async fn seleciona_pedido_por_id(&self, id: usize) -> Result<Pedido, DomainError> {
        let pedido_repository = self.pedido_repository.lock().await;
        pedido_repository.get_pedido_by_id(id).await
    }

    pub async fn novo_pedido(&self, pedido_input: CreatePedidoInput) -> Result<Pedido, DomainError> {
        let cliente = if let Some(cliente_id) = pedido_input.cliente_id {
            let cliente_repo = self.cliente_repository.lock().await;
            cliente_repo.get_cliente_by_id(cliente_id).await.ok()
        } else {
            None
        };
    
        let lanche = if let Some(lanche_id) = pedido_input.lanche_id {
            let produto_repo = self.produto_repository.lock().await;
            produto_repo.get_produto_by_id(lanche_id).await.ok()
        } else {
            None
        };
    
        let bebida = if let Some(bebida_id) = pedido_input.bebida_id {
            let produto_repo = self.produto_repository.lock().await;
            produto_repo.get_produto_by_id(bebida_id).await.ok()
        } else {
            None
        };
    
        let acompanhamento = if let Some(acompanhamento_id) = pedido_input.acompanhamento_id {
            let produto_repo = self.produto_repository.lock().await;
            produto_repo.get_produto_by_id(acompanhamento_id).await.ok()
        } else {
            None
        };

        let _now = Utc::now().format("%Y-%m-%d %H:%M:%S%.3f%z").to_string();
        let pedido = Pedido::new(
            0, 
            cliente,
            lanche,
            acompanhamento,
            bebida,
            String::from("Mercado Pago"),
            Status::Pendente,
            _now.clone(),
            _now,
        );
    
        let mut pedido_repository = self.pedido_repository.lock().await;
        pedido_repository.create_pedido(pedido).await
    }

    pub async fn adicionar_cliente(
        &self,
        pedido_id: usize,
        cliente_id: usize,
    ) -> Result<Pedido, DomainError> {
        let cliente_repo = self.cliente_repository.lock().await;
        let cliente = cliente_repo.get_cliente_by_id(cliente_id).await?;
        drop(cliente_repo);
        let mut pedido_repository = self.pedido_repository.lock().await;
        pedido_repository.cadastrar_cliente(pedido_id, cliente).await
    }

    pub async fn lista_lanches(&self) -> Result<Vec<Produto>, DomainError> {
        let produtos_repository = self.produto_repository.lock().await;
        produtos_repository
            .get_produtos_by_categoria(Categoria::Lanche)
            .await
    }

    pub async fn adicionar_lanche_com_personalizacao(
        &self,
        pedido_id: usize,
        lanche_id: usize,
    ) -> Result<Pedido, DomainError> {
        let produto_repository = self.produto_repository.lock().await;
        let lanche = produto_repository.get_produto_by_id(lanche_id).await?;
        drop(produto_repository);
        let mut pedido_repository = self.pedido_repository.lock().await;
        pedido_repository.cadastrar_lanche(pedido_id, lanche).await
    }

    pub async fn lista_acompanhamentos(&self) -> Result<Vec<Produto>, DomainError> {
        let produtos_repository = self.produto_repository.lock().await;
        produtos_repository
            .get_produtos_by_categoria(Categoria::Acompanhamento)
            .await
    }

    pub async fn adicionar_acompanhamento(
        &self,
        pedido_id: usize,
        acompanhamento_id: usize,
    ) -> Result<Pedido, DomainError> {
        let produto_repository = self.produto_repository.lock().await;
        let acompanhamento = produto_repository
            .get_produto_by_id(acompanhamento_id)
            .await?;
        drop(produto_repository);
        let mut pedido_repository = self.pedido_repository.lock().await;
        pedido_repository
            .cadastrar_acompanhamento(pedido_id, acompanhamento)
            .await
    }

    pub async fn lista_bebidas(&self) -> Result<Vec<Produto>, DomainError> {
        let produtos_repository = self.produto_repository.lock().await;
        produtos_repository
            .get_produtos_by_categoria(Categoria::Bebida)
            .await
    }

    pub async fn adicionar_bebida(
        &self,
        pedido_id: usize,
        bebida_id: usize,
    ) -> Result<Pedido, DomainError> {
        let produto_repository = self.produto_repository.lock().await;
        let bebida = produto_repository.get_produto_by_id(bebida_id).await?;
        drop(produto_repository);
        let mut pedido_repository = self.pedido_repository.lock().await;
        pedido_repository.cadastrar_bebida(pedido_id, bebida).await
    }

    pub async fn realizar_pagamento_do_pedido(
        &self,
        pedido_id: usize,
    ) -> Result<Pedido, DomainError> {
        let mut pedido_repository = self.pedido_repository.lock().await;
        let pagamento_adapter = self.pagamento_adapter.lock().await;

        let pedido: Pedido = pedido_repository.get_pedido_by_id(pedido_id).await?;
        let total_pedido = pedido.get_total_valor_pedido();

        let status_pagamento = pagamento_adapter.processa_pagamento(pedido_id, total_pedido)?;

        if status_pagamento == StatusPagamento::Successo {
            pedido_repository
                .atualiza_status(pedido_id, Status::Recebido)
                .await?;
        } else {
            Err(DomainError::Invalid("Pagamento não realizado".to_string()))?;
        }

        Ok(pedido)
    }
}

unsafe impl Send for PedidosEPagamentosUseCase {}
unsafe impl Sync for PedidosEPagamentosUseCase {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::application::ports::pagamento_port::MockPagamentoPort;
    use crate::core::domain::entities::{cliente::Cliente, pedido::Pedido};
    use crate::core::domain::repositories::{
        cliente_repository::MockClienteRepository, pedido_repository::MockPedidoRepository,
        produto_repository::MockProdutoRepository,
    };
    use crate::core::domain::value_objects::{cpf::Cpf, ingredientes::Ingredientes};
    use mockall::predicate::eq;
    use rocket::futures::lock::Mutex;
    use std::sync::Arc;
    use tokio;

    #[tokio::test]
    async fn test_lista_pedidos() {
        let mut mock = MockPedidoRepository::new();

        let returned_pedido = Pedido::new(
            1,
            None,
            None,
            None,
            None,
            "id_pagamento".to_string(),
            Status::Recebido,
            "2021-10-10".to_string(),
            "2021-10-10".to_string(),
        );

        let expected_pedido = returned_pedido.clone();

        mock.expect_lista_pedidos()
            .times(1)
            .returning(move || Ok(vec![returned_pedido.clone()]));

        let use_case = PedidosEPagamentosUseCase::new(
            Arc::new(Mutex::new(mock)),
            Arc::new(Mutex::new(MockClienteRepository::new())),
            Arc::new(Mutex::new(MockProdutoRepository::new())),
            Arc::new(Mutex::new(MockPagamentoPort::new())),
        );
        let result = use_case.lista_pedidos().await;
        assert_eq!(result.unwrap()[0].id(), expected_pedido.id());
    }

    #[tokio::test]
    async fn test_seleciona_pedido_por_id() {
        let mut mock = MockPedidoRepository::new();

        let returned_pedido = Pedido::new(
            1,
            None,
            None,
            None,
            None,
            "id_pagamento".to_string(),
            Status::Recebido,
            "2021-10-10".to_string(),
            "2021-10-10".to_string(),
        );

        let expected_pedido = returned_pedido.clone();

        mock.expect_get_pedido_by_id()
            .times(1)
            .returning(move |_| Ok(returned_pedido.clone()));

        let use_case = PedidosEPagamentosUseCase::new(
            Arc::new(Mutex::new(mock)),
            Arc::new(Mutex::new(MockClienteRepository::new())),
            Arc::new(Mutex::new(MockProdutoRepository::new())),
            Arc::new(Mutex::new(MockPagamentoPort::new())),
        );
        let result = use_case.seleciona_pedido_por_id(1).await;
        assert_eq!(result.unwrap().id(), expected_pedido.id());
    }

    #[tokio::test]
    async fn test_novo_pedido() {
        let mut mock_pedido_repository = MockPedidoRepository::new();
        let mut mock_cliente_repository = MockClienteRepository::new();

        let returned_cliente = Cliente::new(
            1,
            "nome".to_string(),
            "email".to_string(),
            Cpf::new("000.000.000-00".to_string()).unwrap(),
            "2021-10-10".to_string(),
            "2021-10-10".to_string(),
        );

        let returned_pedido = Pedido::new(
            1,
            Some(returned_cliente.clone()),
            None,
            None,
            None,
            "id_pagamento".to_string(),
            Status::Recebido,
            "2021-10-10".to_string(),
            "2021-10-10".to_string(),
        );

        let expected_pedido = returned_pedido.clone();

        mock_pedido_repository
            .expect_create_pedido()
            .times(1)
            .returning(move |_| Ok(returned_pedido.clone()));

        mock_cliente_repository
            .expect_get_cliente_by_id()
            .times(1)
            .returning(move |_| Ok(returned_cliente.clone()));

        let use_case = PedidosEPagamentosUseCase::new(
            Arc::new(Mutex::new(mock_pedido_repository)),
            Arc::new(Mutex::new(mock_cliente_repository)),
            Arc::new(Mutex::new(MockProdutoRepository::new())),
            Arc::new(Mutex::new(MockPagamentoPort::new())),
        );
        let result = use_case.novo_pedido(CreatePedidoInput {
            cliente_id: Some(1), lanche_id: None, acompanhamento_id: None, bebida_id: None,
        }).await;
        assert_eq!(result.unwrap().id(), expected_pedido.id());
    }

    #[tokio::test]
    async fn test_lista_lanches() {
        let mut mock = MockProdutoRepository::new();

        let ingredientes = Ingredientes::new(vec![
            "Pão".to_string(),
            "Hambúrguer".to_string(),
            "Queijo".to_string(),
        ])
        .unwrap();

        let returned_produto = Produto::new(
            1,
            "X-Bacon".to_string(),
            "foto.png".to_string(),
            "Saundiche de queijo e bacon".to_string(),
            Categoria::Lanche,
            10.0,
            ingredientes,
            "2021-10-10".to_string(),
            "2021-10-10".to_string(),
        );

        let expected_produto = returned_produto.clone();

        mock.expect_get_produtos_by_categoria()
            .times(1)
            .with(eq(Categoria::Lanche))
            .returning(move |_| Ok(vec![returned_produto.clone()]));

        let use_case = PedidosEPagamentosUseCase::new(
            Arc::new(Mutex::new(MockPedidoRepository::new())),
            Arc::new(Mutex::new(MockClienteRepository::new())),
            Arc::new(Mutex::new(mock)),
            Arc::new(Mutex::new(MockPagamentoPort::new())),
        );
        let result = use_case.lista_lanches().await;
        assert_eq!(result.unwrap()[0].id(), expected_produto.id());
    }

    #[tokio::test]
    async fn test_adicionar_lanche_com_personalizacao() {
        let mut mock_produto_repository = MockProdutoRepository::new();

        let mut mock_pedido_repository = MockPedidoRepository::new();

        let ingredientes = Ingredientes::new(vec![
            "Pão".to_string(),
            "Hambúrguer".to_string(),
            "Queijo".to_string(),
        ])
        .unwrap();

        let returned_produto = Produto::new(
            1,
            "X-Bacon".to_string(),
            "foto.png".to_string(),
            "Saundiche de queijo e bacon".to_string(),
            Categoria::Lanche,
            10.0,
            ingredientes,
            "2021-10-10".to_string(),
            "2021-10-10".to_string(),
        );

        let returned_pedido = Pedido::new(
            1,
            None,
            Some(returned_produto.clone()),
            None,
            None,
            "id_pagamento".to_string(),
            Status::Recebido,
            "2021-10-10".to_string(),
            "2021-10-10".to_string(),
        );

        let expected_pedido = returned_pedido.clone();

        mock_produto_repository
            .expect_get_produto_by_id()
            .times(1)
            .returning(move |_| Ok(returned_produto.clone()));

        mock_pedido_repository
            .expect_cadastrar_lanche()
            .times(1)
            .returning(move |_, _| Ok(returned_pedido.clone()));

        let use_case = PedidosEPagamentosUseCase::new(
            Arc::new(Mutex::new(mock_pedido_repository)),
            Arc::new(Mutex::new(MockClienteRepository::new())),
            Arc::new(Mutex::new(mock_produto_repository)),
            Arc::new(Mutex::new(MockPagamentoPort::new())),
        );
        let result = use_case.adicionar_lanche_com_personalizacao(1, 1).await;
        assert_eq!(result.unwrap().id(), expected_pedido.id());
    }

    #[tokio::test]
    async fn test_lista_acompanhamentos() {
        let mut mock = MockProdutoRepository::new();

        let ingredientes = Ingredientes::new(vec![]).unwrap();

        let returned_produto = Produto::new(
            1,
            "Batata Frita M".to_string(),
            "foto.png".to_string(),
            "Batata frita do tamanho médio".to_string(),
            Categoria::Acompanhamento,
            10.0,
            ingredientes,
            "2021-10-10".to_string(),
            "2021-10-10".to_string(),
        );

        let expected_produto = returned_produto.clone();

        mock.expect_get_produtos_by_categoria()
            .times(1)
            .with(eq(Categoria::Acompanhamento))
            .returning(move |_| Ok(vec![returned_produto.clone()]));

        let use_case = PedidosEPagamentosUseCase::new(
            Arc::new(Mutex::new(MockPedidoRepository::new())),
            Arc::new(Mutex::new(MockClienteRepository::new())),
            Arc::new(Mutex::new(mock)),
            Arc::new(Mutex::new(MockPagamentoPort::new())),
        );
        let result = use_case.lista_acompanhamentos().await;
        assert_eq!(result.unwrap()[0].id(), expected_produto.id());
    }

    #[tokio::test]
    async fn test_adicionar_acompanhamento() {
        let mut mock_produto_repository = MockProdutoRepository::new();

        let mut mock_pedido_repository = MockPedidoRepository::new();

        let ingredientes = Ingredientes::new(vec![]).unwrap();

        let returned_produto = Produto::new(
            1,
            "Batata Frita M".to_string(),
            "foto.png".to_string(),
            "Batata frita do tamanho médio".to_string(),
            Categoria::Acompanhamento,
            10.0,
            ingredientes,
            "2021-10-10".to_string(),
            "2021-10-10".to_string(),
        );

        let returned_pedido = Pedido::new(
            1,
            None,
            Some(returned_produto.clone()),
            None,
            None,
            "id_pagamento".to_string(),
            Status::Recebido,
            "2021-10-10".to_string(),
            "2021-10-10".to_string(),
        );

        let expected_pedido = returned_pedido.clone();

        mock_produto_repository
            .expect_get_produto_by_id()
            .times(1)
            .returning(move |_| Ok(returned_produto.clone()));

        mock_pedido_repository
            .expect_cadastrar_acompanhamento()
            .times(1)
            .returning(move |_, _| Ok(returned_pedido.clone()));

        let use_case = PedidosEPagamentosUseCase::new(
            Arc::new(Mutex::new(mock_pedido_repository)),
            Arc::new(Mutex::new(MockClienteRepository::new())),
            Arc::new(Mutex::new(mock_produto_repository)),
            Arc::new(Mutex::new(MockPagamentoPort::new())),
        );

        let result = use_case.adicionar_acompanhamento(1, 1).await;
        assert_eq!(result.unwrap().id(), expected_pedido.id());
    }

    #[tokio::test]
    async fn test_lista_bebidas() {
        let mut mock = MockProdutoRepository::new();

        let ingredientes = Ingredientes::new(vec![]).unwrap();

        let returned_produto = Produto::new(
            1,
            "Refrigerante de Cola M".to_string(),
            "foto.png".to_string(),
            "Refrigerante de Cola do tamanho médio".to_string(),
            Categoria::Bebida,
            10.0,
            ingredientes,
            "2021-10-10".to_string(),
            "2021-10-10".to_string(),
        );

        let expected_produto = returned_produto.clone();

        mock.expect_get_produtos_by_categoria()
            .times(1)
            .with(eq(Categoria::Bebida))
            .returning(move |_| Ok(vec![returned_produto.clone()]));

        let use_case = PedidosEPagamentosUseCase::new(
            Arc::new(Mutex::new(MockPedidoRepository::new())),
            Arc::new(Mutex::new(MockClienteRepository::new())),
            Arc::new(Mutex::new(mock)),
            Arc::new(Mutex::new(MockPagamentoPort::new())),
        );

        let result = use_case.lista_bebidas().await;
        assert_eq!(result.unwrap()[0].id(), expected_produto.id());
    }

    #[tokio::test]
    async fn test_adicionar_bebida() {
        let mut mock_produto_repository = MockProdutoRepository::new();

        let mut mock_pedido_repository = MockPedidoRepository::new();

        let ingredientes = Ingredientes::new(vec![]).unwrap();

        let returned_produto = Produto::new(
            1,
            "Refrigerante de Cola M".to_string(),
            "foto.png".to_string(),
            "Refrigerante de Cola do tamanho médio".to_string(),
            Categoria::Bebida,
            10.0,
            ingredientes,
            "2021-10-10".to_string(),
            "2021-10-10".to_string(),
        );

        let returned_pedido = Pedido::new(
            1,
            None,
            Some(returned_produto.clone()),
            None,
            None,
            "id_pagamento".to_string(),
            Status::Recebido,
            "2021-10-10".to_string(),
            "2021-10-10".to_string(),
        );

        let expected_pedido = returned_pedido.clone();

        mock_produto_repository
            .expect_get_produto_by_id()
            .times(1)
            .returning(move |_| Ok(returned_produto.clone()));

        mock_pedido_repository
            .expect_cadastrar_bebida()
            .times(1)
            .returning(move |_, _| Ok(returned_pedido.clone()));

        let use_case = PedidosEPagamentosUseCase::new(
            Arc::new(Mutex::new(mock_pedido_repository)),
            Arc::new(Mutex::new(MockClienteRepository::new())),
            Arc::new(Mutex::new(mock_produto_repository)),
            Arc::new(Mutex::new(MockPagamentoPort::new())),
        );

        let result = use_case.adicionar_bebida(1, 1).await;
        assert_eq!(result.unwrap().id(), expected_pedido.id());
    }

    #[tokio::test]
    async fn test_realizar_pagamento_do_pedido() {
        let mut mock = MockPedidoRepository::new();
        let mut mock_pagamento = MockPagamentoPort::new();

        let ingredientes = Ingredientes::new(vec![]).unwrap();

        let bebida = Produto::new(
            1,
            "Refrigerante de Cola M".to_string(),
            "foto.png".to_string(),
            "Refrigerante de Cola do tamanho médio".to_string(),
            Categoria::Bebida,
            10.0,
            ingredientes,
            "2021-10-10".to_string(),
            "2021-10-10".to_string(),
        );

        let returned_pedido = Pedido::new(
            1,
            None,
            None,
            None,
            Some(bebida.clone()),
            "id_pagamento".to_string(),
            Status::Pendente,
            "2021-10-10".to_string(),
            "2021-10-10".to_string(),
        );

        let mut updated_pedido = returned_pedido.clone();
        updated_pedido.set_status(Status::Recebido);

        let expected_pedido = updated_pedido.clone();

        mock.expect_get_pedido_by_id()
            .times(1)
            .returning(move |_| Ok(returned_pedido.clone()));

        mock_pagamento
            .expect_processa_pagamento()
            .times(1)
            .with(eq(1), eq(10.0))
            .returning(move |_, _| Ok(StatusPagamento::Successo));

        mock.expect_atualiza_status()
            .times(1)
            .returning(move |_, _| Ok(updated_pedido.clone()));

        let use_case = PedidosEPagamentosUseCase::new(
            Arc::new(Mutex::new(mock)),
            Arc::new(Mutex::new(MockClienteRepository::new())),
            Arc::new(Mutex::new(MockProdutoRepository::new())),
            Arc::new(Mutex::new(mock_pagamento)),
        );
        let result = use_case.realizar_pagamento_do_pedido(1).await;
        assert_eq!(result.unwrap().id(), expected_pedido.id());
    }
}
