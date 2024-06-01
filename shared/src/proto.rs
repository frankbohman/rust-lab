pub mod echo {
  include!(concat!(env!("OUT_DIR"), "/tonic", "/echo.rs"));
  #[allow(dead_code)]
  pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("tonic/echo_descriptor");
}

pub mod health {
  pub mod v1 {
    include!(concat!(env!("OUT_DIR"), "/tonic", "/grpc.health.v1.rs"));
    #[allow(dead_code)]
    pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("tonic/health_descriptor");
  }
}
