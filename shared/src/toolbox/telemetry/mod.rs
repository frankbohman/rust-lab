pub mod grpc;
pub mod http;
pub mod middleware;
pub mod setup;

use anyhow::Result;

use opentelemetry::KeyValue;
use opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge;
use opentelemetry_sdk::Resource;
use opentelemetry_semantic_conventions::resource as semconv;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter};

pub fn bootstrap(service_name: &str, service_version: &str, deployment_environment: &str) -> Result<()> {
  let resource = Resource::new(vec![
    KeyValue::new(semconv::SERVICE_NAME.to_string(), service_name.to_string()),
    KeyValue::new(semconv::SERVICE_VERSION.to_string(), service_version.to_string()),
    KeyValue::new(
      semconv::DEPLOYMENT_ENVIRONMENT.to_string(),
      deployment_environment.to_string(),
    ),
  ]);

  let tracer = self::setup::init_tracer(&resource)?;
  let meter_provider = self::setup::init_meter(&resource)?;
  let _ = self::setup::init_logger(&resource)?;

  let logger_provider = opentelemetry::global::logger_provider();

  let logging_layer = OpenTelemetryTracingBridge::new(&logger_provider);
  let tracing_layer = tracing_opentelemetry::layer().with_tracer(tracer);
  let metrics_layer = tracing_opentelemetry::MetricsLayer::new(meter_provider);

  let fmt_layer = tracing_subscriber::fmt::layer()
    .with_line_number(true)
    .with_file(true)
    .event_format(crate::toolbox::traceidformat::TraceIdFormat);
  let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

  let subscriber = tracing_subscriber::registry()
    .with(env_filter)
    .with(metrics_layer)
    .with(tracing_layer)
    .with(logging_layer)
    .with(fmt_layer);

  tracing::subscriber::set_global_default(subscriber)?;

  tracing::info!("This event will be logged in the root span.");
  info!("This event will be logged as a log.");

  // #[cfg(feature = "profiling")]
  // crate::toolbox::profiling::start(service_name)?;

  Ok(())
}
