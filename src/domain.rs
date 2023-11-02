mod base;
mod entities;
mod value_objects;

fn main() {
    println!("Hello, world!");
    let error = base::domain_exception::DomainError::new("message".to_string(), None);
}

