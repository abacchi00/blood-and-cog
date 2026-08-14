use macroquad::prelude::{KeyCode, is_key_down};

#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
  Up,
  Down,
  Left,
  Right,
}

impl Direction {
  pub fn current_directions() -> Vec<Self> {
    let mut dirs = Vec::new();
    
    if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
      dirs.push(Direction::Up);
    }
    if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
      dirs.push(Direction::Down);
    }
    if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
      dirs.push(Direction::Left);
    }
    if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
      dirs.push(Direction::Right);
    }
    
    dirs
  }
}
