use anyhow::Error;

pub async fn bootstrap() -> Result<(), Error> {
  env_logger::init();
}
