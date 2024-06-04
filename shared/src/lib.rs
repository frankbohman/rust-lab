pub mod proto;
pub mod toolbox;

pub use axum;
pub use proto::crud;
pub use tokio;
pub use tonic;
pub use tonic_health;
pub use tonic_reflection;
pub use tonic_web;
pub use tracing;

pub use tower;
pub use tower_http;

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    let result = 2 + 2;
    assert_eq!(result, 4);
  }
}
