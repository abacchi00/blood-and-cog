use macroquad::prelude::*;

use crate::traits::{Renderable, Collidable, CollisionShape};
use crate::config::*;

pub struct Player {
  pub pos: Vec2,
}

impl Player {
  pub fn new(pos: Vec2) -> Self {
    Self {
      pos,
    }
  }

  pub fn calculate_movement_delta(&self, input_dir: Vec2, dt: f32) -> Vec2 {
    let mut dx = input_dir.x;
    let mut dy = input_dir.y;

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
