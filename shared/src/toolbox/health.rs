use std::pin::Pin;

use crate::proto::health::v1::{
  health_check_response::ServingStatus, health_server::Health, HealthCheckRequest, HealthCheckResponse,
};
use tokio_stream::Stream;
use tonic::{Response, Status};

pub use crate::proto::health::v1::health_server::HealthServer;
pub struct HealthService {
  services: Vec<String>,
}
type HealthCheckResult<T> = Result<Response<T>, Status>;
type ResponseStream = Pin<Box<dyn Stream<Item = Result<HealthCheckResponse, Status>> + Send + 'static>>;

impl Default for HealthService {
  fn default() -> Self {
    Self {
      services: vec!["".to_string()],
    }
  }
}

#[tonic::async_trait]
impl Health for HealthService {
  type WatchStream = ResponseStream;

  async fn check(
    &self,
    request: tonic::Request<HealthCheckRequest>,
  ) -> HealthCheckResult<HealthCheckResponse> {
    let service = request.into_inner().service;
    if self.services.contains(&service) {
      Ok(tonic::Response::new(HealthCheckResponse {
        status: ServingStatus::Serving as i32,
      }))
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
