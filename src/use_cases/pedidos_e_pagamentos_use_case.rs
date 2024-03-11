use crate::base::domain_error::DomainError;
use crate::entities::pagamento;
use crate::entities::{
    pagamento::Pagamento,
    pedido::{Pedido, Status},
    produto::{Categoria, Produto},
};
use crate::traits::{
    cliente_gateway::ClienteGateway, pagamento_gateway::PagamentoGateway,
    pedido_gateway::PedidoGateway, produto_gateway::ProdutoGateway,
};

use crate::adapters::mercadopago_pagamento_webhook_adapter::MercadoPagoPagamentoWebhookAdapter;
use crate::traits::pagamento_webhook_adapter::PagamentoWebhookAdapter;
use chrono::Utc;
use rocket::serde::json::Value;
use schemars::JsonSchema;
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone, Debug, Deserialize, JsonSchema)]
pub struct CreatePedidoInput {
    cliente_id: Option<usize>,
    lanche_id: Option<usize>,
    acompanhamento_id: Option<usize>,
    bebida_id: Option<usize>,
}

#[derive(Clone)]
pub struct PedidosEPagamentosUseCase {
    pedido_repository: Arc<Mutex<dyn PedidoGateway + Sync + Send>>,
    cliente_repository: Arc<Mutex<dyn ClienteGateway + Sync + Send>>,
    produto_repository: Arc<Mutex<dyn ProdutoGateway + Sync + Send>>,
    pagamento_repository: Arc<Mutex<dyn PagamentoGateway + Sync + Send>>,
}

impl PedidosEPagamentosUseCase {
    pub fn new(
        pedido_repository: Arc<Mutex<dyn PedidoGateway + Sync + Send>>,
        cliente_repository: Arc<Mutex<dyn ClienteGateway + Sync + Send>>,
        produto_repository: Arc<Mutex<dyn ProdutoGateway + Sync + Send>>,
        pagamento_repository: Arc<Mutex<dyn PagamentoGateway + Sync + Send>>,
    ) -> Self {
        PedidosEPagamentosUseCase {
            pedido_repository,
            cliente_repository,
            produto_repository,
            pagamento_repository,
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

    pub async fn novo_pedido(
        &self,
        pedido_input: CreatePedidoInput,
    ) -> Result<Pedido, DomainError> {
        let cliente = if let Some(cliente_id) = pedido_input.cliente_id {
            let cliente_repo = self.cliente_repository.lock().await;
            Some(cliente_repo.get_cliente_by_id(cliente_id).await?)
        } else {
            None
        };

        let lanche = if let Some(lanche_id) = pedido_input.lanche_id {
            let produto_repo = self.produto_repository.lock().await;
            Some(produto_repo.get_produto_by_id(lanche_id).await?)
        } else {
            None
        };

        let bebida = if let Some(bebida_id) = pedido_input.bebida_id {
            let produto_repo = self.produto_repository.lock().await;
            Some(produto_repo.get_produto_by_id(bebida_id).await?)
        } else {
            None
        };

        let acompanhamento = if let Some(acompanhamento_id) = pedido_input.acompanhamento_id {
            let produto_repo = self.produto_repository.lock().await;
            Some(produto_repo.get_produto_by_id(acompanhamento_id).await?)
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
            _now.clone(),
        );

        let mut pedido_repository = self.pedido_repository.lock().await;
        let novo_pedido = pedido_repository.create_pedido(pedido).await;

        match novo_pedido {
            Ok(pedido) => {
                self
                    .criar_pagamento_do_pedido(pedido.id().clone())
                    .await?;
                Ok(pedido)
            }
            Err(err) => Err(err),
        }
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
        pedido_repository
            .cadastrar_cliente(pedido_id, cliente)
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
        if lanche.categoria().clone() != Categoria::Lanche {
            Err(DomainError::Invalid("Produto não é um lanche".to_string()))?;
        }
        let mut pedido_repository = self.pedido_repository.lock().await;
        pedido_repository.cadastrar_lanche(pedido_id, lanche).await
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
        if acompanhamento.categoria().clone() != Categoria::Acompanhamento {
            Err(DomainError::Invalid(
                "Produto não é um acompanhamento".to_string(),
            ))?;
        }
        let mut pedido_repository = self.pedido_repository.lock().await;
        pedido_repository
            .cadastrar_acompanhamento(pedido_id, acompanhamento)
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
        if bebida.categoria().clone() != Categoria::Bebida {
            Err(DomainError::Invalid("Produto não é uma bebida".to_string()))?;
        }
        let mut pedido_repository = self.pedido_repository.lock().await;
        pedido_repository.cadastrar_bebida(pedido_id, bebida).await
    }

    pub async fn get_pagamento_by_pedido_id(
        &self,
        pedido_id: usize,
    ) -> Result<Pagamento, DomainError> {
        let mut pagamento_repository = self.pagamento_repository.lock().await;
        pagamento_repository
            .get_pagamento_by_id_pedido(pedido_id)
            .await

    }

    pub async fn criar_pagamento_do_pedido(
        &self,
        pedido_id: usize,
    ) -> Result<Pagamento, DomainError> {
        let pedido_repository = self.pedido_repository.lock().await;
        let mut pagamento_repository = self.pagamento_repository.lock().await;

        let pedido: Pedido = pedido_repository.get_pedido_by_id(pedido_id).await?;
        let _now = Utc::now().format("%Y-%m-%d %H:%M:%S%.3f%z").to_string();
        let pagamento = Pagamento::new(
            0,
            pedido.id().clone(),
            String::from("pendente"),
            pedido.valor(),
            pedido.pagamento().clone(),
            String::from(""),
            _now.clone(),
        );
        let mut pagamento = pagamento_repository.create_pagamento(pagamento).await?;

        // set webhook
        match pagamento.metodo().as_str() {
            "Mercado Pago" => {
                let mercado_pago_adapter: Arc<dyn PagamentoWebhookAdapter + Sync + Send> =
                    Arc::new(MercadoPagoPagamentoWebhookAdapter::new());

                match mercado_pago_adapter.set_webhook_pagamento(pagamento.clone()).await {
                    Ok(pagamento_updated) => {
                        println!("Webhook set successfully");
                        let updated_pagamento = pagamento_repository
                            .update_pagamento(pagamento_updated.clone())
                            .await?;
                        Ok(updated_pagamento)
                    }
                    Err(err) => {
                        eprintln!("Failed to set webhook");
                        pagamento.set_estado(String::from("cancelado"));
                        pagamento_repository
                            .update_pagamento(pagamento.clone())
                            .await?;
                        Err(DomainError::Invalid("Internal server error".to_string()))
                    }
                }
            }
            _ => {
                println!("Metodo de pagamento invalido");
                Err(DomainError::Invalid("Internal server error".to_string()))
            }
        }
    }

    pub async fn webhook_pagamento(
        &self,
        pedido_id: usize,
        data_pagamento: Value,
    ) -> Result<Pagamento, DomainError> {
        let mut pedido_repository = self.pedido_repository.lock().await;
        let mut pagamento_repository = self.pagamento_repository.lock().await;
        let pedido: Pedido = pedido_repository.get_pedido_by_id(pedido_id).await?;
        let mut pagamento: Pagamento = pagamento_repository
            .get_pagamento_by_id_pedido(pedido_id)
            .await?;
        match pedido.pagamento().as_str() {
            "Mercado Pago" => {
                let mercado_pago_adapter: Arc<dyn PagamentoWebhookAdapter + Sync + Send> =
                    Arc::new(MercadoPagoPagamentoWebhookAdapter::new());

                pagamento = mercado_pago_adapter.processa_webhook(data_pagamento, pagamento);
                pagamento_repository
                    .update_pagamento(pagamento.clone())
                    .await?;
                println!("{}", *pagamento.estado());
                if *pagamento.estado() == String::from("aprovado")
                    && *pedido.status() == Status::Pendente
                {
                    pedido_repository
                        .atualiza_status(*pedido.id(), Status::Pago)
                        .await?;
                }
                Ok(pagamento)
            }
            _ => {
                println!("Metodo de pagamento invalido");
                Err(DomainError::Invalid("Internal server error".to_string()))
            }
        }
    }
}

unsafe impl Send for PedidosEPagamentosUseCase {}
unsafe impl Sync for PedidosEPagamentosUseCase {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::{cliente::Cliente, cpf::Cpf, ingredientes::Ingredientes, pedido::Pedido};
    use crate::traits::{
        cliente_gateway::MockClienteGateway, pagamento_gateway::MockPagamentoGateway,
        pedido_gateway::MockPedidoGateway, produto_gateway::MockProdutoGateway,
    };
    use mockall::predicate::eq;
    use tokio::sync::Mutex;
    use std::sync::Arc;
    use tokio;

    #[tokio::test]
    async fn test_lista_pedidos() {
        let mut mock = MockPedidoGateway::new();

        let returned_pedido = Pedido::new(
            1,
            None,
            None,
            None,
            None,
            "id_pagamento".to_string(),
            Status::Pendente,
            "2021-10-10".to_string(),
            "2021-10-10".to_string(),
        );

        let expected_pedido = returned_pedido.clone();

        mock.expect_lista_pedidos()
            .times(1)
            .returning(move || Ok(vec![returned_pedido.clone()]));

        let use_case = PedidosEPagamentosUseCase::new(
            Arc::new(Mutex::new(mock)),
            Arc::new(Mutex::new(MockClienteGateway::new())),
            Arc::new(Mutex::new(MockProdutoGateway::new())),
            Arc::new(Mutex::new(MockPagamentoGateway::new())),
        );
        let result = use_case.lista_pedidos().await;
        assert_eq!(result.unwrap()[0].id(), expected_pedido.id());
    }

    #[tokio::test]
    async fn test_seleciona_pedido_por_id() {
        let mut mock = MockPedidoGateway::new();

        let returned_pedido = Pedido::new(
            1,
            None,
            None,
            None,
            None,
            "id_pagamento".to_string(),
            Status::Pendente,
            "2021-10-10".to_string(),
            "2021-10-10".to_string(),
        );

        let expected_pedido = returned_pedido.clone();

        mock.expect_get_pedido_by_id()
            .times(1)
            .returning(move |_| Ok(returned_pedido.clone()));

        let use_case = PedidosEPagamentosUseCase::new(
            Arc::new(Mutex::new(mock)),
            Arc::new(Mutex::new(MockClienteGateway::new())),
            Arc::new(Mutex::new(MockProdutoGateway::new())),
            Arc::new(Mutex::new(MockPagamentoGateway::new())),
        );
        let result = use_case.seleciona_pedido_por_id(1).await;
        assert_eq!(result.unwrap().id(), expected_pedido.id());
    }

    #[tokio::test]
    async fn test_novo_pedido() {
        let mut mock_pedido_repository = MockPedidoGateway::new();
        let mut mock_cliente_repository = MockClienteGateway::new();

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
            Status::Pendente,
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
            Arc::new(Mutex::new(MockProdutoGateway::new())),
            Arc::new(Mutex::new(MockPagamentoGateway::new())),
        );
        let result = use_case
            .novo_pedido(CreatePedidoInput {
                cliente_id: Some(1),
                lanche_id: None,
                acompanhamento_id: None,
                bebida_id: None,
            })
            .await;
        assert_eq!(result.unwrap().id(), expected_pedido.id());
    }

    #[tokio::test]
    async fn test_lista_lanches() {
        let mut mock = MockProdutoGateway::new();

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
            Arc::new(Mutex::new(MockPedidoGateway::new())),
            Arc::new(Mutex::new(MockClienteGateway::new())),
            Arc::new(Mutex::new(mock)),
            Arc::new(Mutex::new(MockPagamentoGateway::new())),
        );
        let result = use_case.lista_lanches().await;
        assert_eq!(result.unwrap()[0].id(), expected_produto.id());
    }

    #[tokio::test]
    async fn test_adicionar_lanche_com_personalizacao() {
        let mut mock_produto_repository = MockProdutoGateway::new();

        let mut mock_pedido_repository = MockPedidoGateway::new();

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
            Status::Pendente,
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
            Arc::new(Mutex::new(MockClienteGateway::new())),
            Arc::new(Mutex::new(mock_produto_repository)),
            Arc::new(Mutex::new(MockPagamentoGateway::new())),
        );
        let result = use_case.adicionar_lanche_com_personalizacao(1, 1).await;
        assert_eq!(result.unwrap().id(), expected_pedido.id());
    }

    #[tokio::test]
    async fn test_lista_acompanhamentos() {
        let mut mock = MockProdutoGateway::new();

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
            Arc::new(Mutex::new(MockPedidoGateway::new())),
            Arc::new(Mutex::new(MockClienteGateway::new())),
            Arc::new(Mutex::new(mock)),
            Arc::new(Mutex::new(MockPagamentoGateway::new())),
        );
        let result = use_case.lista_acompanhamentos().await;
        assert_eq!(result.unwrap()[0].id(), expected_produto.id());
    }

    #[tokio::test]
    async fn test_adicionar_acompanhamento() {
        let mut mock_produto_repository = MockProdutoGateway::new();

        let mut mock_pedido_repository = MockPedidoGateway::new();

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
            Status::Pendente,
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
            Arc::new(Mutex::new(MockClienteGateway::new())),
            Arc::new(Mutex::new(mock_produto_repository)),
            Arc::new(Mutex::new(MockPagamentoGateway::new())),
        );

        let result = use_case.adicionar_acompanhamento(1, 1).await;
        assert_eq!(result.unwrap().id(), expected_pedido.id());
    }

    #[tokio::test]
    async fn test_lista_bebidas() {
        let mut mock = MockProdutoGateway::new();

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
            Arc::new(Mutex::new(MockPedidoGateway::new())),
            Arc::new(Mutex::new(MockClienteGateway::new())),
            Arc::new(Mutex::new(mock)),
            Arc::new(Mutex::new(MockPagamentoGateway::new())),
        );

        let result = use_case.lista_bebidas().await;
        assert_eq!(result.unwrap()[0].id(), expected_produto.id());
    }

    #[tokio::test]
    async fn test_adicionar_bebida() {
        let mut mock_produto_repository = MockProdutoGateway::new();

        let mut mock_pedido_repository = MockPedidoGateway::new();

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
            Status::Pendente,
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
            Arc::new(Mutex::new(MockClienteGateway::new())),
            Arc::new(Mutex::new(mock_produto_repository)),
            Arc::new(Mutex::new(MockPagamentoGateway::new())),
        );

        let result = use_case.adicionar_bebida(1, 1).await;
        assert_eq!(result.unwrap().id(), expected_pedido.id());
    }

}
