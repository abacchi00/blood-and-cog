pub trait Expirable {
  // Returns true if the object should be DELETED from game
  fn should_clean(&self) -> bool; 
}
