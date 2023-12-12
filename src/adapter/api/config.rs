use std::{env, str::FromStr};

#[derive(Debug, PartialEq)]
pub enum Env {
  Dev,
  Prod,
  Test,
}

impl FromStr for Env {

  type Err = ();

  fn from_str(input: &str) -> Result<Env, Self::Err> {
      match input {
          "dev" => Ok(Env::Dev),
          "prod" => Ok(Env::Prod),
          "test" => Ok(Env::Test),
          _ => Err(()),
      }
  }
}

impl ToString for Env {
  fn to_string(&self) -> String {
      match self {
          Env::Dev => "dev".to_string(),
          Env::Prod => "prod".to_string(),
          Env::Test => "test".to_string(),
      }
  }
}

pub struct Config {
  pub secret: String,
  pub env: Env,
  pub db_url: String,
}

impl Config {
  pub fn build() -> Config {
      let secret = env::var("SECRET").unwrap_or("secret".to_string());
      let env = env::var("ENV").unwrap_or("dev".to_string());
      let env = Env::from_str(&env).unwrap_or(Env::Dev);
      let db_url = env::var("DB_URL").unwrap_or("postgres://postgres:postgres@localhost:5432/postgres".to_string());

      println!("Environment variables loaded successfully!");

      Config {
          secret,
          env,
          db_url,
      }
  }
}
