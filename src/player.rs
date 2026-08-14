use crate::direction::Direction;
use crate::position::Pos;

pub struct Player {
  pub dirs: Vec<Direction>,
  pub pos: Pos,
}

impl Player {
  pub fn new() -> Self {
    Self {
      dirs: Vec::new(),
      pos: Pos { x: 50.0, y: 50.0 },
    }
  }

  pub fn change_dirs(&mut self, new_dirs: Vec<Direction>) {
    self.dirs = new_dirs;
  }

  pub fn update_pos(&mut self) {
    let mut dx = 0.0f32;
    let mut dy = 0.0f32;
    let base_speed = 5.0f32;

    for dir in &self.dirs {
      match dir {
        Direction::Up => dy -= 1.0,
        Direction::Down => dy += 1.0,
        Direction::Left => dx -= 1.0,
        Direction::Right => dx += 1.0,
      }
    }

    if dx != 0.0 && dy != 0.0 {
      let normalization = 1.0f32 / (2.0f32).sqrt();
      dx *= normalization;
      dy *= normalization;
    }

    self.pos.x += dx * base_speed;
    self.pos.y += dy * base_speed;
  }
}
