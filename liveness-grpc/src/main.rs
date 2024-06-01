use std::time::Duration;

use shared::axum::body::Body;
use shared::axum::http::Request;
use shared::tokio;
use shared::tokio::signal::unix::signal;
use shared::tokio::signal::unix::SignalKind;
use shared::tokio::time;
use shared::tonic;
use shared::toolbox::health::HealthService;

use shared::tracing::field;
use shared::tracing::info;
use shared::tracing::info_span;
use shared::tracing::Span;
use tonic::transport::Server;

type Result<T> = std::result::Result<T, anyhow::Error>;

const SERVICE_VERSION: &str = env!("CARGO_PKG_VERSION");
const SERVICE_NAME: &str = env!("CARGO_PKG_NAME");

mod config;

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

  let health = HealthService::new(SERVICE_NAME);

  let health_service: shared::toolbox::health::HealthServer<HealthService> =
    shared::proto::health::v1::health_server::HealthServer::new(health.clone());

  let web_service = shared::tonic_web::enable(health_service.clone());
  let addr: std::net::SocketAddr = config.grpc.endpoint.parse().unwrap();

  let reflection_service = shared::tonic_reflection::server::Builder::configure()
    .register_encoded_file_descriptor_set(shared::proto::health::v1::FILE_DESCRIPTOR_SET)
    .build()
    .unwrap();

  let app = Server::builder()
    .accept_http1(true)
    .add_service(web_service)
    .add_service(health_service)
    .add_service(reflection_service);

  app
    .serve_with_shutdown(addr, shutdown_signal(Some(Duration::from_secs(1))))
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
// let trace_layer = ServiceBuilder::new()
//   .layer(TraceLayer::new_for_grpc().make_span_with(make_span))
//   .map_request(accept_trace)
//   .map_request(record_trace_id);

// let trace_layer = ServiceBuilder::new().layer(TraceLayer::new_for_grpc().make_span_with(make_span));
// let trace_layer = ServiceBuilder::new().layer(TraceLayer::new_for_grpc());
