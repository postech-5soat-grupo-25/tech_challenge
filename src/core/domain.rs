pub mod base;
pub mod entities;
pub mod value_objects;
pub mod repositories;

fn main() {
    println!("Hello, world!");
    let error = base::domain_exception::DomainError::new("message".to_string(), None);
}

