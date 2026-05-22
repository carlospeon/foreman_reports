use config::{Config, ConfigError};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Server {
  pub listen: String,
  pub concurency_limit: usize,
  pub loglevel: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Database {
  pub url: String,
  pub max_connections: u32,
  pub acquire_timeout: u64,
  pub materialized_view_ttl: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
 pub struct Ldap {
   pub url: String,
   pub domain: String,
   pub search: String,
   pub member_of: String,
 }

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Configuration {
  pub server: Server,
  pub database: Database,
  pub ldap: Ldap
}

impl Default for Configuration {
  fn default () -> Self {
    Self {
      server: Server {
        listen: "127.0.0.1:3000".to_string(),
        concurency_limit: 5,
        loglevel: "info".to_string()
      },
      database: Database {
        url: "postgres://postgres:password@localhost".to_string(),
        max_connections: 5,
        acquire_timeout: 5,
        materialized_view_ttl: 1,
      },
      ldap: Ldap {
        url: "ldap://localhost:3268".to_string(),
        domain: "local".to_string(),
        search: "DC=local".to_string(),
        member_of: "CN=group,DC=local".to_string()
      }
    }
  }
}

// const CONFIG_FILE_PATH: &str = "./configuration.toml";

impl Configuration {
  pub fn new(file: &str) -> Result<Self, ConfigError> {

    let default_config = Config::try_from(&Configuration::default())
      .expect("Serialization failed");
    let cfg_builder = Config::builder()
      .add_source(default_config)
      .add_source(config::File::with_name(file))
      .add_source(config::Environment::with_prefix("FR").separator("_"))
      .build()?;

    cfg_builder.try_deserialize()
  }

}
