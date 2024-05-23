use std::pin::Pin;

use crate::proto::health::v1::{
  health_check_response::ServingStatus, health_server::Health, HealthCheckRequest, HealthCheckResponse,
};
use tokio_stream::Stream;
use tonic::{Response, Status};

pub use crate::proto::health::v1::health_server::HealthServer;
pub struct HealthService {
  // todo add mpsc channel to push statuses from elsewhere
  services: Vec<(String, Status)>,
}
type HealthCheckResult<T> = Result<Response<T>, Status>;
type ResponseStream = Pin<Box<dyn Stream<Item = Result<HealthCheckResponse, Status>> + Send + 'static>>;

impl Default for HealthService {
  fn default() -> Self {
    Self {
      // TODO: add a memorychannel to push status from somewhere else
      services: vec![],
    }
  }
}

impl HealthService {
  pub fn new() -> Self { Self::default() }

  pub fn register_service(&mut self, service_name: &str) {
    self
      .services
      .push((service_name.to_string(), Status::ok("registered")));
  }

  pub fn update_service(&mut self, service_name: &str, status: Status) {
    self.services.push((service_name.to_string(), status));
  }
}

#[tonic::async_trait]
impl Health for HealthService {
  type WatchStream = ResponseStream;

  async fn check(
    &self,
    request: tonic::Request<HealthCheckRequest>,
  ) -> HealthCheckResult<HealthCheckResponse> {
    let service = &request.into_inner().service;

    if let Some(svc) = self.services.iter().find(|x| x.0.as_str() == service) {
      Ok(tonic::Response::new(HealthCheckResponse {
        status: svc.1.code() as i32,
      }))
      // Ok(tonic::Response::new(HealthCheckResponse {
      //   status: ServingStatus::Serving as i32,
      // }))
    } else {
      Ok(tonic::Response::new(HealthCheckResponse {
        status: ServingStatus::NotServing as i32,
      }))
    }
  }

  async fn watch(
    &self,
    _request: tonic::Request<HealthCheckRequest>,
  ) -> HealthCheckResult<Self::WatchStream> {
    todo!()
  }
}
