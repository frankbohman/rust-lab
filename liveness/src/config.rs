use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct GrpcConfig {
  pub endpoint: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct WebConfig {
  pub endpoint: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct AppConfig {
  pub grpc: GrpcConfig,
  pub web:  WebConfig,
}
