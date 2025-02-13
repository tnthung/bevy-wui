use std::sync::{Arc, Mutex, MutexGuard};


#[derive(Debug, Clone, Default)]
pub(crate) struct Queue<E>(Arc<Mutex<Vec<E>>>);


impl<E> Queue<E> {
  pub fn lock(&self) -> MutexGuard<Vec<E>> {
    self.0.lock().unwrap()
  }
}
