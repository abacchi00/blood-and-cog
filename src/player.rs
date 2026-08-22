use macroquad::prelude::*;

use crate::config::*;
use crate::traits::{Renderable, Collidable, CollisionShape};
use crate::alt_shapes::BorderedCircle;

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
    BorderedCircle {
      x: self.pos.x,
      y: self.pos.y,
      radius: PLAYER_RADIUS,
      color: PLAYER_COLOR,
      b_thick: PLAYER_BORDER_THICKNESS,
      b_color: PLAYER_BORDER_COLOR,
    }.draw();
  }
}

impl Collidable for Player {
  fn pos(&self) -> Vec2 { self.pos }
  fn shape(&self) -> CollisionShape {
    CollisionShape::Circle { radius: PLAYER_RADIUS }
  }
}
