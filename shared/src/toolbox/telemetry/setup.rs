use anyhow::Result;

use opentelemetry::global;
use opentelemetry_otlp::{Protocol, WithExportConfig};
use opentelemetry_sdk::{
  self, logs,
  metrics::{
    reader::{DefaultAggregationSelector, DefaultTemporalitySelector},
    MeterProvider,
  },
  propagation::TraceContextPropagator,
  trace::{self, Sampler},
  Resource,
};
use std::time::Duration;

#[allow(dead_code)]
pub fn init_meter(resource: &Resource) -> Result<MeterProvider> {
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
pub fn init_tracer(resource: &Resource) -> Result<opentelemetry_sdk::trace::Tracer> {
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

#[allow(dead_code)]
pub fn init_logger(resource: &Resource) -> Result<logs::Logger> {
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
