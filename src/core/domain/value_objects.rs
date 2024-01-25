pub mod cpf;
pub mod ingredientes;
pub trait ValueObject {
    fn is_same_as(&self, other: &Self) -> bool;
}