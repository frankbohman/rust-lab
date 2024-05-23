use anyhow::Result;

use opentelemetry::{global, KeyValue};
use opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge;
use opentelemetry_otlp::{Protocol, WithExportConfig};
use opentelemetry_sdk::{
  logs,
  metrics::{
    reader::{DefaultAggregationSelector, DefaultTemporalitySelector},
    MeterProvider,
  },
  propagation::TraceContextPropagator,
  trace::{self, Sampler},
  Resource,
};
use opentelemetry_semantic_conventions::resource as semconv;
use std::time::Duration;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter};

#[allow(dead_code)]
fn init_meter(resource: &Resource) -> Result<MeterProvider> {
  // let endpoint = format!("{}/v1/traces", endpoint);
  global::set_text_map_propagator(TraceContextPropagator::new());

  // let headers = otlp_headers()?;
  let meter = opentelemetry_otlp::new_pipeline()
    .metrics(opentelemetry_sdk::runtime::TokioCurrentThread)
    .with_temporality_selector(DefaultTemporalitySelector::default())
    .with_aggregation_selector(DefaultAggregationSelector::default())
    .with_period(Duration::from_secs(2))
    .with_timeout(Duration::from_secs(10))
    .with_exporter(
      opentelemetry_otlp::new_exporter()
        .http()
        // .with_headers(headers)
        .with_protocol(Protocol::HttpBinary)
        .with_timeout(Duration::from_secs(1)),
    )
    .with_resource(resource.to_owned())
    .build()?;

  global::set_meter_provider(meter.clone());

  Ok(meter)
}

#[allow(dead_code)]
fn init_tracer(resource: &Resource) -> Result<opentelemetry_sdk::trace::Tracer> {
  // let endpoint = format!("{}/v1/traces", endpoint);
  global::set_text_map_propagator(TraceContextPropagator::new());

  // let headers = otlp_headers()?;
  let sdk_tracer = opentelemetry_otlp::new_pipeline()
    .tracing()
    .with_exporter(
      opentelemetry_otlp::new_exporter()
        .http()
        // .with_headers(headers)
        .with_protocol(Protocol::HttpBinary)
        .with_timeout(Duration::from_secs(1)),
    )
    .with_trace_config(
      trace::config()
        .with_sampler(Sampler::AlwaysOn)
        .with_max_events_per_span(64)
        .with_max_attributes_per_span(16)
        .with_max_events_per_span(16)
        .with_resource(resource.to_owned()),
    )
    .install_batch(opentelemetry_sdk::runtime::TokioCurrentThread)?;

  global::set_tracer_provider(sdk_tracer.provider().expect("failed to initialize traceprovider"));

  Ok(sdk_tracer)
}

// fn otlp_headers() -> Result<HashMap<String, String>> {
//   let headers = std::env::var("OTEL_EXPORTER_OTLP_HEADERS")?;

//   let otlp_headers = headers
//     .split(',')
//     .map(|x| x.split_once('=').unwrap())
//     .map(|(k, v)| (k.to_owned(), v.to_owned()))
//     .collect::<Vec<(String, String)>>();

//   // let mut metadata = MetadataMap::with_capacity(headers.len());
//   let mut headers: HashMap<String, String> = HashMap::new();
//   for (k, v) in otlp_headers {
//     headers.insert(k, v);
//     // let k = tonic::metadata::AsciiMetadataKey::from_bytes(k.as_bytes())?;
//     // metadata.insert(k, v.parse()?);
//   }

//   Ok(headers)
// }

#[allow(dead_code)]
fn init_logger(resource: &Resource) -> Result<logs::Logger> {
  // let headers = otlp_headers()?;

  let logger = opentelemetry_otlp::new_pipeline()
    .logging()
    .with_log_config(logs::Config::default().with_resource(resource.to_owned()))
    .with_exporter(
      opentelemetry_otlp::new_exporter().http(), // .with_headers(headers)
    )
    .install_batch(opentelemetry_sdk::runtime::Tokio)?;
  global::set_logger_provider(logger.provider().expect("failed to initialize loggerprovider"));
  Ok(logger)
}

pub fn bootstrap(service_name: &str, service_version: &str, deployment_environment: &str) -> Result<()> {
  let resource = Resource::new(vec![
    KeyValue::new(semconv::SERVICE_NAME.to_string(), service_name.to_string()),
    KeyValue::new(semconv::SERVICE_VERSION.to_string(), service_version.to_string()),
    KeyValue::new(
      semconv::DEPLOYMENT_ENVIRONMENT.to_string(),
      deployment_environment.to_string(),
    ),
  ]);
  let tracer = init_tracer(&resource)?;
  let meter_provider = init_meter(&resource)?;
  let _ = init_logger(&resource)?;

  let logger_provider = opentelemetry::global::logger_provider();

  let logging_layer = OpenTelemetryTracingBridge::new(&logger_provider);
  let tracing_layer = tracing_opentelemetry::layer().with_tracer(tracer);
  let metrics_layer = tracing_opentelemetry::MetricsLayer::new(meter_provider);

  let fmt_layer = tracing_subscriber::fmt::layer().event_format(crate::toolbox::traceidformat::TraceIdFormat);
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
