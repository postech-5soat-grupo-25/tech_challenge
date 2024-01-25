use crate::core::domain::{
  base::domain_error::DomainError, 
  entities::{
    pedido::Pedido,
    produto::Produto,
  }
};

#[async_trait]
pub trait PedidoRepository {
  async fn create_pedido(&mut self, pedido: Pedido) -> Result<Pedido, DomainError>;

  async fn get_pedido_by_id(&self, pedido_id: usize) -> Result<Pedido, DomainError>;

  async fn cadastrar_acompanhamento(&mut self, pedido_id: usize, acompanhamento: Produto) -> Result<Pedido, DomainError>;
  
  async fn cadastrar_bebida(&mut self, pedido_id: usize, bebida: Produto) -> Result<Pedido, DomainError>;

  async fn cadastrar_pagamento(&mut self, pedido_id: usize, pagamento: String) -> Result<Pedido, DomainError>;

  async fn cadastrar_lanche(&mut self, pedido_id: usize, lanche: Produto) -> Result<Pedido, DomainError>;

  async fn update_pedido(&mut self, pedido_id: usize, pedido: Pedido) -> Result<Pedido, DomainError>;
}