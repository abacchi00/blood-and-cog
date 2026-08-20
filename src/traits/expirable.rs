pub trait Expirable {
  // Returns true if the object should be DELETED from game
  fn should_clean(&self) -> bool; 
}

pub trait ExpirableVec {
  fn clean_expired(&mut self);
}

impl<T: Expirable> ExpirableVec for Vec<T> {
  fn clean_expired(&mut self) {
    self.retain(|item| !item.should_clean());
  }
}
