use std::{
  collections::HashMap,
  sync::{Arc, Mutex},
};

use shared::crud::Payload;

// pub struct Entry {
//   id:      String,
//   message: String,
// }

#[derive(Clone)]
pub struct State {
  pub data: Arc<Mutex<HashMap<String, Payload>>>,
}

impl Default for State {
  fn default() -> Self {
    Self {
      data: Arc::new(Mutex::new(HashMap::new())),
    }
  }
}
