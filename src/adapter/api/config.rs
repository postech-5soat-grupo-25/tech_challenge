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
}

impl Config {
  pub fn build() -> Config {
      let secret = env::var("SECRET").unwrap_or("secret".to_string());
      let env = env::var("ENV").unwrap_or("dev".to_string());
      let env = Env::from_str(&env).unwrap_or(Env::Dev);

      Config {
          secret,
          env,
      }
  }
}
