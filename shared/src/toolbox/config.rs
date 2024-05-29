use config::{Config, Environment, File};
use serde::de::DeserializeOwned;
use std::fmt::Error;

type Result<T> = std::result::Result<T, Error>;

pub async fn load<T: DeserializeOwned>(deployment_environment: Option<String>) -> Result<T> {
  let env = deployment_environment.unwrap_or("development".into());

  Config::builder()
    .add_source(File::with_name("./config/default"))
    .add_source(File::with_name(&format!("./config/{}", env)).required(false))
    .add_source(Environment::with_prefix("APP"))
    .build()
    .map_err(|_| Error)?
    .try_deserialize::<T>()
    .map_err(|_| Error)
}
