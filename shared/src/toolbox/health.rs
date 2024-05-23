use std::{pin::Pin, vec};

use crate::proto::health::v1::{
  health_check_response::ServingStatus, health_server::Health, HealthCheckRequest, HealthCheckResponse,
};

use tokio_stream::Stream;
use tonic::{Response, Status};

pub use crate::proto::health::v1::health_server::HealthServer;

#[derive(Clone)]
pub struct ServiceStatus {
  service_name: String,
  status:       ServingStatus,
}

impl ServiceStatus {
  pub fn new(service_name: &str, status: ServingStatus) -> Self {
    Self {
      service_name: service_name.to_string(),
      status,
    }
  }
}

#[derive(Clone)]
pub struct HealthService {
  // todo add mpsc channel to push statuses from elsewhere
  services: Vec<ServiceStatus>,
}
type HealthCheckResult<T> = Result<Response<T>, Status>;
type ResponseStream = Pin<Box<dyn Stream<Item = Result<HealthCheckResponse, Status>> + Send + 'static>>;

impl HealthService {
  pub fn new(service_name: &str) -> Self {
    let services = vec![ServiceStatus::new(service_name, ServingStatus::Unknown)];
    Self {
      services,
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
    let service = &request.into_inner().service;

    if let Some(svc) = self.services.iter().find(|x| &x.service_name == service) {
      Ok(tonic::Response::new(HealthCheckResponse {
        status: svc.status as i32,
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
