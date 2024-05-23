use shared::axum::routing::get;
use shared::axum::Router;
use shared::tokio;
use shared::tonic;
use shared::toolbox::health::HealthService;
use shared::tracing;
use tonic::transport::Server;

type Result<T> = std::result::Result<T, anyhow::Error>;

const SERVICE_VERSION: &str = env!("CARGO_PKG_VERSION");
const SERVICE_NAME: &str = env!("CARGO_PKG_NAME");

#[tokio::main]
async fn main() -> Result<()> {
  let deployment_environment = std::env::var("DEPLOYMENT_ENVIRONMENT").unwrap_or("development".to_string());
  println!(
    "{}::{}::{}",
    deployment_environment, SERVICE_NAME, SERVICE_VERSION
  );
  shared::toolbox::telemetry::bootstrap(SERVICE_NAME, SERVICE_VERSION, &deployment_environment)?;

  let grpc_addr = "127.0.0.1:50052".parse().unwrap();

  let grpc_service = shared::proto::health::v1::health_server::HealthServer::new(HealthService::default());

  tokio::spawn(async move {
    Server::builder()
      .add_service(grpc_service)
      .serve(grpc_addr)
      .await
      .expect("gRPC server failed");
  });

  let app = Router::new().route("/", get(handler));
  shared::axum::serve(
    tokio::net::TcpListener::bind("127.0.0.1:8080").await?,
    app.into_make_service(),
  )
  .await?;

  Ok(())
}
#[tracing::instrument]
async fn handler() -> &'static str {
  // ...
  "Hello, World!"
}
