use shared::axum::routing::get;
use shared::axum::Router;
use shared::tokio;
use shared::tonic;
use shared::toolbox::health::CustomHealthService;
use shared::tracing;
use shared::tracing::info;
use shared::
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

  let health = CustomHealthService::new(SERVICE_NAME);

  let grpc_service: shared::toolbox::health::HealthServer<CustomHealthService> =
    shared::proto::health::v1::health_server::HealthServer::new(health.clone());

  let grpc_addr: std::net::SocketAddr = config.grpc.endpoint.parse().unwrap();

  // to multiplex or not to multiplex (with tower), thats the question
  tokio::spawn(async move {
    Server::builder()
      .add_service(grpc_service)
      .serve(grpc_addr)
      .await
      .expect("gRPC server failed");
  });

  let app = Router::new().route("/", get(handler));
  shared::axum::serve(
    tokio::net::TcpListener::bind(config.web.endpoint).await?,
    app.into_make_service(),
  )
  .await?;

  Ok(())
}
#[tracing::instrument]
async fn handler() -> &'static str {
  // ...
  // info!("Hello....");
  "Hello, World!"
}
