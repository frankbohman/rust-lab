use std::time::Duration;

use crud::CrudService;
use shared::axum::body::Body;

use shared::axum::http::Request;
use shared::crud::crud_server::CrudServer;
use shared::tokio;
use shared::tokio::signal::unix::signal;
use shared::tokio::signal::unix::SignalKind;
use shared::tokio::time;
use shared::tonic;
use state::State;
use tonic::transport::Server;
use tracing::field;
use tracing::info;
use tracing::info_span;
use tracing::Span;

type Result<T> = std::result::Result<T, anyhow::Error>;

const SERVICE_VERSION: &str = env!("CARGO_PKG_VERSION");
const SERVICE_NAME: &str = env!("CARGO_PKG_NAME");

mod config;
mod crud;
mod state;

#[tokio::main]
async fn main() -> Result<()> {
  let deployment_environment = std::env::var("DEPLOYMENT_ENVIRONMENT").unwrap_or("development".to_string());
  println!(
    "{}::{}::{}",
    deployment_environment, SERVICE_NAME, SERVICE_VERSION
  );
  shared::toolbox::telemetry::bootstrap(SERVICE_NAME, SERVICE_VERSION, &deployment_environment)?;

  // load config from ./config/default
  // then merge with ./config/<environment>
  let config = shared::toolbox::config::load::<config::AppConfig>(Some(deployment_environment)).await?;
  info!("{:?}", config);

  let (mut health_reporter, health_service) = shared::tonic_health::server::health_reporter();
  health_reporter.set_serving::<CrudServer<CrudService>>().await;

  let crud = CrudService::new(State::default());
  let crud = CrudServer::new(crud);

  let crud_reflection = shared::tonic_reflection::server::Builder::configure()
    .register_encoded_file_descriptor_set(shared::proto::crud::FILE_DESCRIPTOR_SET)
    .build()?;

  // grpcurl -plaintext 127.0.0.1:50052 describe crud.Crud
  // grpcurl -d '{"payload":{"message": "Heppåre!"}}' -import-path shared/proto -proto shared/proto/crud/crud.proto -plaintext 127.0.0.1:50051 crud.Crud/Create
  // grpcurl -d '{"id": "91632d47-b5de-42f6-85d8-1c6e58cf7845"}' -import-path shared/proto -proto shared/proto/crud/crud.proto -plaintext 127.0.0.1:50051 crud.Crud/Read
  Server::builder()
    .accept_http1(true)
    .add_service(shared::tonic_web::enable(crud.clone()))
    .add_service(health_service)
    .add_service(crud_reflection)
    .serve_with_shutdown(
      config.grpc.endpoint.parse()?,
      shutdown_signal(Some(Duration::from_secs(1))),
    )
    .await?;

  Ok(())
}

#[allow(dead_code)]
fn make_span(request: &Request<Body>) -> Span {
  let headers = request.headers();
  info_span!("incoming request", ?headers, trace_id = field::Empty)
}

async fn shutdown_signal(shutdown_timeout: Option<Duration>) {
  signal(SignalKind::terminate())
    .expect("install SIGTERM handler")
    .recv()
    .await;
  if let Some(shutdown_timeout) = shutdown_timeout {
    time::sleep(shutdown_timeout).await;
  }
}
