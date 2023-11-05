pub mod cpf;
pub mod endereco;
pub trait ValueObject {
    fn is_same_as(&self, other: &Self) -> bool;
}