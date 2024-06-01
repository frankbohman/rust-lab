use shared::axum::body::Body;
use shared::axum::http::Request;
use shared::axum::routing::get;
use shared::axum::Router;
use shared::tokio;
use shared::tokio::net::TcpListener;
use shared::tower::ServiceBuilder;
use shared::tower_http::trace::TraceLayer;
use shared::tracing::{self, field, Span};
use shared::tracing::{info, info_span, trace_span};

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

  let app = Router::new()
    .route("/", get(handler))
    .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http().make_span_with(make_span)));

  shared::axum::serve(
    TcpListener::bind(config.http.endpoint).await?,
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

fn make_span(request: &Request<Body>) -> Span {
  let headers = request.headers();

  let path = request.uri().path();

  // Disable (well, silence) spans/traces for root spans.
  if path.is_empty() || path == "/" {
    trace_span!("incoming request", path, ?headers, trace_id = field::Empty)
  } else {
    info_span!("incoming request", path, ?headers, trace_id = field::Empty)
  }
}
