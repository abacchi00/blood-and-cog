use macroquad::prelude::*;

use crate::direction::Direction;
use crate::traits::{Renderable, Collidable, CollisionShape};
use crate::config::*;

pub struct Player {
  pub dirs: Vec<Direction>,
  pub pos: Vec2,
}
impl Player {
  pub fn new(pos: Vec2) -> Self {
    Self {
      dirs: Vec::new(),
      pos,
    }
  }

  pub fn update_input(&mut self) {
    self.dirs.clear();
    if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) { self.dirs.push(Direction::Up); }
    if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) { self.dirs.push(Direction::Down); }
    if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) { self.dirs.push(Direction::Left); }
    if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) { self.dirs.push(Direction::Right); }
  }

  pub fn calculate_movement_delta(&self, dt: f32) -> Vec2 {
    let mut dx = 0.0f32;
    let mut dy = 0.0f32;

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

    vec2(dx * PLAYER_BASE_SPEED * dt, dy * PLAYER_BASE_SPEED * dt)
  }
}

impl Renderable for Player {
  fn render(&self) {
    draw_circle(self.pos.x, self.pos.y, PLAYER_RADIUS, PLAYER_BORDER_COLOR);
    draw_circle(self.pos.x, self.pos.y, PLAYER_RADIUS - PLAYER_BORDER_THICKNESS, PLAYER_COLOR);
  }
}

impl Collidable for Player {
  fn pos(&self) -> Vec2 { self.pos }
  fn shape(&self) -> CollisionShape {
    CollisionShape::Circle { radius: PLAYER_RADIUS }
  }
}
