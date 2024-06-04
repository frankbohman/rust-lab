use std::{
  pin::Pin,
  task::{Context, Poll},
  time::Duration,
};
use tonic::{body::BoxBody, transport::Server, Request, Response, Status};
use tower::{Layer, Service};

#[derive(Debug, Clone, Default)]
struct TelemetryLayer;

#[derive(Debug, Clone)]
struct TelemetryMiddleware<S> {
  inner: S,
}

impl<S> Layer<S> for TelemetryLayer {
  type Service = TelemetryMiddleware<S>;

  fn layer(&self, service: S) -> Self::Service {
    TelemetryMiddleware {
      inner: service
    }
  }
}
