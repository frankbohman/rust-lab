use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct HttpConfig {
  pub endpoint: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct AppConfig {
  pub http: HttpConfig,
}
