use std::env;

use std::fmt::Error;

use config::{Config, Environment, File};
use serde::de::DeserializeOwned;

type Result<T> = std::result::Result<T, Error>;
pub fn load<T: DeserializeOwned>() -> Result<T> {
  let env = env::var("DEPLOYMENT_ENVIRONMENT").unwrap_or_else(|_| "development".into());

  let s = Config::builder()
    .add_source(File::with_name("config/default"))
    .add_source(File::with_name(&format!("config/{}", env)).required(false))
    .add_source(Environment::with_prefix("APP"))
    .build()
    .map_err(|_| Error)?;

  s.try_deserialize::<T>().map_err(|_| Error)
}
