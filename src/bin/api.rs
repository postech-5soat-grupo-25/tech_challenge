use rust_ddd_template::adapter::api;

fn main() {
  match api::server::main() {
    Ok(_) => println!("Server finished"),
    Err(e) => println!("Error: {}", e),
  };
}
