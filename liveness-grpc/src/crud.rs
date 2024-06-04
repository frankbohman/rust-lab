use std::sync::Arc;

use shared::{
  crud::{
    crud_server::Crud, CreateRequest, DeleteReply, DeleteRequest, Payload, ReadReply, ReadRequest,
    UpdateRequest,
  },
  tonic::{self, Response, Status},
};
use uuid::Uuid;

use crate::state::State;

#[derive(Clone)]
pub struct CrudService {
  state: State,
}

impl CrudService {
  pub fn new(state: State) -> Self {
    Self {
      state,
    }
  }
}

#[tonic::async_trait]
impl Crud for CrudService {
  async fn create(&self, request: tonic::Request<CreateRequest>) -> Result<Response<ReadReply>, Status> {
    let id = Uuid::new_v4().to_string();
    let payload = Payload {
      message: "Howdy...!".into(),
    };

    if let Some(payload) = request.get_ref().payload {
      let mut data = self.state.data.lock().unwrap();
      data.insert(id.clone(), payload.clone());
      Ok(Response::new(ReadReply {
        id,
        Some(payload),
      }))
    } else {
      Err(Status::invalid_argument("No payload provided"))
    }
  }

  async fn read(&self, request: tonic::Request<ReadRequest>) -> Result<Response<ReadReply>, Status> {
    if let Some(payload) = self.state.data.lock().unwrap().get(&request.get_ref().id) {
      Ok(Response::new(ReadReply {
        id:      request.get_ref().id.clone(),
        payload: Some(payload.clone()),
      }))
    } else {
      Err(Status::not_found("Nothing here!"))
    }
  }

  async fn update(&self, _request: tonic::Request<UpdateRequest>) -> Result<Response<ReadReply>, Status> {
    todo!()
  }

  async fn delete(&self, _request: tonic::Request<DeleteRequest>) -> Result<Response<DeleteReply>, Status> {
    todo!()
  }
}
