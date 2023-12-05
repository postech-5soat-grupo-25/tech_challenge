use rust_ddd_template::adapter::api;

fn main() {
  match api::server::main() {
    Ok(_) => println!("Server started"),
    Err(e) => println!("Error: {}", e),
  };
}
