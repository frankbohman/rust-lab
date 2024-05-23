pub mod proto;
pub mod toolbox;
pub use axum;
pub use tokio;
pub use tonic;
pub use tracing;

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    let result = 2 + 2;
    assert_eq!(result, 4);
  }
}
