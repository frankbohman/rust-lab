pub mod crud {
  include!(concat!(env!("OUT_DIR"), "/tonic", "/crud.rs"));
  #[allow(dead_code)]
  pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("tonic/crud_descriptor");
}

pub mod health {
  pub mod v1 {
    include!(concat!(env!("OUT_DIR"), "/tonic", "/grpc.health.v1.rs"));
    #[allow(dead_code)]
    pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("tonic/health_descriptor");
  }
}
