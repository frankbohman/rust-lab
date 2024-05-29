use std::env;

use config::{Config, Environment, File};

type Result<T> = std::result::Result<T, anyhow::Error>;
pub async fn load() -> Result<()> {
  let env = env::var("DEPLOYMENT_ENVIRONMENT").unwrap_or_else(|_| "development".into());

  let s = Config::builder()
    .add_source(File::with_name("config/default"))
    .add_source(File::with_name(&format!("config/{}", env)).required(false))
    .add_source(Environment::with_prefix("APP"))
    .build()?;

  Ok(())
}
