use macroquad::prelude::*;

use crate::config::*;
use crate::traits::{
  Renderable,
  Expirable,
  Collidable, CollisionShape,
};
use crate::alt_shapes::BorderedCircle;

pub struct Enemy {
  pub pos: Vec2,
  pub radius: f32,
  life: f32,
}

impl Enemy {
  pub fn new(pos: Vec2) -> Self {
    Self {
      pos,
      life: ENEMY_INITIAL_LIFE,
      radius: ENEMY_RADIUS,
    }
  }

  pub fn take_hit(&mut self) {
    self.life -= 20.0;
    self.life = self.life.max(0.0);
  }

  pub fn is_alive(&self) -> bool {
    self.life > 0.0
  }

  pub fn calculate_movement_delta(&self, player_pos: Vec2, dt: f32) -> Vec2 {
    let direction = (player_pos - self.pos).normalize_or_zero();
    direction * ENEMY_BASE_SPEED * dt
  }
}

impl Renderable for Enemy {
  fn render(&self) {
    BorderedCircle {
      x: self.pos.x,
      y: self.pos.y,
      radius: self.radius,
      color: ENEMY_COLOR,
      b_thick: ENEMY_BORDER_THICKNESS,
      b_color: ENEMY_BORDER_COLOR,
    }.draw();

    draw_text(format!("{}", self.life), self.pos.x - self.radius, self.pos.y + self.radius * 2.0, 20.0, WHITE);
  }
}

impl Expirable for Enemy {
  fn should_clean(&self) -> bool {
    !self.is_alive()
  }
}

impl Collidable for Enemy {
  fn pos(&self) -> Vec2 { self.pos }
  fn shape(&self) -> CollisionShape {
    CollisionShape::Circle { radius: self.radius }
  }
}
