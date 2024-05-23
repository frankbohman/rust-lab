pub mod echo {
  include!(concat!(env!("OUT_DIR"), "/tonic", "/echo.rs"));
}

pub mod health {
  pub mod v1 {
    include!(concat!(env!("OUT_DIR"), "/tonic", "/grpc.health.v1.rs"));
  }
}
