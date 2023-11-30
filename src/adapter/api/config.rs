use std::env;
pub struct Config {
  pub secret: String,
}

impl Config {
  pub fn build() -> Config {
      let secret = env::var("SECRET").unwrap_or("secret".to_string());

      Config {
          secret,
      }
  }
}
